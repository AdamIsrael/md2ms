// uses the pulldown-cmark crate to parse the markdown
//
use crate::constants;
use docx_rs::Run;
use pulldown_cmark::Options;
use pulldown_cmark::{Event, Parser, Tag, TextMergeStream};

pub fn new_run_with_size() -> Run {
    Run::new().size(constants::FONT_SIZE)
}

/// Parse a paragraph of a Markdown document into a list of Runs
pub fn parse_paragraph(input: &str) -> Vec<Run> {
    let mut runs: Vec<Run> = vec![];

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(input, options);
    let iterator = TextMergeStream::new(parser);

    let mut run = new_run_with_size();
    for event in iterator {
        match event {
            Event::Start(name) => {
                // save the current run and start a new one with the correct formatting
                match name {
                    Tag::Paragraph => {}
                    Tag::Emphasis => {
                        // It used to be practice to UNDERLINE emphasised text, because typewriters couldn't do italics.
                        // That's no longer the case with digital text, so we'll use italics instead.
                        // TODO: Make this configurable?
                        runs.push(run);
                        run = new_run_with_size().italic();
                    }
                    Tag::Strong => {
                        runs.push(run);
                        run = new_run_with_size().bold();
                    }
                    Tag::Strikethrough => {
                        runs.push(run);
                        run = new_run_with_size().strike();
                    }
                    _ => {}
                }
            }
            Event::Text(text) => {
                // TODO: Might be a good place to handle emdash and endash here?
                // "There’s no need to put spaces around the dash." -- Shunn
                run = run.add_text(text.to_string());
            }
            Event::End(_) => {
                // We're at the end of a run, so save what we have and start the next one.
                runs.push(run);
                run = new_run_with_size();
            }
            _ => {}
        }
    }
    runs
}

#[cfg(test)]
mod tests {
    use super::*; // Import the parent module's items

    #[test]
    fn test_basic_parsing() {
        let input = "Hello world, this is a ~~complicated~~ *very simple* _example_.";
        let runs = parse_paragraph(input);
        assert!(runs.len() == 7);
    }
}
