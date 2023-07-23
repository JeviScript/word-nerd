use std::{fs, path::Path};

use scraper::Html;

pub enum TestHtml {
    Oxford(OxfordHtml)
}

pub enum OxfordHtml {
    Cat1,
    Refrain1,
    Fling1,
    Husky1,
    Take1,
}

impl TestHtml {
    fn to_path<'a>(&self) -> &'a Path {
        match self {
            TestHtml::Oxford(OxfordHtml::Cat1) => Path::new("./src/tests/htmls/oxford/cat_1.html"),
            TestHtml::Oxford(OxfordHtml::Refrain1) => Path::new("./src/tests/htmls/oxford/refrain_1.html"),
            TestHtml::Oxford(OxfordHtml::Fling1) => Path::new("./src/tests/htmls/oxford/fling_1.html"),
            TestHtml::Oxford(OxfordHtml::Husky1) => Path::new("./src/tests/htmls/oxford/husky_1.html"),
            TestHtml::Oxford(OxfordHtml::Take1) => Path::new("./src/tests/htmls/oxford/take_1.html"),
        }
    }
}

pub fn parse_html(test_html: TestHtml) -> Html {
    let content = fs::read_to_string(test_html.to_path()).expect("Should parse the html");
    Html::parse_document(content.as_str())
}
