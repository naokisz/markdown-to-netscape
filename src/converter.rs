use pulldown_cmark::{Event, Options, Parser, Tag};
use url::Url;

#[derive(Debug)]
pub enum ConvError {
    ParseError(String),
    InvalidUrl(String),
}

pub fn convert_markdown_to_netscape(src: &str) -> Result<String, ConvError> {
    // Very small parser: look for markdown link items like - [title](url)
    let mut links: Vec<(String, String)> = Vec::new();

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
                    // use URL as title if empty
                    current_title = current_url.clone();
                }
                // validate URL
                if Url::parse(&current_url).is_err() {
                    return Err(ConvError::InvalidUrl(current_url.clone()));
                }
                links.push((current_title.clone(), current_url.clone()));
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

    // Build Netscape Bookmark HTML
    let mut out = String::new();
    out.push_str("<!DOCTYPE NETSCAPE-Bookmark-file-1>\n");
    out.push_str("<!-- This is an automatically generated file. -->\n");
    out.push_str("<META HTTP-EQUIV=\"Content-Type\" CONTENT=\"text/html; charset=UTF-8\">\n");
    out.push_str("<TITLE>Bookmarks</TITLE>\n");
    out.push_str("<H1>Bookmarks</H1>\n");
    out.push_str("<DL><p>\n");

    for (title, url) in links {
        out.push_str(&format!("    <DT><A HREF=\"{}\">{}</A>\n", html_escape(&url), html_escape(&title)));
    }

    out.push_str("</DL><p>\n");

    Ok(out)
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_simple() {
        let md = "- [Rust](https://www.rust-lang.org)\n- [Search](https://example.com/search)";
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
