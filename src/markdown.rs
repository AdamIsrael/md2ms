// use std::collections::HashMap;

use crate::cmark::parse_paragraph;
use crate::constants;
use crate::context::Context;
use crate::error::Md2msError;
use crate::metadata::Metadata;
use crate::pii::PII;
use docx_rs::*;
use regex::Regex;
use yaml_front_matter::{Document, YamlFrontMatter};

/// Strip Markdown comments out of the content
fn strip_comments(mut content: String) -> String {
    // Add support single and multi-line %% comment blocks %%
    let re = Regex::new(r"(?s)%%\s+.*?\s+%%").unwrap();
    content = Regex::replace_all(&re, content.as_str(), "").to_string();
    // Trim the whitespace we may have left behind
    content.trim().to_string()
}

/// Convert the content of a Markdown into a collection of paragraphs.
fn content_to_paragraphs(mut content: String) -> Vec<Paragraph> {
    // Pre-process the content

    // Add support single and multi-line %% comment blocks %%
    content = strip_comments(content);

    let mut paragraphs: Vec<Paragraph> = vec![];
    let sep = Paragraph::new()
        .add_run(Run::new().add_text("#"))
        .align(AlignmentType::Center)
        .size(constants::FONT_SIZE)
        .line_spacing(LineSpacing::new().after_lines(100));

    if content.lines().count() > 0 {
        content.lines().for_each(|line| {
            // If the line is empty, skip it. We'll handle line spacing elsewhere.
            if !line.trim().is_empty() {
                // need an "is separator function"
                if line.trim() == "#" {
                    // This will add the separator for single Markdown documents that explicitly
                    // include the separator, like the `standalone.md` example.
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
                        // https://stackoverflow.com/questions/14360183/default-wordml-unit-measurement-pixel-or-point-or-inches
                        // 1.48cm == 0.5826772 inches == 839.05 dxa
                        // According to a Scrivener-compiled document, the indentation should be:
                        // 0.63cm == 0.2480315 inches == 357.16536 dxa
                        .indent(None, Some(SpecialIndentType::FirstLine(357)), None, None);
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
) -> Result<Vec<Paragraph>, Md2msError> {
    let mut paragraphs: Vec<Paragraph> = vec![];
    let mut sep = Paragraph::new();

    // TODO: support variable font sizes (typically 10/12pt.
    // If the metadata doesn't include an include stanza, there's nothing to flatten; it's a standalone document.
    if document.metadata.include.is_none() {
        // println!("No include in metadata");
        return Ok(content_to_paragraphs(document.content));
    }

    for file in document.metadata.include.clone().unwrap() {
        // TODO: need the folders where we might want to show the chapter or act numbers.
        // I've added a per-folder metadata file, but need to handle it.
        // let markdown = ctx.get_file_metadata(file.clone());
        // println!("Markdown for {}: {:?}", file, markdown);

        if !ctx.file_exists(file.clone()) {
            return Err(Md2msError::FileNotFound(file));
        }

        if let Some(md) = ctx.get_file(file.clone()) {
            // is this still needed?

            // If there is a heading in the metadata, add it here.
            if let Some(heading) = md.metadata.heading.clone() {
                // Reset the separator
                sep = Paragraph::new();
                // TODO: Add page break before the heading
                // Center heading on page?
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

            // If there is a separator, add it to the list of paragraphs to create a visible
            // break between scenes.
            if !sep.raw_text().is_empty() {
                paragraphs.push(sep.clone());
            }

            let mut p = content_to_paragraphs(md.content);
            if !p.is_empty() {
                // Add all the paragraphs to the current list of paragraphs
                paragraphs.append(&mut p);

                sep = Paragraph::new()
                    .add_run(Run::new().add_text("#"))
                    .align(AlignmentType::Center)
                    .size(constants::FONT_SIZE)
                    .line_spacing(LineSpacing::new().after_lines(100));
            }
        } else {
            // If a file is noted to be included, but we can't find it, that's a problem.
            return Err(Md2msError::FileNotFound(file));
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
    document.content = trim_doublespace(document.content.as_str());

    // Remove hyperlinks
    document.content = trim_links(document.content.as_str());

    Ok(document)
}

/// Trim double-spaces from a string.
fn trim_doublespace(s: &str) -> String {
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

    const COMMENTS: &str = r#"
    %%

    Attempting to follow the structure from this series of articles:
    https://storybilder.com/blog/structure-flash-fiction-part-1
    https://storybilder.com/blog/structure-flash-fiction-part-2

    %%

    %% 1. In 2-3 sentences, set the place and introduce the character(s). %%

    "#;

    #[test]
    fn test_strip_comments() {
        let content = strip_comments(COMMENTS.to_string());
        assert!(content.is_empty());
    }

    #[test]
    fn test_trim_doublespace() {
        let s = "This is a test.  This is only a test.\nIf this were an actual emergency, you would be instructed where to go and what to do.";
        assert!(trim_doublespace(s) == "This is a test. This is only a test.\nIf this were an actual emergency, you would be instructed where to go and what to do.");
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
