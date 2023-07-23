use crate::models::vocabulary::WordVariant;
use crate::{
    cloudflare_bypasser,
    models::{
        shared::{Audio, Pronunciation, PronunciationVariant},
        vocabulary::{Example, SubDefinition},
    },
    utils::Css,
};
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

static DEFINITION_BASE_URL: &str = "https://www.vocabulary.com/dictionary";
static EXAMPLES_BASE_URL: &str = "https://corpus.vocabulary.com/api/1.0/examples.json";

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Definition {
    pub id_ref: String,
    pub header: String,
    pub pronunciations: Vec<Pronunciation>,
    pub other_forms: Vec<String>,
    pub short_description: String,
    pub long_description: String,
    pub definitions: Vec<SubDefinition>,
    pub examples: Vec<Example>,
}

pub fn get_word_url(word: &str) -> String {
    format!("{DEFINITION_BASE_URL}/{word}")
}

pub fn get_word_examples_url(word: &str, max_results: u8) -> String {
    let url =
        format!("{EXAMPLES_BASE_URL}?maxResults={max_results}&query={word}&sartOffset=0&domain=F");
    url
}

#[derive(Debug)]
pub enum ScrapeErr {
    BypassErr(cloudflare_bypasser::BypassErr),
    GetRequestErr(reqwest::Error),
}

enum ElementSelector {
    Header,
    IpaSection,
    OtherForms,
    ShortDescription,
    LongDescription,
    Definitions,
    Definition,
    Example,
    Synonym,
}

impl From<ElementSelector> for Selector {
    fn from(value: ElementSelector) -> Self {
        let css_selector = match value {
            ElementSelector::Header => "[id=hdr-word-area]",
            ElementSelector::IpaSection => ".ipa-with-audio",
            ElementSelector::OtherForms => ".word-forms > b:nth-child(1)",
            ElementSelector::ShortDescription => ".short",
            ElementSelector::LongDescription => ".long",
            ElementSelector::Definitions => ".word-definitions > ol > li",
            ElementSelector::Definition => ".definition",
            ElementSelector::Example => ".example",
            ElementSelector::Synonym => ".defContent > .instances .word",
        };

        Selector::parse(css_selector)
            .unwrap_or_else(|_| panic!("Could not parse the css_selector: {css_selector}"))
    }
}

pub async fn scrape(word: &str) -> Result<Definition, ScrapeErr> {
    let bypasser = cloudflare_bypasser::Bypasser::new();
    // TODO handle redirects and not found page
    let html = bypasser
        .get(get_word_url(word).as_str())
        .await
        .map_err(ScrapeErr::BypassErr)?;

    let examples = scrape_examples(word).await?;

    let html_doc = Html::parse_document(html.as_str());

    let header = html_doc
        .select(&ElementSelector::Header.into())
        .next()
        .map_or_else(|| "", |el| el.text().next().unwrap_or_default())
        .trim()
        .to_string();

    let other_forms: Vec<String> = html_doc
        .select(&ElementSelector::OtherForms.into())
        .next()
        .map_or_else(|| "", |el| el.text().next().unwrap_or_default())
        .trim()
        .split(';')
        .map(|x| x.trim().to_string())
        .collect();

    let short_description = html_doc
        .select(&ElementSelector::ShortDescription.into())
        .next()
        .map_or_else(
            || "".to_string(),
            |el| el.text().collect::<Vec<_>>().join(""),
        )
        .trim()
        .to_string();

    let long_description = html_doc
        .select(&ElementSelector::LongDescription.into())
        .next()
        .map_or_else(
            || "".to_string(),
            |el| el.text().collect::<Vec<_>>().join(""),
        )
        .trim()
        .to_string();

    let definitions = scrape_definitions(html_doc.clone())?;
    let pronunciations = scrape_pronunciations(html_doc).await;

    let word = Definition {
        id_ref: word.to_string(),
        header,
        pronunciations,
        other_forms,
        short_description,
        long_description,
        definitions,
        examples,
    };

    Ok(word)
}

async fn scrape_pronunciations(html: Html) -> Vec<Pronunciation> {
    let parsed: Vec<_> = {
        html.select(&ElementSelector::IpaSection.into())
            .map(|el| {
                let variant = match el.select(&Css("div").into()).next() {
                    Some(el) => {
                        let sens = scraper::CaseSensitivity::AsciiCaseInsensitive;
                        let el = el.value();
                        if el.has_class("us-flag-icon", sens) {
                            PronunciationVariant::Usa
                        } else if el.has_class("uk-flag-icon", sens) {
                            PronunciationVariant::Uk
                        } else {
                            PronunciationVariant::Other
                        }
                    }
                    None => PronunciationVariant::Other,
                };

                let ipa_str = el
                    .select(&Css("h3").into())
                    .next()
                    .map_or_else(|| "", |val| val.text().next().unwrap_or_default())
                    .to_string();

                let audio_src_url = get_audio_src_url(el);

                (variant, ipa_str, audio_src_url)
            })
            .collect()
    };

    let async_result = parsed
        .into_iter()
        .map(|(variant, ipa_str, audio_src)| async {
            let audio: Option<Audio> = match audio_src {
                Some(url) => get_audio_from_url(url).await,
                None => None,
            };

            Pronunciation {
                variant,
                ipa_str,
                audio,
            }
        });

    futures::future::join_all(async_result).await
}

fn get_audio_src_url(el: ElementRef) -> Option<String> {
    let url: Option<String> = el.select(&Css(".audio").into()).next().map_or_else(
        || None,
        |val| {
            let found = val.value().attrs().find(|(k, _v)| *k == "data-audio");

            let url: Option<String> = if let Some(val) = found {
                let us_url = format!("https://audio.vocab.com/1.0/us/{}.mp3", val.1);
                Some(us_url)
            } else {
                let audio_el = val.select(&Css("audio").into()).next();
                if let Some(el) = audio_el {
                    let src = el
                        .value()
                        .attrs()
                        .find(|(k, _v)| *k == "src")
                        .map(|(_k, v)| v.to_string());
                    src
                } else {
                    None
                }
            };
            url
        },
    );
    url
}

async fn get_audio_from_url(url: String) -> Option<Audio> {
    let response = reqwest::get(url).await;

    let audio = match response {
        Ok(res) => {
            let content_type = match res.headers().get(reqwest::header::CONTENT_TYPE) {
                Some(val) => match val.to_str() {
                    Ok(val) => Some(val.to_string()),
                    Err(_) => None,
                },
                None => None,
            };

            let bytes = res.bytes().await;

            match (content_type, bytes) {
                (Some(val), Ok(bytes)) => Some(Audio {
                    content_type: val,
                    bytes: bytes.into_iter().collect::<Vec<u8>>(),
                }),
                _ => None,
            }
        }
        Err(_err) => None,
    };

    audio
}

fn scrape_definitions(html: Html) -> Result<Vec<SubDefinition>, ScrapeErr> {
    let definitions = html
        .select(&ElementSelector::Definitions.into())
        .map(|el| {
            let definition = el
                .select(&ElementSelector::Definition.into())
                .next()
                .map_or_else(Vec::new, |el| {
                    el.text().map(|x| x.trim().to_string()).collect::<Vec<_>>()
                });

            let variant = definition.get(1).unwrap_or(&"".to_string()).clone();

            let variant = match variant.as_str().trim() {
                "verb" => WordVariant::Verb,
                "noun" => WordVariant::Noun,
                "adverb" => WordVariant::Adverb,
                "adjective" => WordVariant::Adjective,
                v => WordVariant::Other(v.to_string()),
            };

            let description = definition.get(2).unwrap_or(&"".to_string()).clone();

            let short_examples: Vec<String> = el
                .select(&ElementSelector::Example.into())
                .map(|el| {
                    el.text()
                        .filter_map(|x| {
                            // believe it or not but those two single “ quotes are not the same
                            let without_quoutes = x.replace(['“', '”'], "");
                            let trimmed = without_quoutes.trim().to_string();
                            match trimmed.is_empty() {
                                true => None,
                                false => Some(trimmed),
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                        .trim()
                        .to_string()
                })
                .collect();

            let synonyms: Vec<String> = el
                .select(&ElementSelector::Synonym.into())
                .map(|el| el.text().next().unwrap_or_default().to_string())
                .collect();

            Ok(SubDefinition {
                variant,
                description,
                image: None, // TODO image
                short_examples,
                synonyms,
            })
        })
        .filter_map(|x: Result<SubDefinition, ScrapeErr>| x.ok())
        .collect();

    Ok(definitions)
}

#[derive(Deserialize, Debug)]
struct GetExampleRes {
    result: ResultRes,
}

#[derive(Deserialize, Debug)]
struct ResultRes {
    sentences: Vec<Sentence>,
}

#[derive(Deserialize, Debug)]
struct Sentence {
    sentence: String,
    volume: Volume,
}

#[derive(Deserialize, Debug)]
struct Volume {
    title: String,
    author: String,
}

async fn scrape_examples(word: &str) -> Result<Vec<Example>, ScrapeErr> {
    let url = get_word_examples_url(word, 24);
    let res = reqwest::get(url).await.map_err(ScrapeErr::GetRequestErr)?;

    let res = res
        .json::<GetExampleRes>()
        .await
        .map_err(ScrapeErr::GetRequestErr)?;

    let examples: Vec<Example> = res
        .result
        .sentences
        .iter()
        .map(|s| Example {
            sentence: s.sentence.clone(),
            author: s.volume.author.clone(),
            source_title: s.volume.title.clone(),
        })
        .collect();

    Ok(examples)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_is_sync() {
        fn is_send<T: Send>() {}
        is_send::<Html>(); // compiles only if true
    }

    #[tokio::test]
    async fn scrape_pronunciations_success() {
        let html = r#"
            <div class="ipa-section">
                <div class="ipa-with-audio">
                    <div class="us-flag-icon"></div>
                        <a class="audio"></a>
                        <span style="white-space:nowrap;"><h3>/sleɪt/</h3></span>
                    </div>
                <div class="ipa-with-audio">
                    <div class="uk-flag-icon"></div>
                    <a class="audio"><audio class="pron-audio"></audio></a>
                    <span style="white-space:nowrap;"><h3>/sleɪt/</h3></span>
                </div>
                <a class="ipa-guide" href="/resources/ipa-pronunciation/">IPA guide</a>
            </div>
        "#;

        let html = Html::parse_document(html);

        // TODO mock http request
        let result = scrape_pronunciations(html).await;

        let first = &result[0];
        let second = &result[1];

        assert_eq!(first.variant, PronunciationVariant::Usa);
        assert_eq!(first.ipa_str, "/sleɪt/");
        assert_eq!(first.audio, None);

        assert_eq!(second.variant, PronunciationVariant::Uk);
        assert_eq!(second.ipa_str, "/sleɪt/");
        assert_eq!(second.audio, None);
    }
}
