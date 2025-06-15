// Syntax: md2ms [options] <file>
// md2ms --output-dir <dir> <files>
use md2ms::constants;

use clap::Parser;
use yaml_front_matter::Document;

use docx_rs::*;
use md2ms::context::Context;
use md2ms::markdown::flatten_markdown;
use md2ms::metadata::Metadata;
use md2ms::obsidian::update_obsidian_vault;
use md2ms::utils::round_up;
use md2ms::{Cli, Commands};

pub fn main() -> Result<(), DocxError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Obsidian(args) => {
            // TODO: write code to integrate w/ Obsidian
            // TODO: remove Obsidian integration?
            // println!("'obsidian' was used but is not implemented yet");

            if let Some(export_path) = args.export_path.clone() {
                if let Some(vault_folder) = args.vault_folder.clone() {
                    println!("Updating Obsidian vault...");
                    update_obsidian_vault(
                        &args.obsidian_path,
                        &export_path,
                        &vault_folder,
                        args.overwrite.unwrap_or(false),
                    );
                }
            }
        }

        Commands::Compile(args) => {
            let ctx = Context::new(args);

            if ctx.word_count {
                // We only need to run compile once to get the word count
                let mut c = ctx.clone();
                let _ = compile(&mut c);
                return Ok(());
            }
            // TODO: We need to iterate through the combination of supported configurations in order
            // to generate a folder of manuscripts, not just a single manuscript.
            for font in constants::FONTS {

                // Create a clone of the context that we can modify without affecting the original.
                let mut c = ctx.clone();
                c.font = font.to_string();
                c.classic = false;
                c.anonymous = false;
                let _ = compile(&mut c);

                let mut c = ctx.clone();
                c.font = font.to_string();
                c.classic = false;
                c.anonymous = true;
                let _ = compile(&mut c);

                let mut c = ctx.clone();
                c.font = font.to_string();
                c.classic = true;
                c.anonymous = false;
                let _ = compile(&mut c);

                let mut c = ctx.clone();
                c.font = font.to_string();
                c.classic = true;
                c.anonymous = true;
                let _ = compile(&mut c);

            }
            // return compile(ctx);
        }
    }

    Ok(())
}

fn compile(ctx: &mut Context) -> Result<(), DocxError> {
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
    if let Ok(md) = flatten_markdown(ctx, mddoc) {
        // Using this crate for now, but maybe convert this to my own code
        // let wc = words_count::count(&md.iter().map(|p| p.raw_text()).collect::<String>());
        let wc = words_count::count(md.iter().map(|p| p.raw_text()).collect::<String>());
        // Round up
        let nwc = round_up(wc.words);
        if ctx.word_count {
            println!("Approximate Word count: {}", nwc);
            return Ok(());
        }

        // A PathBuf to build the path to the output file
        let docx_file = &mut ctx.output_dir;

        let mut format = String::from("Modern");
        if ctx.classic {
            format = String::from("Classic");
        }

        docx_file.push(format!("{}/", metadata.title.clone().unwrap()));

        // Create the directory, if it doesn't exist
        if std::fs::create_dir_all(docx_file.clone()).is_ok() {
            // Finally, format the file name with title, format, font, and if it's anonymous or not.
            // `Drafts/{title}/{title} - {format} - {font} ({anon}).docx`
            if ctx.anonymous {
                docx_file.push(format!("{} - {} - {} (Anonymous).docx", metadata.title.clone().unwrap(), format, ctx.font.clone()));
            } else {
                docx_file.push(format!("{} - {} - {}.docx", metadata.title.clone().unwrap(), format, ctx.font.clone()));
            }
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
            if let Some(my) = &ctx.pii {
                let mut paragraphs: Vec<Paragraph> = Vec::new();

                // Add to the PII information as available
                if let Some(legal_name) = my.metadata.legal_name.clone() {
                    paragraphs.push(
                        Paragraph::new()
                            .add_run(Run::new().add_text(legal_name).size(constants::FONT_SIZE)),
                    );
                }
                if let Some(address1) = my.metadata.address1.clone() {
                    paragraphs.push(
                        Paragraph::new().add_run(Run::new().add_text(address1).size(constants::FONT_SIZE)),
                    );
                }
                if let Some(address2) = my.metadata.address2.clone() {
                    paragraphs.push(
                        Paragraph::new().add_run(Run::new().add_text(address2).size(constants::FONT_SIZE)),
                    );
                }
                if let Some(city) = my.metadata.city.clone() {
                    if let Some(state) = my.metadata.state.clone() {
                        if let Some(postal_code) = my.metadata.postal_code.clone() {
                            paragraphs.push(
                                Paragraph::new().add_run(
                                    Run::new()
                                        .add_text(format!("{}, {}, {}", city, state, postal_code))
                                        .size(constants::FONT_SIZE),
                                ),
                            );
                        }
                    }
                }
                if let Some(country) = my.metadata.country.clone() {
                    paragraphs.push(
                        Paragraph::new().add_run(Run::new().add_text(country).size(constants::FONT_SIZE)),
                    );
                }
                if let Some(email) = my.metadata.email.clone() {
                    paragraphs.push(
                        Paragraph::new().add_run(Run::new().add_text(email).size(constants::FONT_SIZE)),
                    );
                }
                if let Some(phone) = my.metadata.phone.clone() {
                    paragraphs.push(
                        Paragraph::new().add_run(Run::new().add_text(phone).size(constants::FONT_SIZE)),
                    );
                }
                if let Some(affiliations) = my.metadata.affiliations.clone() {
                    let affiliation = format!("Active member: {}", affiliations.join(", "));
                    paragraphs.push(
                        Paragraph::new()
                            .add_run(Run::new().add_text(affiliation).size(constants::FONT_SIZE)),
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
                            .size(constants::FONT_SIZE),
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
                    .size(constants::FONT_SIZE),
            )
            .align(AlignmentType::Center)
            .line_spacing(LineSpacing::new().after_lines(100));

        let mut byline = Paragraph::new();
        if !ctx.anonymous {
            byline = byline
                .add_run(
                    Run::new()
                        .add_text(format!("by {}", metadata.author.unwrap()))
                        .size(constants::FONT_SIZE),
                )
                .align(AlignmentType::Center);
        }

        let end = Paragraph::new()
            .add_run(Run::new().add_text("END"))
            .align(AlignmentType::Center)
            .size(constants::FONT_SIZE)
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
                .add_run(Run::new().add_text(header_text).size(constants::FONT_SIZE))
                .align(AlignmentType::Right)
                .add_page_num(PageNum::new()),
        );

        let mut doc = Docx::new()
            // .add_style(s)
            // Add flag to set the default font? TNR is a fine default, but some markets want Courier (and I like it better)
            .default_fonts(RunFonts::new().ascii(constants::DEFAULT_FONT))
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
