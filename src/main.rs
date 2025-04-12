// Syntax: md2ms [options] <file>
// md2ms --output-dir <dir> <files>
use clap::Parser;
use yaml_front_matter::Document;

use docx_rs::*;
use md2ms::context::Context;
use md2ms::markdown::flatten_markdown;
use md2ms::metadata::Metadata;
use md2ms::utils::round_up;
use md2ms::{Cli, Commands};

pub fn main() -> Result<(), DocxError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Obsidian(_args) => {
            // TODO: write code to integrate w/ Obsidian
            // TODO: remove Obsidian integration?
            println!("'obsidian' was used but is not implemented yet");
        }
        Commands::Compile(args) => {
            let ctx = Context::new(args);
            return compile(ctx);
        }
    }

    Ok(())
}

fn compile(mut ctx: Context) -> Result<(), DocxError> {
    // If there are no files, exit.
    if ctx.files.is_empty() {
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
    // TODO: Case-sensitivity? It might be Metadata.md
    if ctx.files.contains_key("metadata.md") {
        if let Some(metadata) = ctx.files.get("metadata.md") {
            mddoc.metadata = metadata.metadata.clone();
        }
    } else {
        // use the metadata from the first file
        if let Some(v) = ctx.files.values().next() {
            mddoc.metadata = v.metadata.clone();
            mddoc.content = v.content.clone();
        }
    }

    // Parse the PII document
    // let pii = parse_pii(ctx.pii);

    // Now that we have the file(s), we can join them into one document

    // Parse the Markdown
    let metadata = mddoc.metadata.clone();
    if let Ok(md) = flatten_markdown(&mut ctx, mddoc) {
        // Using this crate for now, but maybe convert this to my own code
        // let wc = words_count::count(&md.iter().map(|p| p.raw_text()).collect::<String>());
        let wc = words_count::count(md.iter().map(|p| p.raw_text()).collect::<String>());
        // Round up
        let nwc = round_up(wc.words);
        if ctx.word_count {
            println!("Approximate Word count: {}", nwc);
            return Ok(());
        }

        // TODO: Need to support output dir here.
        let mut docx_file = ctx.output_dir;

        // Create the directory, if it doesn't exist
        if std::fs::create_dir_all(docx_file.clone()).is_ok() {
            docx_file.push(format!("{}.docx", metadata.title.clone().unwrap()));
            println!("Full path to output file: {:?}", docx_file);
        } else {
            // Abort if we can't create the directory
            return Err(DocxError::Unknown);
        }

        let path = std::path::Path::new(&docx_file);
        let file = std::fs::File::create(path).unwrap();

        let mut pii = TableCell::new();

        // If we're not anonymous, add the author's contact information
        if !ctx.anonymous {
            if let Some(my) = ctx.pii {
                let mut paragraphs: Vec<Paragraph> = Vec::new();

                // Add to the PII information as available
                if let Some(legal_name) = my.metadata.legal_name {
                    paragraphs.push(
                        Paragraph::new()
                            .add_run(Run::new().add_text(legal_name).size(ctx.font_size)),
                    );
                }
                if let Some(address1) = my.metadata.address1 {
                    paragraphs.push(
                        Paragraph::new().add_run(Run::new().add_text(address1).size(ctx.font_size)),
                    );
                }
                if let Some(address2) = my.metadata.address2 {
                    paragraphs.push(
                        Paragraph::new().add_run(Run::new().add_text(address2).size(ctx.font_size)),
                    );
                }
                if let Some(city) = my.metadata.city {
                    if let Some(state) = my.metadata.state {
                        if let Some(postal_code) = my.metadata.postal_code {
                            paragraphs.push(
                                Paragraph::new().add_run(
                                    Run::new()
                                        .add_text(format!("{}, {}, {}", city, state, postal_code))
                                        .size(ctx.font_size),
                                ),
                            );
                        }
                    }
                }
                if let Some(country) = my.metadata.country {
                    paragraphs.push(
                        Paragraph::new().add_run(Run::new().add_text(country).size(ctx.font_size)),
                    );
                }
                if let Some(email) = my.metadata.email {
                    paragraphs.push(
                        Paragraph::new().add_run(Run::new().add_text(email).size(ctx.font_size)),
                    );
                }
                if let Some(phone) = my.metadata.phone {
                    paragraphs.push(
                        Paragraph::new().add_run(Run::new().add_text(phone).size(ctx.font_size)),
                    );
                }
                if let Some(affiliations) = my.metadata.affiliations {
                    let affiliation = format!("Active member: {}", affiliations.join(", "));
                    paragraphs.push(
                        Paragraph::new()
                            .add_run(Run::new().add_text(affiliation).size(ctx.font_size)),
                    );
                }
                // Add all of the PII information to the header
                for p in paragraphs {
                    pii = pii.add_paragraph(p);
                }
            } else {
                pii = pii.add_paragraph(
                    Paragraph::new().add_run(Run::new().add_text("No PII supplied.")),
                );
            }
        }

        let mut table = Table::new(vec![TableRow::new(vec![
            pii,
            // Don't add if anonymous is true
            TableCell::new().add_paragraph(
                Paragraph::new()
                    .add_run(
                        Run::new()
                            .add_text(format!("{} words", nwc))
                            .size(ctx.font_size),
                    )
                    .align(AlignmentType::Right),
            ),
        ])]);

        // Turn off borders
        table = table.clear_all_border();

        // This is a hack. Can't seem to find a way to set it to autofit, but this works because it's an 8 inch page, with 1 inch margins
        table = table.width(1440 * 6, WidthType::Dxa);

        let title = Paragraph::new()
            .add_run(
                Run::new()
                    .add_text(metadata.title.unwrap())
                    .size(ctx.font_size),
            )
            .align(AlignmentType::Center)
            .line_spacing(LineSpacing::new().after_lines(100));

        let mut byline = Paragraph::new();
        if !ctx.anonymous {
            byline = byline
                .add_run(
                    Run::new()
                        .add_text(format!("by {}", metadata.author.unwrap()))
                        .size(ctx.font_size),
                )
                .align(AlignmentType::Center);
        }

        let end = Paragraph::new()
            .add_run(Run::new().add_text("END"))
            .align(AlignmentType::Center)
            .size(ctx.font_size)
            .line_spacing(LineSpacing::new().after_lines(100));

        let mut header_text = format!("{} / ", metadata.short_title.as_ref().unwrap());
        if !ctx.anonymous {
            // Get the short author name and title from the metadata
            header_text = format!(
                "{} / {} / ",
                metadata.short_author.unwrap(),
                metadata.short_title.unwrap()
            );
        }

        let header = Header::new().add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text(header_text).size(ctx.font_size))
                .align(AlignmentType::Right)
                .add_page_num(PageNum::new()),
        );

        let mut doc = Docx::new()
            // .add_style(s)
            // Add flag to set the default font? TNR is a fine default, but some markets want Courier (and I like it better)
            .default_fonts(RunFonts::new().ascii(ctx.font))
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
    // use super::*;
}
