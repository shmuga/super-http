use scraper::{Html, Selector};

lazy_static! {
    static ref TAG_SELECTOR: Selector = Selector::parse("span.tag").expect("error");
    static ref BODY_SELECTOR: Selector = Selector::parse("main").expect("error");
    static ref TITLE_SELECTOR: Selector = Selector::parse("title").expect("error");
    static ref A_SELECTOR: Selector = Selector::parse("a").expect("error");
    static ref HEADER_SELECTOR: Selector = Selector::parse("h1, h2, h3, h4, h5").expect("error");
}

pub struct ParsedFile {
    pub title: String,
    pub headers: Vec<String>,
    pub body: String,
    pub tags: Vec<String>,
    pub links: Vec<(String, String)>,
}
pub fn parse_content(content: String) -> Result<ParsedFile, std::io::Error> {
    let fragment = Html::parse_document(&content);

    let tags = fragment
        .select(&TAG_SELECTOR)
        .map(|el| el.text().collect::<String>())
        .collect::<Vec<_>>();

    let links = fragment
        .select(&A_SELECTOR)
        .map(|el| {
            (
                el.value().attr("href").unwrap_or("").to_string(),
                el.text().collect::<String>(),
            )
        })
        .collect::<Vec<_>>();

    let headers = fragment
        .select(&HEADER_SELECTOR)
        .map(|el| el.text().collect::<String>())
        .collect::<Vec<_>>();

    let body = fragment
        .select(&BODY_SELECTOR)
        .map(|el| el.text().collect::<String>())
        .collect::<Vec<_>>()
        .get(0)
        .unwrap_or(&String::from(""))
        .to_owned();

    let title = fragment
        .select(&TITLE_SELECTOR)
        .map(|el| el.text().collect::<String>())
        .collect::<Vec<_>>();

    Ok(ParsedFile {
        tags,
        body,
        headers,
        links,
        title: title.join(" "),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_html() {
        let html = "".to_string();
        let parsed = parse_content(html).unwrap();

        assert_eq!(parsed.title, "");
        assert_eq!(parsed.body, "");
        assert_eq!(parsed.tags, Vec::<String>::new());
    }

    #[test]
    fn test_has_content() {
        let html = r#"
            <html>
                <head>
                    <title>Title is nice</title>
                </head>
                <body>
                    <main>
                        This is the content with <span class="tag">Tag 1</span>
                        And some other <span class="tag">Tag 2</span>
                    </main>
                    <a href="https://google.com">External Link</a>
                    <a href="./some/path/index.html">Internal Link</a>
                </body>
            </html>
        "#
        .to_string();
        let parsed = parse_content(html).unwrap();

        assert_eq!(parsed.title, "Title is nice");
        assert_eq!(parsed.headers, Vec::<String>::new());
        assert_eq!(
            parsed.body.trim(),
            "This is the content with Tag 1
                        And some other Tag 2"
        );
        assert_eq!(parsed.tags, vec!["Tag 1".to_string(), "Tag 2".to_string()]);
        assert_eq!(
            parsed.links,
            vec![
                (
                    "https://google.com".to_string(),
                    "External Link".to_string()
                ),
                (
                    "./some/path/index.html".to_string(),
                    "Internal Link".to_string()
                )
            ]
        );
    }
}
