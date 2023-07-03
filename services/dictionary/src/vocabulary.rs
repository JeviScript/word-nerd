use std::time::Duration;

use reqwest::{header::HeaderValue, Response};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::cloudflare_bypasser;

static DEFINITION_BASE_URL: &str = "https://www.vocabulary.com/dictionary";
static EXAMPLES_BASE_URL: &str = "https://corpus.vocabulary.com/api/1.0/examples.json";

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Word {
    pub header: String,
    pub other_forms: Vec<String>,
    pub short_description: String,
    pub long_description: String,
    pub definitions: Vec<Definition>,
    pub examples: Vec<Example>,
    pub html: String,
    // TODO synonyms
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
    println!("{}", url);
    url
}

#[derive(Debug)]
pub enum ScrapeErr {
    BypassErr(cloudflare_bypasser::BypassErr),
    GetRequestErr(reqwest::Error),
    CssSelectorErr(scraper::error::SelectorErrorKind<'static>),
}

pub async fn scrape(word: &str) -> Result<Word, ScrapeErr> {
    let bypasser = cloudflare_bypasser::Bypasser::new();
    let html = bypasser
        .get(get_word_url(word).as_str())
        .await
        .map_err(ScrapeErr::BypassErr)?;

    let examples = scrape_examples(word).await?;

    let html_doc = Html::parse_document(html.as_str());

    let selector = Selector::parse("[id=hdr-word-area]").map_err(ScrapeErr::CssSelectorErr)?;
    let header = html_doc
        .select(&selector)
        .next()
        .map_or_else(|| "", |el| el.text().next().unwrap_or_default())
        .trim()
        .to_string();

    let selector =
        Selector::parse(".word-forms > b:nth-child(1)").map_err(ScrapeErr::CssSelectorErr)?;

    let other_forms: Vec<String> = html_doc
        .select(&selector)
        .next()
        .map_or_else(|| "", |el| el.text().next().unwrap_or_default())
        .trim()
        .split(';')
        .map(|x| x.trim().to_string())
        .collect();

    let selector = Selector::parse(".short").map_err(ScrapeErr::CssSelectorErr)?;

    let short_description = html_doc
        .select(&selector)
        .next()
        .map_or_else(
            || "".to_string(),
            |el| el.text().collect::<Vec<_>>().join("").to_string(),
        )
        .trim()
        .to_string();

    let selector = Selector::parse(".long").map_err(ScrapeErr::CssSelectorErr)?;

    let long_description = html_doc
        .select(&selector)
        .next()
        .map_or_else(
            || "".to_string(),
            |el| el.text().collect::<Vec<_>>().join("").to_string(),
        )
        .trim()
        .to_string();

    let definitions = scrape_definitions(html_doc)?;

    let word = Word {
        header,
        html: html.trim().to_string(),
        other_forms,
        short_description,
        long_description,
        definitions,
        examples,
    };

    Ok(word)
}

fn scrape_definitions(html: Html) -> Result<Vec<Definition>, ScrapeErr> {
    let selector =
        Selector::parse(".word-definitions > ol > li").map_err(ScrapeErr::CssSelectorErr)?;

    let definitions = html
        .select(&selector)
        .map(|el| {
            let selector = Selector::parse(".definition").map_err(ScrapeErr::CssSelectorErr)?;

            let definitionInners = el.select(&selector).next().map_or_else(
                || Vec::new(),
                |el| el.text().map(|x| x.trim().to_string()).collect::<Vec<_>>(),
            );

            println!("{:?}", definitionInners);

            let variant = definitionInners.get(1).unwrap_or(&"".to_string()).clone();

            let variant = match variant.as_str().trim() {
                "verb" => WordVariant::Verb,
                "noun" => WordVariant::Noun,
                "adverb" => WordVariant::Adverb,
                "adjective" => WordVariant::Adjective,
                v => WordVariant::Other(v.to_string()),
            };

            let description = definitionInners.get(2).unwrap_or(&"".to_string()).clone();
            let selector = Selector::parse(".example").map_err(ScrapeErr::CssSelectorErr)?;
            let select = el.select(&selector);

            let short_examples: Vec<String> = select
                .map(|el| {
                    el.text()
                        .filter_map(|x| {
                            let trimmed = x.trim();
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

            Ok(Definition {
                variant,
                description,
                image: None,
                short_examples,
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

    println!("{:?}", res);

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
