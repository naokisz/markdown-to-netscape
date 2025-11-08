use pulldown_cmark::{Event, Options, Parser, Tag};
use url::Url;

use crate::models::Link;

#[derive(Debug)]
pub enum ConvError {
    InvalidUrl,
}

pub struct ConvertResult {
    pub html: String,
    pub warnings: Vec<String>,
}

pub fn parse_markdown_links(src: &str) -> (Vec<Link>, Vec<String>) {
    let mut links: Vec<Link> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();

    let opts = Options::empty();
    let parser = Parser::new_ext(src, opts);

    // Use a stack to track nested links. Each entry is (title, url).
    let mut stack: Vec<(String, String)> = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::Link(_link_type, dest, _title)) => {
                stack.push((String::new(), dest.to_string()));
            }
            Event::End(Tag::Link(_, _, _)) => {
                if stack.is_empty() {
                    continue;
                }
                let (title, url) = stack.pop().unwrap();
                if stack.is_empty() {
                    let mut final_title = title.clone();
                    let final_url = url.clone();
                    if final_title.trim().is_empty() {
                        final_title = final_url.clone();
                    }
                    if final_url.trim().is_empty() {
                        warnings.push(format!("リンク '{}' はURLが空のため無視されました。", final_title));
                        continue;
                    }
                    if Url::parse(&final_url).is_err() {
                        warnings.push(format!("リンク '{}' のURL '{}' は無効なため無視されました。", final_title, final_url));
                        continue;
                    }
                    links.push(Link::new(final_title, final_url));
                } else {
                    if let Some(parent) = stack.last_mut() {
                        parent.0.push_str(&title);
                    }
                }
            }
            Event::Text(text) => {
                if let Some(top) = stack.last_mut() {
                    top.0.push_str(&text);
                }
            }
            _ => {}
        }
    }

    // Run fallback scanner and use its results only if pulldown-cmark found nothing.
    let fb = fallback_parse_links_with_warnings(src);
    if !fb.0.is_empty() {
        if links.is_empty()
            || fb.0.iter().any(|f| !links.iter().any(|l| l.url == f.url))
        {
            links = fb.0;
            warnings.extend(fb.1);
        }
    }

    (links, warnings)
}

// Fallback: simple regex-based extraction for cases pulldown-cmark didn't recognize as links
fn fallback_parse_links_with_warnings(src: &str) -> (Vec<Link>, Vec<String>) {
    // Manual scanner that finds outermost bracketed link patterns: [title](url)
    // Handles nested brackets in the title by balancing brackets.
    let mut res: Vec<Link> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();

    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '[' {
            // found potential start
            let mut depth = 1usize;
            let mut j = i + 1;
            while j < chars.len() && depth > 0 {
                if chars[j] == '\\' {
                    // skip escaped char
                    j = j.saturating_add(2);
                    continue;
                }
                if chars[j] == '[' {
                    depth += 1;
                } else if chars[j] == ']' {
                    depth -= 1;
                }
                j += 1;
            }
            if depth == 0 {
                // j is position after the matching ']' (unescaped)
                let title_start = i + 1;
                let title_end_idx = j - 1; // inclusive index of the matching ']'
                // skip whitespace and any extra closing ']' characters that may appear before the URL
                let mut k = j;
                while k < chars.len() && (chars[k].is_whitespace() || chars[k] == ']') {
                    k += 1;
                }
                if k < chars.len() && chars[k] == '(' {
                    // parse parentheses for URL with simple balancing
                    let mut pdepth = 1usize;
                    let mut l = k + 1;
                    while l < chars.len() && pdepth > 0 {
                        if chars[l] == '\\' {
                            // skip escaped char
                            l = l.saturating_add(2);
                            continue;
                        }
                        if chars[l] == '(' {
                            pdepth += 1;
                        } else if chars[l] == ')' {
                            pdepth -= 1;
                        }
                        l += 1;
                    }
                    if pdepth == 0 {
                        let url_start = k + 1;
                        // l is position after the matching ')' ; exclude the closing paren from url
                        let url_end_idx = l.saturating_sub(2); // inclusive index of last char of URL
                        // Build title string from title_start..=title_end_idx, but handle escapes when scanning
                        let title: String = if title_end_idx >= title_start {
                            chars[title_start..=title_end_idx].iter().collect()
                        } else {
                            String::new()
                        };
                        let url: String = if url_end_idx >= url_start {
                            chars[url_start..=url_end_idx].iter().collect()
                        } else {
                            String::new()
                        };
                        // Unescape common markdown escapes in title, and trim
                        let title_unescaped = unescape_markdown(&title);
                        let url_trim = url.trim().to_string();
                        let mut final_title = title_unescaped.trim().to_string();
                        if final_title.is_empty() {
                            final_title = url_trim.clone();
                        }
                        if url_trim.is_empty() {
                            warnings.push(format!("リンク '{}' はURLが空のため無視されました。", final_title));
                        } else if Url::parse(&url_trim).is_err() {
                            warnings.push(format!("リンク '{}' のURL '{}' は無効なため無視されました。", final_title, url_trim));
                        } else {
                            res.push(Link::new(final_title, url_trim));
                        }
                        // advance i past the parsed URL (position after closing paren)
                        i = l;
                        continue;
                    }
                }
            }
        }
        i += 1;
    }
    (res, warnings)
}

fn unescape_markdown(s: &str) -> String {
    let mut out = s.replace("\\\\", "\\");
    out = out.replace("\\[", "[");
    out = out.replace("\\]", "]");
    out = out.replace("\\(", "(");
    out = out.replace("\\)", ")");
    out
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

pub fn convert_markdown_to_netscape(src: &str) -> Result<ConvertResult, ConvError> {
    let (links, warnings) = parse_markdown_links(src);
    Ok(ConvertResult {
        html: generate_netscape_html(&links),
        warnings,
    })
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
        let md = "- [Rust](https://www.rust-lang.org)\n- [Search](https://example.com/search)";
        let res = convert_markdown_to_netscape(md).unwrap();
        assert!(res.html.contains("https://www.rust-lang.org"));
        assert!(res.html.contains("Rust"));
        assert!(res.html.contains("https://example.com/search"));
        assert!(res.warnings.is_empty());
    }

    #[test]
    fn test_invalid_url() {
        let md = "- [Bad](ht!tp://no)";
        let (links, warnings) = parse_markdown_links(md);
        assert_eq!(links.len(), 0);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("無効"));
    }

    #[test]
    fn test_title_with_literal_brackets() {
        let md = r#"- [\[abc\]](https://example.com)"#;
        let (links, warnings) = parse_markdown_links(md);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].title, "[abc]");
        assert_eq!(links[0].url, "https://example.com");
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_nested_markdown_link() {
        let md = "- [outer [inner](https://inner)](https://outer)";
        let (links, warnings) = parse_markdown_links(md);
        assert_eq!(links.len(), 1);
        assert!(links[0].title.contains("inner"));
        assert_eq!(links[0].url, "https://outer");
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_empty_title_uses_url() {
        let md = "- [](https://example.com)";
        let (links, warnings) = parse_markdown_links(md);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].title, "https://example.com");
        assert_eq!(links[0].url, "https://example.com");
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_empty_url_is_ignored() {
        let md = "- [Title]()";
        let (links, warnings) = parse_markdown_links(md);
        assert_eq!(links.len(), 0);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("空"));
    }

    #[test]
    fn test_both_title_and_url_empty_is_ignored() {
        let md = "- []()";
        let (links, warnings) = parse_markdown_links(md);
        assert_eq!(links.len(), 0);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("空"));
    }

    #[test]
    fn test_unescaped_open_bracket_in_title_is_parsed() {
        let md = "- [[abc]](https://example.com)";
        let (links, warnings) = parse_markdown_links(md);
        assert_eq!(links.len(), 1);
        assert!(links[0].title.contains("abc"));
        assert_eq!(links[0].url, "https://example.com");
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_unescaped_closing_bracket_in_title_is_parsed() {
        let md = "- [abc]](https://example.com)";
        let (links, warnings) = parse_markdown_links(md);
        assert_eq!(links.len(), 1);
        assert!(links[0].title.contains("abc"));
        assert_eq!(links[0].url, "https://example.com");
        assert!(warnings.is_empty());
    }

    // debug tests removed
}
