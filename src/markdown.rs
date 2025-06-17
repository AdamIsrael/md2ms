// use std::collections::HashMap;

use crate::cmark::parse_paragraph;
use crate::constants;
use crate::context::Context;
use crate::metadata::Metadata;
use crate::pii::PII;
use docx_rs::*;
use regex::Regex;
use yaml_front_matter::{Document, YamlFrontMatter};


/// Convert the content of a Markdown into a collection of paragraphs.
fn content_to_paragraphs(mut content: String) -> Vec<Paragraph> {

    // Pre-process the content

    // Add support single and multi-line %% comment blocks %%
    let re = Regex::new(r"%%\s+.*?\s+%%").unwrap();
    content = Regex::replace_all(&re, content.as_str(), "").to_string();

    let mut paragraphs: Vec<Paragraph> = vec![];
    let sep = Paragraph::new()
        .add_run(Run::new().add_text("#"))
        .align(AlignmentType::Center)
        .size(constants::FONT_SIZE)
        .line_spacing(LineSpacing::new().after_lines(100));

    if content.lines().count() > 0 {
        content.lines().for_each(|line| {
            if !line.is_empty() {
                // need an "is separator function"
                if line.trim() == "#" {
                    paragraphs.push(sep.clone());
                } else {
                    // Parse the paragraph into runs, which will handle simple formatting.
                    let runs = parse_paragraph(line);

                    let mut p = Paragraph::new()
                        .line_spacing(
                            LineSpacing::new()
                                // https://stackoverflow.com/questions/19719668/how-is-line-spacing-measured-in-ooxml
                                .line_rule(LineSpacingType::Auto)
                                .line(480), // double spaced
                        )
                        // Indent the first line: one half-inch
                        // FIX: The indent is a little bigger than Shunn recommends (one half-inch)
                        // https://stackoverflow.com/questions/14360183/default-wordml-unit-measurement-pixel-or-point-or-inches
                        // 1.48cm == 0.5826772 inches == 839.05 dxa
                        .indent(None, Some(SpecialIndentType::FirstLine(839)), None, None);
                    for run in runs {
                        p = p.add_run(run);
                    }
                    paragraphs.push(p);
                }
            }
        });
    }
    paragraphs
}

pub fn flatten_markdown(
    ctx: &mut Context,
    document: Document<Metadata>,
) -> Result<Vec<Paragraph>, &'static str> {
    let mut paragraphs: Vec<Paragraph> = vec![];
    let mut sep = Paragraph::new();

    // TODO: support variable font sizes (typically 10/12pt.
    // If the metadata doesn't include an include stanza, there's nothing to flatten; it's a standalone document.
    if document.metadata.include.is_none() {
        println!("No include in metadata");
        return Ok(content_to_paragraphs(document.content));
    }

    for file in document.metadata.include.clone().unwrap() {
        // TODO: need the folders where we might want to show the chapter or act numbers.
        // I've added a per-folder metadata file, but need to handle it.
        // let markdown = ctx.get_file_metadata(file.clone());
        // println!("Markdown for {}: {:?}", file, markdown);

        if let Some(md) = ctx.get_file(file) {
            // is this still needed?
            if !sep.raw_text().is_empty() {
                paragraphs.push(sep.clone());
            }

            // If there is a heading in the metadata, add it here.
            if let Some(heading) = md.metadata.heading.clone() {
                // TODO: Add page break before the heading
                // Center heading on page?
                // TODO: Only page break/center if it's a new section. A new chapter,
                // for example, should start at the top of a new page
                paragraphs.push(
                    Paragraph::new()
                        .add_run(Run::new().add_text("").size(constants::FONT_SIZE))
                        .align(AlignmentType::Center)
                        .page_break_before(true)
                        .line_spacing(LineSpacing::new().after_lines(100)),
                );

                for _ in 0..23 {
                    paragraphs.push(Paragraph::new());
                }
                paragraphs.push(
                    Paragraph::new()
                        .add_run(Run::new().add_text(heading).size(constants::FONT_SIZE))
                        .align(AlignmentType::Center)
                        .line_spacing(LineSpacing::new().after_lines(100)),
                );
            }

            let mut p = content_to_paragraphs(md.content);
            if !p.is_empty() {
                paragraphs.append(&mut p);

                sep = Paragraph::new()
                    .add_run(Run::new().add_text("#"))
                    .align(AlignmentType::Center)
                    .size(constants::FONT_SIZE)
                    .line_spacing(LineSpacing::new().after_lines(100));
            }
        } else {
            // TODO: Handle this better. Return an Err maybe?
            // If a file is noted to be included, but we can't find it, that's a problem.
            // println!("Failed to get file: {}", file);
        }
    }

    Ok(paragraphs)
}

/// Parse the PII document
pub fn parse_pii(md: String) -> Result<Document<PII>, &'static str> {
    let mut pii = Document {
        metadata: PII {
            legal_name: None,
            address1: None,
            address2: None,
            city: None,
            state: None,
            postal_code: None,
            country: None,
            email: None,
            phone: None,
            affiliations: None,
        },
        content: "".to_string(),
    };

    if let Ok(doc) = YamlFrontMatter::parse::<PII>(&md) {
        pii.metadata = doc.metadata;
        // pii.content = doc.content;
    }

    Ok(pii)
}

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
        // println!("{}", trim_links(s));
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
