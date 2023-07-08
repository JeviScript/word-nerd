use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use crate::cloudflare_bypasser;

static DEFINITION_BASE_URL: &str = "https://www.vocabulary.com/dictionary";
static EXAMPLES_BASE_URL: &str = "https://corpus.vocabulary.com/api/1.0/examples.json";

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Word {
    pub header: String,
    pub pronunciations: Vec<Pronunciation>,
    pub other_forms: Vec<String>,
    pub short_description: String,
    pub long_description: String,
    pub definitions: Vec<Definition>,
    pub examples: Vec<Example>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Pronunciation {
    pub variant: PronunciationVariant,
    pub audio: Audio,
    pub ipa_str: String, // https://www.vocabulary.com/resources/ipa-pronunciation/
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Audio {
    pub format: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub enum PronunciationVariant {
    #[default]
    UK,
    USA,
    Other,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub enum WordVariant {
    #[default]
    Noun,
    Verb,
    Adjective,
    Adverb,
    Other(String),
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Definition {
    pub variant: WordVariant,
    pub description: String,
    pub image: Option<Image>,
    pub short_examples: Vec<String>,
    pub synonyms: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Example {
    sentence: String,
    author: String,
    source_title: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Image {
    pub bytes: Vec<u8>,
    pub format: String,
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
    Synonym
}

impl From<ElementSelector> for Selector {
    fn from(value: ElementSelector) -> Self {
        let css_selector = match value {
            ElementSelector::Header => "[id=hdr-word-area]",
            ElementSelector::IpaSection => ".ipa-section",
            ElementSelector::OtherForms => ".word-forms > b:nth-child(1)",
            ElementSelector::ShortDescription => ".short",
            ElementSelector::LongDescription => ".long",
            ElementSelector::Definitions => ".word-definitions > ol > li",
            ElementSelector::Definition => ".definition",
            ElementSelector::Example => ".example",
            ElementSelector::Synonym => ".defContent > .instances .word"
        };

        Selector::parse(css_selector)
            .unwrap_or_else(|_| panic!("Could not parse the css_selector: {css_selector}"))
    }
}

pub async fn scrape(word: &str) -> Result<Word, ScrapeErr> {
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
    let pronunciations = scrape_pronunciations(html_doc).await?;

    let word = Word {
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

async fn scrape_pronunciations(html: Html) -> Result<Vec<Pronunciation>, ScrapeErr> {
    Ok(Vec::new())
    // html.select(&ElementSelector::IpaSection.into())
}

fn scrape_definitions(html: Html) -> Result<Vec<Definition>, ScrapeErr> {
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
                            let without_quoutes = x.replace('“', "").replace('”', "");
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

            let synonyms: Vec<String> = el.select(&ElementSelector::Synonym.into())
                .map(|el| {
                    el.text().next().unwrap_or_default().to_string()
                })
                .collect();

            Ok(Definition {
                variant,
                description,
                image: None,
                short_examples,
                synonyms
            })
        })
        .filter_map(|x: Result<Definition, ScrapeErr>| x.ok())
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
