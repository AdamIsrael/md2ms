// use std::collections::HashMap;

use crate::metadata::Metadata;
use regex::Regex;
use yaml_front_matter::{Document, YamlFrontMatter};

// pub fn parse_markdown_files(
//     files: Vec<Document<Metadata>>,
// ) -> Result<Vec<Document<Metadata>>, &'static str> {
//     let mut documents = vec![];
//     for file in files {
//         let doc = parse_markdown(file.content);
//         documents.push(doc.unwrap());
//     }
//     Ok(documents)
// }

/// Parse the markdown document
pub fn parse_markdown(md: String) -> Result<Document<Metadata>, &'static str> {
    let mut document = Document {
        metadata: Metadata {
            author: None,
            content_warnings: None,
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

    // TODO: Detect scene break(s)

    // Eliminate double whitespace
    document.content = trim_whitespace(document.content.as_str());

    return Ok(document);
}

/// Trim double-spaces from a string.
fn trim_whitespace(s: &str) -> String {
    let re = Regex::new(r"[ ]+").unwrap();
    re.replace_all(s, " ").to_string()
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
