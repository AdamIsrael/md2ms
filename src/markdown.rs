use crate::metadata::Metadata;
use yaml_front_matter::{Document, YamlFrontMatter};

/// Parse the markdown document
pub fn parse_markdown(md: String) -> Result<Document<Metadata>, &'static str> {
    if let Ok(document) = YamlFrontMatter::parse::<Metadata>(&md) {
        return Ok(document);
    }
    Err("Couldn't parse frontmatter")
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
    fn test_parse_markdown() {
        let md = parse_markdown(SIMPLE_MARKDOWN_YFM.to_string()).unwrap();
        assert!(md.metadata.author == "Adam Israel");
        assert!(md.metadata.title == "Parsing a Markdown file metadata into a struct");
        assert!(md
            .content
            .contains("This is the content of the markdown file"))
    }
}
