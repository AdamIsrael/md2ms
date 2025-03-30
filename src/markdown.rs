// use std::collections::HashMap;

use crate::metadata::Metadata;
use regex::Regex;
use yaml_front_matter::{Document, YamlFrontMatter};

/// Parse the markdown document
pub fn parse_markdown(md: String) -> Result<Document<Metadata>, &'static str> {
    let mut document = Document {
        metadata: Metadata {
            author: None,
            content_warnings: None,
            heading: None,
            include: None,
            short_title: None,
            short_author: None,
            title: None,
        },
        content: "".to_string(),
    };

    if let Ok(doc) = YamlFrontMatter::parse::<Metadata>(&md) {
        document.metadata = doc.metadata;
        document.content = doc.content;
    } else {
        // No YAML front matter, so we'll just use the content
        document.content = md;
    }

    // Might want to move this out of this function. Trimming whitespace and links is easy, but other
    // markdown tranformations might be more complex, like italics or bold text, to work in Word.
    // Parsing other markdown, like tables and lists, will be even harder but not necessary right now.
    // This is supporting a very lightweight subset of Markdown (content, not formatting), so this should be enough.

    // Eliminate double whitespace
    document.content = trim_whitespace(document.content.as_str());

    // Remove hyperlinks
    document.content = trim_links(document.content.as_str());

    Ok(document)
}

/// Trim double-spaces from a string.
fn trim_whitespace(s: &str) -> String {
    let re = Regex::new(r"[ ]+").unwrap();
    re.replace_all(s, " ").to_string()
}

// Replace all links in a string with their target text
// Credit: https://github.com/GeckoEidechse/remove-markdown-links
fn trim_links(s: &str) -> String {
    let re = Regex::new(r"\[([^\[\]]+)\]\(([^)]+)\)").unwrap();
    re.replace_all(s, |caps: &regex::Captures| {
        caps.get(1).unwrap().as_str().to_string()
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SIMPLE_MARKDOWN_YFM: &str = r#"
    ---
    author: 'Adam Israel'
    short_author: 'Israel'
    short_title: 'Markdown'
    title: 'Parsing a Markdown file metadata into a struct'
    description: 'This tutorial walks you through the practice of parsing markdown files for metadata'
    tags: ['markdown', 'rust', 'files', 'parsing', 'metadata']
    date: '2021-09-13T03:48:00'
    ---
    This is the content of the markdown file
    "#;

    #[test]
    fn test_trim_whitespace() {
        let s = "This is a test.  This is only a test.\nIf this were an actual emergency, you would be instructed where to go and what to do.";
        assert!(trim_whitespace(s) == "This is a test. This is only a test.\nIf this were an actual emergency, you would be instructed where to go and what to do.");
    }

    #[test]
    fn test_trim_links() {
        let s = "This is a test. [This is a link](https://example.com). This is only a test.\nIf this were an actual emergency, you would be instructed where to go and what to do.";
        println!("{}", trim_links(s));
        assert!(trim_links(s) == "This is a test. This is a link. This is only a test.\nIf this were an actual emergency, you would be instructed where to go and what to do.");
    }

    #[test]
    fn test_parse_markdown() {
        let md = parse_markdown(SIMPLE_MARKDOWN_YFM.to_string()).unwrap();
        assert!(md.metadata.author == Some("Adam Israel".to_string()));
        assert!(
            md.metadata.title == Some("Parsing a Markdown file metadata into a struct".to_string())
        );
        assert!(md
            .content
            .contains("This is the content of the markdown file"))
    }
}
