use pulldown_cmark::{Event, Options, Parser, Tag};
use url::Url;

use crate::models::Link;

#[derive(Debug)]
pub enum ConvError {
    ParseError(String),
    InvalidUrl(String),
}

pub fn parse_markdown_links(src: &str) -> Result<Vec<Link>, ConvError> {
    let mut links: Vec<Link> = Vec::new();

    let opts = Options::empty();
    let parser = Parser::new_ext(src, opts);

    let mut in_link = false;
    let mut current_title = String::new();
    let mut current_url = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Link(_link_type, dest, _title)) => {
                in_link = true;
                current_url = dest.to_string();
            }
            Event::End(Tag::Link(_, _, _)) => {
                if current_title.trim().is_empty() {
                    current_title = current_url.clone();
                }
                if Url::parse(&current_url).is_err() {
                    return Err(ConvError::InvalidUrl(current_url.clone()));
                }
                links.push(Link::new(current_title.clone(), current_url.clone()));
                current_title.clear();
                current_url.clear();
                in_link = false;
            }
            Event::Text(text) => {
                if in_link {
                    current_title.push_str(&text);
                }
            }
            _ => {}
        }
    }

    Ok(links)
}

pub fn generate_netscape_html(links: &[Link]) -> String {
    let mut out = String::new();
    out.push_str("<!DOCTYPE NETSCAPE-Bookmark-file-1>\n");
    out.push_str("<!-- This is an automatically generated file. -->\n");
    out.push_str("<META HTTP-EQUIV=\"Content-Type\" CONTENT=\"text/html; charset=UTF-8\">\n");
    out.push_str("<TITLE>Bookmarks</TITLE>\n");
    out.push_str("<H1>Bookmarks</H1>\n");
    out.push_str("<DL><p>\n");

    for link in links {
        out.push_str(&format!(
            "    <DT><A HREF=\"{}\">{}</A>\n",
            html_escape(&link.url),
            html_escape(&link.title)
        ));
    }

    out.push_str("</DL><p>\n");
    out
}

pub fn convert_markdown_to_netscape(src: &str) -> Result<String, ConvError> {
    let links = parse_markdown_links(src)?;
    Ok(generate_netscape_html(&links))
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_simple() {
        let md = "- [Rust](https://www.rust-lang.org)\\n- [Search](https://example.com/search)";
        let res = convert_markdown_to_netscape(md).unwrap();
        assert!(res.contains("https://www.rust-lang.org"));
        assert!(res.contains("Rust"));
        assert!(res.contains("https://example.com/search"));
    }

    #[test]
    fn test_invalid_url() {
        let md = "- [Bad](ht!tp://no)";
        let res = convert_markdown_to_netscape(md);
        assert!(res.is_err());
    }
}
