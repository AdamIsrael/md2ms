// Syntax: md2ms [options] <file>
// m2ms --output-dir <dir> <files>
use clap::Parser;
use yaml_front_matter::Document;

use docx_rs::*;
use md2ms::context::Context;
use md2ms::metadata::Metadata;
use md2ms::utils::{get_file_basedir, round_up};
use std::path::PathBuf;

use words_count;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The file or directory containing the manuscript in Markdown format
    filename_or_path: String,

    /// The font to use in the manuscript
    #[arg(long, value_name = "Times New Roman")]
    font: Option<String>,

    /// The output directory
    #[arg(short, long, value_name = "FILE")]
    output_dir: Option<PathBuf>,
}

/// Convert the content of a Markdown into a collection of paragraphs.
fn content_to_paragraphs(content: String) -> Vec<Paragraph> {
    let mut paragraphs: Vec<Paragraph> = vec![];
    let sep = Paragraph::new()
        .add_run(Run::new().add_text("#"))
        .align(AlignmentType::Center)
        .size(24)
        .line_spacing(LineSpacing::new().after_lines(100));

    if content.lines().count() > 0 {
        content.lines().for_each(|line| {
            if line.len() > 0 {
                // need an "is separator function"
                if line.trim() == "#" {
                    paragraphs.push(sep.clone());
                } else {
                    paragraphs.push(
                        Paragraph::new()
                            .add_run(Run::new().add_text(line).size(24))
                            .line_spacing(
                                LineSpacing::new()
                                    // https://stackoverflow.com/questions/19719668/how-is-line-spacing-measured-in-ooxml
                                    .line_rule(LineSpacingType::Auto)
                                    .line(480), // double spaced
                            )
                            // Indent the first line
                            // https://stackoverflow.com/questions/14360183/default-wordml-unit-measurement-pixel-or-point-or-inches
                            // 1.48cm == 0.5826772 inches == 839.05 dxa
                            .indent(None, Some(SpecialIndentType::FirstLine(839)), None, None),
                    );
                }
            }
        });
    }
    paragraphs
}

// I think this needs to be refactored to return a collection of Paragraphs, so that we can insert things like chapter titles
// and the like between them. Kinda ugh, but that'll also fix how to center the scene breaks.
fn flatten_markdown(
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
            if sep.raw_text().len() > 0 {
                paragraphs.push(sep.clone());
            }

            // If there is a heading in the metadata, add it here.
            md.metadata.heading.clone().map(|heading| {
                // TODO: Add page break before the heading
                // Center heading on page?
                // TODO: Only page break/center if it's a new section. A new chapter,
                // for example, should start at the top of a new page
                paragraphs.push(
                    Paragraph::new()
                        .add_run(Run::new().add_text("").size(24))
                        .align(AlignmentType::Center)
                        .page_break_before(true)
                        .line_spacing(LineSpacing::new().after_lines(100)),
                );

                for _ in 0..23 {
                    paragraphs.push(Paragraph::new());
                }
                paragraphs.push(
                    Paragraph::new()
                        .add_run(Run::new().add_text(heading).size(24))
                        .align(AlignmentType::Center)
                        // .page_break_before(true)
                        .line_spacing(LineSpacing::new().after_lines(100)),
                );
            });

            let mut p = content_to_paragraphs(md.content);
            if p.len() > 0 {
                paragraphs.append(&mut p);

                sep = Paragraph::new()
                    .add_run(Run::new().add_text("#"))
                    .align(AlignmentType::Center)
                    .size(24)
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

pub fn main() -> Result<(), DocxError> {
    // Take the filename from positional arguments
    let args = Args::parse();

    let mut ctx = Context::new(args.filename_or_path.clone());
    // for file in ctx.files.keys() {
    //     // println!("{}/{}", path_dir, file);
    //     println!("File: {}", file);
    // }

    // return Ok(());

    // If filename_or_path is a directory, process all the files in the directory
    // and look for the metadata to assemble the manuscript

    let basedir = get_file_basedir(args.filename_or_path.clone());
    println!("Got basedir: {:?}", basedir);
    // let files = get_files(basedir.clone(), args.filename_or_path.clone());
    // for file in files.keys() {
    //     println!("{}/{}", path_dir, file);
    // }

    // If there are no files, exit.
    if ctx.files.len() == 0 {
        println!("No files found in {:?}", args.filename_or_path);
        return Ok(());
    }

    let mut mddoc = Document {
        metadata: Metadata {
            content_warnings: None,
            author: None,
            short_author: None,
            heading: None,
            include: None,
            short_title: None,
            title: None,
        },
        content: "".to_string(),
    };

    // Check for the presence of base metadata.md
    if ctx.files.contains_key("metadata.md") {
        println!("Got metadata!");
        if let Some(metadata) = ctx.files.get("metadata.md") {
            mddoc.metadata = metadata.metadata.clone();
        }
    } else {
        // use the metadata from the first file
        println!("Found standalone file?");
        ctx.files.values().next().map(|v| {
            mddoc.metadata = v.metadata.clone();
            mddoc.content = v.content.clone();
        });
    }

    // Now that we have the file(s), we can join them into one document

    // Parse the Markdown
    let metadata = mddoc.metadata.clone();
    if let Ok(md) = flatten_markdown(&mut ctx, mddoc) {
        // Using this crate for now, but maybe convert this to my own code
        let wc = words_count::count(&md.iter().map(|p| p.raw_text()).collect::<String>());

        // Round up
        let nwc = round_up(wc.words);
        println!("Approximate Word count: {}", nwc);

        // // Eliminate double whitespace
        // println!("Metadata: {:?}", metadata);
        let docx_file = format!("{}.docx", metadata.title.clone().unwrap());
        let path = std::path::Path::new(&docx_file);
        let file = std::fs::File::create(path).unwrap();

        let mut table = Table::new(vec![TableRow::new(vec![
            TableCell::new()
                // TODO: Pull the author information from the metadata
                .add_paragraph(
                    Paragraph::new().add_run(Run::new().add_text("Adam Israel").size(24)),
                )
                .add_paragraph(
                    Paragraph::new().add_run(Run::new().add_text("P.O. Box 1946").size(24)),
                )
                .add_paragraph(
                    Paragraph::new().add_run(Run::new().add_text("Tilbury, ON, Canada").size(24)),
                )
                .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Canada").size(24)))
                .add_paragraph(
                    Paragraph::new().add_run(Run::new().add_text("adam@adamisrael.com").size(24)),
                ),
            TableCell::new().add_paragraph(
                Paragraph::new()
                    .add_run(Run::new().add_text(format!("{} words", nwc)).size(24))
                    .align(AlignmentType::Right),
            ),
        ])]);

        // Turn off borders
        table = table.clear_all_border();

        // This is a hack. Can't seem to find a way to set it to autofit, but this works because it's an 8 inch page, with 1 inch margins
        table = table.width(1440 * 6, WidthType::Dxa);
        // println!("{:?}", table);

        let title = Paragraph::new()
            .add_run(Run::new().add_text(metadata.title.unwrap()).size(24))
            .align(AlignmentType::Center)
            .line_spacing(LineSpacing::new().after_lines(100));

        let byline = Paragraph::new()
            .add_run(
                Run::new()
                    .add_text(format!("by {}", metadata.author.unwrap()))
                    .size(24),
            )
            .align(AlignmentType::Center);

        let end = Paragraph::new()
            .add_run(Run::new().add_text("END"))
            .align(AlignmentType::Center)
            .size(24)
            .line_spacing(LineSpacing::new().after_lines(100));

        // Get the short author name and title from the metadata
        let header_text = format!(
            "{} / {} / ",
            metadata.short_author.unwrap(),
            metadata.short_title.unwrap()
        );
        let header = Header::new().add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text(header_text))
                .align(AlignmentType::Right)
                .add_page_num(PageNum::new()),
        );

        let font = args.font.unwrap_or("Times New Roman".to_string());

        let mut doc = Docx::new()
            // .add_style(s)
            // Add flag to set the default font? TNR is a fine default, but some markets want Courier (and I like it better)
            .default_fonts(RunFonts::new().ascii(font))
            .header(header)
            .first_header(Header::new())
            .add_table(table)
            // There are 46 lines per page. The title should appear at the 1/3 to 1/2 point
            // So 15 lines down, including the header (5-6 lines)
            .add_paragraph(Paragraph::new())
            .add_paragraph(Paragraph::new())
            .add_paragraph(Paragraph::new())
            .add_paragraph(Paragraph::new())
            .add_paragraph(Paragraph::new())
            .add_paragraph(Paragraph::new())
            .add_paragraph(Paragraph::new())
            .add_paragraph(Paragraph::new())
            .add_paragraph(Paragraph::new())
            .add_paragraph(Paragraph::new())
            // Add the title and byline
            .add_paragraph(title)
            .add_paragraph(byline)
            .add_paragraph(Paragraph::new())
            .add_paragraph(Paragraph::new());

        // Now we need to add the content of the manuscript
        for p in md {
            doc = doc.add_paragraph(p);
        }

        // Signal the end of the document
        doc = doc.add_paragraph(end);

        // Build and pack the document
        doc.build().pack(file)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
