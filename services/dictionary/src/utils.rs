use scraper::{element_ref::Select, ElementRef, Selector};

pub struct Css(pub &'static str);

impl From<Css> for Selector {
    fn from(css_selector: Css) -> Self {
        Selector::parse(css_selector.0)
            .unwrap_or_else(|_| panic!("Could not parse the css_selector: {}", css_selector.0))
    }
}

impl Css {
    pub fn first_text(self, el: ElementRef) -> String {
        el.select(&self.into())
            .next()
            .map_or_else(|| "", |el| el.text().next().unwrap_or_default())
            .to_string()
    }
}

pub trait CssUtils {
    fn first_text(&mut self) -> String;
    fn join_text(&mut self) -> String;
}

impl CssUtils for Select<'_, '_> {
    fn first_text(&mut self) -> String {
        self.next()
            .map_or_else(|| "", |el| el.text().next().unwrap_or_default())
            .to_string()
    }

    fn join_text(&mut self) -> String {
        self.next().map_or_else(
            || "".to_string(),
            |el| {
                el.text()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            },
        )
    }
}

pub trait AttrUtils {
    fn href_last_part(&self) -> String;
}

impl AttrUtils for ElementRef<'_> {
    fn href_last_part(&self) -> String {
        self.value()
            .attr("href")
            .unwrap_or_default()
            .split('/')
            .last()
            .unwrap_or_default()
            .to_string()
    }
}

impl CssUtils for ElementRef<'_> {
    fn first_text(&mut self) -> String {
        self.text().next().unwrap_or_default().to_string()
    }

    fn join_text(&mut self) -> String {
        self.text()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}
