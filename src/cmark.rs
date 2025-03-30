// uses the pulldown-cmark crate to parse the markdown
use docx_rs::Run;
use pulldown_cmark::Options;
use pulldown_cmark::{Event, Parser, Tag, TagEnd, TextMergeStream};

/// Parse a paragraph of a Markdown document into a list of Runs
pub fn parse_paragraph(input: &str) -> Vec<Run> {
    let mut runs: Vec<Run> = vec![];

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(input, options);
    let iterator = TextMergeStream::new(parser);

    let mut run = Run::new();
    for event in iterator {
        match event {
            Event::Start(name) => {
                // save the current run and start a new one with the correct formatting
                match name {
                    Tag::Paragraph => {}
                    Tag::Emphasis => {
                        runs.push(run);
                        run = Run::new().italic();
                    }
                    Tag::Strong => {
                        runs.push(run);
                        run = Run::new().bold();
                    }
                    Tag::Strikethrough => {
                        runs.push(run);
                        run = Run::new().strike();
                    }
                    _ => {}
                }
            }
            Event::Text(text) => {
                run = run.add_text(text.to_string());
            }
            Event::End(name) => match name {
                _ => {
                    runs.push(run);
                    run = Run::new();
                }
            },
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
        let input = "Hello world, this is a ~~complicated~~ *very simple* example.";
        // println!("{}", input);
        let runs = parse_paragraph(input);
        // println!("There are {} runs", runs.len());
        // for run in &runs {
        //     println!("{:?}", run);
        // }
        assert!(runs.len() == 5);
        // println!("{:?}", runs);
    }
}
