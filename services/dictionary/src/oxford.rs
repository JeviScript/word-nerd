use crate::models::oxford::DefinitionGroup;
use crate::models::oxford::Idiom;
use crate::models::oxford::SimilarResult;
use crate::models::oxford::SubDefinition;
use crate::models::oxford::VebForm;
use crate::models::oxford::WordRef;
use crate::models::shared::Audio;
use crate::models::shared::Pronunciation;
use crate::models::shared::PronunciationVariant;
use crate::utils::AttrUtils;
use crate::utils::Css;
use crate::utils::CssUtils;
use reqwest::StatusCode;
use scraper::{Element, ElementRef, Html};

static DEFINITION_BASE_URL: &str = "https://www.oxfordlearnersdictionaries.com/definition/english";

pub async fn scrape(word: &str) -> Result<Definition, ScrapeErr> {
    let response = reqwest::Client::new()
        .get(get_word_url(word))
        .send()
        .await
        .map_err(ScrapeErr::GetRequestErr)?;

    let status_code = response.status();

    let html = response.text().await.map_err(ScrapeErr::GetRequestErr)?;

    match status_code {
        StatusCode::OK => {
            let scraped = scrape_html(&html);
            let pros = load_audio(&scraped.pronunciations).await;
            Ok(scraped.into_definition(pros))
        }
        StatusCode::NOT_FOUND => Err(ScrapeErr::NotFound(parse_not_found_page(&html))),
        other => Err(ScrapeErr::UnexpectedHtmlStatusCode(other)),
    }
}

// gotta do it separatly to any async operations since html is not Send
fn scrape_html(html: &str) -> Scraped {
    let html = Html::parse_document(html);
    Scraped {
        ref_id: get_id(&html),
        header: get_header(&html),
        inflections: get_inflections(&html),
        note: get_note(&html),
        grammar_hint: get_grammar_hint(&html),
        word_variant: get_word_variant(&html),
        similar_results: get_similar_results(&html),
        pronunciations: get_pronunciations(&html),
        definitions: get_definitions(&html),
        see_also: get_see_also(&html),
        word_origin: get_word_origin(&html),
        idioms: get_idioms(&html),
        phrasal_verbs: get_phrasal_verbs(&html),
        // TODO
        veb_forms: Vec::new(),
    }
}

fn get_id(html: &Html) -> String {
    html.select(&Css("div.entry").into())
        .next()
        .map_or_else(|| "", |el| el.value().attr("id").unwrap_or_default())
        .trim()
        .to_string()
}

fn get_header(html: &Html) -> String {
    html.select(&Css("h1.headword").into())
        .next()
        .map_or_else(|| "", |el| el.text().next().unwrap_or_default())
        .trim()
        .to_string()
}

fn get_word_variant(html: &Html) -> String {
    html.select(&Css("span.pos").into())
        .next()
        .map_or_else(|| "", |el| el.text().next().unwrap_or_default())
        .trim()
        .to_string()
}

fn get_note(html: &Html) -> String {
    html.select(&Css("div.entry .labels").into())
        .next()
        .map_or_else(|| "", |el| el.text().next().unwrap_or_default())
        .trim()
        .to_string()
}

fn get_grammar_hint(html: &Html) -> String {
    html.select(&Css("div.entry .grammar").into())
        .next()
        .map_or_else(|| "", |el| el.text().next().unwrap_or_default())
        .trim()
        .to_string()
}

fn get_see_also(html: &Html) -> Vec<WordRef> {
    html.select(&Css("div.entry .senses_multiple > .xrefs a").into())
        .map(|el| {
            let id_ref = el
                .value()
                .attr("href")
                .unwrap_or_default()
                .split('/')
                .last()
                .unwrap_or_default()
                .to_string();

            let word = el.text().next().unwrap_or_default().to_string();

            WordRef { id_ref, word }
        })
        .collect()
}

fn get_word_origin(html: &Html) -> String {
    let res = html
        .select(&Css("[unbox='wordorigin'] .body").into())
        .next()
        .map_or_else(
            || "".to_string(),
            |el| {
                el.text()
                    .map(|val| val.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            },
        );

    res
}

fn get_idioms(html: &Html) -> Vec<Idiom> {
    html.select(&Css(".idioms .idm-g").into())
        .map(|el| {
            let idiom = el.select(&Css(".idm").into()).first_text();
            let description = el.select(&Css(".def").into()).join_text();
            let notes = el
                .select(&Css(".labels").into())
                .map(|el| {
                    let mut note: String = el.text().next().unwrap_or_default().to_string();
                    if let Some(val) = el.next_sibling_element() {
                        if val.has_class(&"v".into(), scraper::CaseSensitivity::CaseSensitive) {
                            let val = val.text().next().unwrap_or_default();
                            note = format!("({} {})", note.as_str(), val);
                        }
                    };
                    note
                })
                .collect();

            let synonyms = el
                .select(&Css(".xrefs[xt='nsyn'] a").into())
                .map(|el| {
                    let id_ref = el
                        .value()
                        .attr("href")
                        .unwrap_or_default()
                        .split('/')
                        .last()
                        .unwrap_or_default()
                        .to_string();
                    let word = el.select(&Css(".xh").into()).first_text();
                    WordRef { id_ref, word }
                })
                .collect();

            let examples = el
                .select(&Css(".examples li .x").into())
                .map(|el| {
                    el.text()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                })
                .collect();

            Idiom {
                idiom,
                description,
                notes,
                synonyms,
                examples,
            }
        })
        .collect()
}

fn get_phrasal_verbs(html: &Html) -> Vec<WordRef> {
    html.select(&Css(".phrasal_verb_links a").into())
        .map(|el| {
            let id_ref = el
                .value()
                .attr("href")
                .unwrap_or_default()
                .split('/')
                .last()
                .unwrap_or_default()
                .to_string();

            let word = el
                .select(&Css(".xh").into())
                .next()
                .map_or_else(|| "", |el| el.text().next().unwrap_or_default())
                .to_string();

            WordRef { id_ref, word }
        })
        .collect()
}

fn get_pronunciations(html: &Html) -> Vec<ScrapedPronunciation> {
    html.select(&Css(".phonetics > div").into())
        .map(|el| {
            let variant = match el.value().attr("geo") {
                Some("br") => PronunciationVariant::Uk,
                Some("n_am") => PronunciationVariant::Usa,
                Some(_other) => PronunciationVariant::Other,
                None => PronunciationVariant::Other,
            };

            let ipa_str = el
                .select(&Css("span.phon").into())
                .next()
                .map_or_else(|| "", |el| el.text().next().unwrap_or_default())
                .trim()
                .to_string();

            let audio_src = el
                .select(&Css("div[data-src-mp3]").into())
                .next()
                .map_or_else(
                    || "",
                    |el| el.value().attr("data-src-mp3").unwrap_or_default(),
                )
                .to_string();

            ScrapedPronunciation {
                variant,
                ipa_str,
                audio_src,
            }
        })
        .collect()
}

fn get_inflections(html: &Html) -> String {
    html.select(&Css(".inflections").into()).next().map_or_else(
        || "".to_string(),
        |el| {
            el.text()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
        },
    )
}

fn get_definitions(html: &Html) -> Vec<DefinitionGroup> {
    let mut elements: Vec<ElementRef> = html
        .select(&Css(".senses_multiple .shcut-g").into())
        .collect();

    if elements.is_empty() {
        elements = html.select(&Css(".senses_multiple").into()).collect();
    }

    elements
        .iter()
        .map(|el| {
            let group_title = el
                .select(&Css(".shcut").into())
                .next()
                .and_then(|val| val.text().map(|x| x.to_string()).next());
            let definitions = el
                .select(&Css(".sense").into())
                .map(|el| {
                    let mut description = el.select(&Css(".def").into()).join_text();
                    let note = el.select(&Css(".labels").into()).first_text();
                    if !note.is_empty() {
                        description = format!("{} {}", note, description);
                    }
                    let note = el.select(&Css(".dis-g").into()).join_text();
                    if !note.is_empty() {
                        description = format!("{} {}", note, description);
                    }
                    let use_note = el.select(&Css(".use").into()).join_text();

                    let examples = el
                        .select(&Css(".sense > .examples li").into())
                        .map(|el| {
                            let mut example = el.select(&Css(".x").into()).join_text();
                            let note = el.select(&Css(".labels").into()).first_text();
                            if !note.is_empty() {
                                example = format!("{} {}", note, example);
                            }

                            example
                        })
                        .collect();

                    let see_also = el
                        .select(&Css(".xrefs[xt='see'] a").into())
                        .map(|el| {
                            let id_ref = el.href_last_part();
                            let word = el.select(&Css("span").into()).join_text();
                            WordRef { id_ref, word }
                        })
                        .collect();

                    let synonyms = el
                        .select(&Css(".xrefs[xt='syn'] a").into())
                        .map(|el| {
                            let id_ref = el.href_last_part();
                            let word = el.select(&Css("span").into()).join_text();
                            WordRef { id_ref, word }
                        })
                        .collect();

                    let extra_examples = el
                        .select(&Css("[unbox='extra_examples'] li").into())
                        .map(|mut el| el.join_text())
                        .collect();

                    SubDefinition {
                        description,
                        use_note,
                        examples,
                        see_also,
                        synonyms,
                        extra_examples,
                        extra_synonyms: Vec::new(), // TODO
                    }
                })
                .collect();
            DefinitionGroup {
                group_title,
                definitions,
            }
        })
        .collect()
}

fn get_similar_results(html: &Html) -> Vec<SimilarResult> {
    html.select(&Css("#relatedentries li a").into())
        .map(|el| {
            let id = el
                .value()
                .attr("href")
                .unwrap_or_default()
                .split('/')
                .last()
                .unwrap_or_default()
                .to_string();

            let word = el
                .select(&Css("span").into())
                .next()
                .map_or_else(|| "", |el| el.text().next().unwrap_or_default())
                .to_string();

            let word_variant = el
                .select(&Css("pos").into())
                .next()
                .map_or_else(|| "", |el| el.text().next().unwrap_or_default())
                .to_string();

            SimilarResult {
                id,
                word,
                word_variant: Some(word_variant),
            }
        })
        .collect()
}

fn parse_not_found_page(html: &str) -> NotFound {
    let _html = Html::parse_document(html);
    NotFound {
        similar_words: Vec::new(), // TODO
    }
}

async fn load_audio(scraped: &[ScrapedPronunciation]) -> Vec<Pronunciation> {
    let pronunciations = scraped.iter().map(|p| async move {
        let audio: Option<Audio> = get_audio(&p.audio_src).await;
        Pronunciation {
            variant: p.variant.clone(),
            ipa_str: p.ipa_str.clone(),
            audio,
        }
    });

    futures::future::join_all(pronunciations).await
}

async fn get_audio(audio_url: &str) -> Option<Audio> {
    let response = reqwest::get(audio_url).await;

    match response {
        Ok(response) => {
            let status_code = response.status();
            match status_code {
                StatusCode::OK => {
                    let content_type = match response.headers().get(reqwest::header::CONTENT_TYPE) {
                        Some(val) => match val.to_str() {
                            Ok(val) => Some(val.to_string()),
                            Err(_) => None,
                        },
                        None => None,
                    };

                    let bytes = response.bytes().await;

                    match (content_type, bytes) {
                        (Some(content_type), Ok(bytes)) => {
                            let audio = Audio {
                                content_type,
                                bytes: bytes.into_iter().collect(),
                            };
                            Some(audio)
                        }
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        Err(_) => None,
    }
}

fn get_word_url(word: &str) -> String {
    format!("{DEFINITION_BASE_URL}/{word}")
}

pub struct Definition {
    pub id_ref: String,
    pub header: String,
    pub inflections: String,
    pub note: String,
    pub grammar_hint: String,
    pub word_variant: String,
    pub similar_results: Vec<SimilarResult>,
    pub pronunciations: Vec<Pronunciation>,
    pub definitions: Vec<DefinitionGroup>,
    pub see_also: Vec<WordRef>,
    pub word_origin: String,
    pub idioms: Vec<Idiom>,
    pub phrasal_verbs: Vec<WordRef>,
    pub veb_forms: Vec<VebForm>,
}

pub struct Scraped {
    pub ref_id: String,
    pub header: String,
    pub inflections: String,
    pub note: String,
    pub grammar_hint: String,
    pub word_variant: String,
    pub similar_results: Vec<SimilarResult>,
    pub pronunciations: Vec<ScrapedPronunciation>,
    pub definitions: Vec<DefinitionGroup>,
    pub see_also: Vec<WordRef>,
    pub word_origin: String,
    pub idioms: Vec<Idiom>,
    pub phrasal_verbs: Vec<WordRef>,
    pub veb_forms: Vec<VebForm>,
}

impl Scraped {
    fn into_definition(self, pros: Vec<Pronunciation>) -> Definition {
        Definition {
            id_ref: self.ref_id,
            header: self.header,
            inflections: self.inflections,
            note: self.note,
            grammar_hint: self.grammar_hint,
            word_variant: self.word_variant,
            similar_results: self.similar_results,
            pronunciations: pros,
            definitions: self.definitions,
            see_also: self.see_also,
            word_origin: self.word_origin,
            idioms: self.idioms,
            phrasal_verbs: self.phrasal_verbs,
            veb_forms: self.veb_forms,
        }
    }
}

#[derive(Debug)]
pub struct NotFound {
    pub similar_words: Vec<WordRef>,
}

pub struct ScrapedPronunciation {
    pub variant: PronunciationVariant,
    pub ipa_str: String,
    pub audio_src: String,
}

#[derive(Debug)]
pub enum ScrapeErr {
    GetRequestErr(reqwest::Error),
    UnexpectedHtmlStatusCode(StatusCode),
    NotFound(NotFound),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::utils::{parse_html, OxfordHtml, TestHtml};

    #[test]
    fn get_id_ok() {
        let html = parse_html(TestHtml::Oxford(OxfordHtml::Cat1));
        assert_eq!("cat_1", get_id(&html).as_str());
    }

    #[test]
    fn get_header_ok() {
        let html = parse_html(TestHtml::Oxford(OxfordHtml::Cat1));
        assert_eq!("cat", get_header(&html).as_str());
    }

    #[test]
    fn get_note_ok() {
        let html = parse_html(TestHtml::Oxford(OxfordHtml::Refrain1));
        assert_eq!("(formal)", get_note(&html).as_str());
    }

    #[test]
    fn get_grammar_hint_ok() {
        let html = parse_html(TestHtml::Oxford(OxfordHtml::Refrain1));
        assert_eq!("[intransitive]", get_grammar_hint(&html).as_str());
    }

    #[test]
    fn get_word_variant_ok() {
        let html = parse_html(TestHtml::Oxford(OxfordHtml::Cat1));
        assert_eq!("noun", get_word_variant(&html).as_str());
    }

    #[test]
    fn get_see_also_ok() {
        let html = parse_html(TestHtml::Oxford(OxfordHtml::Cat1));
        let results = get_see_also(&html);
        assert_eq!("fat cat", results[0].word);
        assert_eq!("fat-cat", results[0].id_ref);
        assert_eq!("wildcat", results[1].word);
        assert_eq!("wildcat_3", results[1].id_ref);
    }

    #[test]
    fn get_word_origin_ok() {
        let html = parse_html(TestHtml::Oxford(OxfordHtml::Cat1));

        assert_eq!("Old English catt, catte, of Germanic origin; related to Dutch kat and German Katze; reinforced in Middle English by forms from late Latin cattus.", get_word_origin(&html));
    }

    #[test]
    fn get_idioms_ok() {
        let html = parse_html(TestHtml::Oxford(OxfordHtml::Cat1));

        let idioms = get_idioms(&html);

        assert_eq!(idioms.len(), 15);
        assert_eq!(&idioms[0].idiom, "be the cat’s whiskers/pyjamas");
        assert_eq!(
            &idioms[0].description,
            "to be the best thing, person, idea, etc."
        );
        assert_eq!(idioms[0].notes.len(), 1);
        assert_eq!(&idioms[0].notes[0], "(old-fashioned, informal)");
        assert_eq!(
            &idioms[0].examples[0],
            "He thinks he's the cat's whiskers (= he has a high opinion of himself)."
        );
        assert_eq!(idioms[0].synonyms.len(), 0);

        let mut idioms_iter = idioms.into_iter();

        let idiom = &idioms_iter
            .find(|i| &i.idiom == "like a cat that’s got the cream")
            .expect("should exist");

        assert_eq!(idiom.notes.len(), 2);
        assert_eq!(&idiom.notes[0], "(British English)");
        assert_eq!(
            &idiom.notes[1],
            "(US English like the cat that got/ate/swallowed the canary)"
        );
        assert_eq!(&idiom.description, "very pleased with yourself");
        assert_eq!(&idiom.synonyms[0].id_ref, "smug");
        assert_eq!(&idiom.synonyms[0].word, "smug");
        assert_eq!(
            &idiom.examples[0],
            "She looked like a cat that’s got the cream. She was almost purring with pleasure."
        );

        let idiom = &idioms_iter
            .find(|i| &i.idiom == "no room to swing a cat")
            .expect("should exist");

        assert_eq!(&idiom.description, "when somebody says there’s no room to swing a cat, they mean that a room is very small and that there is not enough space");
        assert_eq!(&idiom.notes[0], "(informal)");
        assert_eq!(idiom.notes.len(), 1);
    }

    #[test]
    fn get_phrasal_verbs_ok() {
        let html = parse_html(TestHtml::Oxford(OxfordHtml::Fling1));
        let verbs = get_phrasal_verbs(&html);

        assert_eq!(verbs.len(), 5);
        assert_eq!(&verbs[0].id_ref, "fling-off#flingoff2_e");
        assert_eq!(&verbs[0].word, "fling off");
    }

    #[test]
    fn get_inflections_ok() {
        let html = parse_html(TestHtml::Oxford(OxfordHtml::Husky1));
        let inflections = get_inflections(&html);
        assert_eq!(&inflections, "(comparative huskier, superlative huskiest)");
    }

    #[test]
    fn get_definitions_ok() {
        let html = parse_html(TestHtml::Oxford(OxfordHtml::Take1));
        let defs = get_definitions(&html);

        let total_defs = defs
            .iter()
            .fold(0, |acc, item| acc + item.definitions.len());

        let all_defs: Vec<&SubDefinition> = defs.iter().map(|x| &x.definitions).flatten().collect();

        assert_eq!(total_defs, 43);
        assert_eq!(defs[0].group_title, Some("carry/lead".to_string()));
        assert_eq!(defs[0].definitions.len(), 3);
        assert_eq!(
            &defs[0].definitions[0].description,
            "to carry or move something from one place to another"
        );
        assert_eq!(defs[0].definitions[0].examples.len(), 6);
        assert_eq!(
            &defs[0].definitions[0].examples[0],
            "Remember to take your coat when you leave."
        );
        assert_eq!(
            &defs[0].definitions[0].examples[4],
            "You need to take your laptop to the technician."
        );
        assert_eq!(defs[0].definitions[0].extra_examples.len(), 3);
        assert_eq!(
            &defs[0].definitions[0].extra_examples[0],
            "My things had already been taken to my room."
        );

        let def = &defs
            .iter()
            .find(|x| x.group_title == Some("choose/buy".to_string()))
            .expect("should exist")
            .definitions[1];

        assert_eq!(
            &def.description,
            "(formal) to buy a newspaper or magazine regularly"
        );

        assert_eq!(
            &all_defs[23].use_note,
            "(not usually used in the progressive tenses)"
        );
        assert_eq!(
            &all_defs[30].examples[1],
            "(informal) 80 take away 5 is 75."
        );
        assert_eq!(
            &all_defs[12].examples[1],
            "Did you take notes in the class?"
        );

        let html = parse_html(TestHtml::Oxford(OxfordHtml::Husky1));
        let defs = get_definitions(&html);
        assert_eq!(defs.len(), 1);
        assert_eq!(defs[0].definitions.len(), 2);
        assert_eq!(&defs[0].definitions[0].description, "(of a person or their voice) sounding deep, quiet and rough, sometimes in an attractive way");

        let html = parse_html(TestHtml::Oxford(OxfordHtml::Cat1));
        let defs = get_definitions(&html);
        assert_eq!(defs.len(), 1);
        assert_eq!(defs[0].definitions.len(), 2);
        assert_eq!(defs[0].definitions[1].examples.len(), 2);
        assert_eq!(
            &defs[0].definitions[1].examples[0],
            "the big cats (= lions, tigers, etc.)"
        );
        assert_eq!(defs[0].definitions[0].see_also.len(), 8);
        assert_eq!(
            &defs[0].definitions[0].see_also[0].id_ref,
            "the-cheshire-cat"
        );
        assert_eq!(&defs[0].definitions[0].see_also[0].word, "the Cheshire Cat");

        let html = parse_html(TestHtml::Oxford(OxfordHtml::Fling1));
        let defs = get_definitions(&html);
        assert_eq!(defs.len(), 1);
        assert_eq!(defs[0].definitions.len(), 3);
        assert_eq!(defs[0].definitions[0].synonyms.len(), 1);
        assert_eq!(&defs[0].definitions[0].synonyms[0].id_ref, "hurl");
        assert_eq!(&defs[0].definitions[0].synonyms[0].word, "hurl");
        assert_eq!(defs[0].definitions[0].extra_examples.len(), 2);
        assert_eq!(
            &defs[0].definitions[0].extra_examples[0],
            "She flung the letter down onto the table."
        );
    }
}
