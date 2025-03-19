// Syntax: md2ms [options] <file>
// m2ms --reference-doc <file> --output-dir <dir> --data-dir <dir>  <files>
use clap::Parser;
// use pandoc::OutputKind;

use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use docx_rs::*;
use md2ms::markdown::parse_markdown;
use regex::Regex;

use words_count;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The Markdown file containing the manuscript
    filename: String,

    /// The font to use in the manuscript
    #[arg(long, value_name = "Times New Roman")]
    font: Option<String>,

    /// The output directory
    #[arg(short, long, value_name = "FILE")]
    output_dir: Option<PathBuf>,
}

// fn main() {
pub fn main() -> Result<(), DocxError> {
    // Take the filename from positional arguments
    let args = Args::parse();

    // Slurp the file
    let md = slurp(args.filename);

    // Parse the Markdown
    if let Ok(mut md) = parse_markdown(md) {
        // Using this crate for now, but maybe convert this to my own code
        let wc = words_count::count(&md.content);

        // // Round up
        let nwc = round_up(wc.words);
        println!("Approximate Word count: {}", nwc);
        // md.content = md.content.replace("{WORDCOUNT}", nwc.to_string().as_str());

        // Eliminate double whitespace
        let re = Regex::new(r"\s+").unwrap();
        md.content = re.replace_all(md.content.as_str(), " ".to_string()).into();

        let docx_file = format!("{}.docx", md.metadata.title);
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

        // TODO: Turn off borders
        //
        // This is a hack. Can't seem to find a way to set it to autofit, but this works because it's an 8 inch page, with 1 inch margins
        table = table.width(1440 * 6, WidthType::Dxa);
        // println!("{:?}", table);

        let title = Paragraph::new()
            .add_run(Run::new().add_text(md.metadata.title))
            .align(AlignmentType::Center)
            .size(24)
            .line_spacing(LineSpacing::new().after_lines(100));

        let byline = Paragraph::new()
            .add_run(Run::new().add_text(format!("by {}", md.metadata.author)))
            .align(AlignmentType::Center)
            .size(24);

        // 46 lines per page. Need to calculate the halfway point and add blank paragraphs to center the title

        // TODO: get the short author name and title from the metadata
        let header_text = format!(
            "{} / {} / ",
            md.metadata.short_author, md.metadata.short_title
        );
        let header = Header::new().add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text(header_text))
                .align(AlignmentType::Right)
                .add_page_num(PageNum::new()),
        );

        let mut doc = Docx::new()
            // .add_style(s)
            // Add flag to set the default font? TNR is a fine default, but some markets want Courier
            .default_fonts(RunFonts::new().ascii("Times New Roman"))
            .header(header)
            .first_header(Header::new())
            .add_table(table)
            .add_paragraph(Paragraph::new())
            .add_paragraph(Paragraph::new())
            .add_paragraph(Paragraph::new())
            .add_paragraph(Paragraph::new())
            .add_paragraph(Paragraph::new())
            .add_paragraph(title)
            .add_paragraph(byline)
            .add_paragraph(Paragraph::new())
            .add_paragraph(Paragraph::new());

        let mut body: Vec<Paragraph> = vec![];

        md.content.lines().for_each(|line| {
            if line.len() > 0 {
                body.push(
                    Paragraph::new()
                        .add_run(Run::new().add_text(line).size(24))
                        // .line_spacing(dbl_spacing.clone())
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
                )
            }
        });

        for p in body {
            doc = doc.add_paragraph(p);
        }

        // Build and pack the document
        doc.build().pack(file)?;

        // // Convert the Markdown to a Word document
        // let mut pandoc = pandoc::new();

        // if let Some(data_dir) = args.data_dir {
        //     println!("Setting Data Directory to {:?}", data_dir);
        //     pandoc.add_option(pandoc::PandocOption::DataDir(data_dir));
        // }

        // // pandoc.add_option(pandoc::PandocOption::Verbose);
        // // pandoc.set_show_cmdline(true);
        // // header = header.replace("$<WC>", nwc.to_string().as_str());

        // // set reference doc
        // if let Some(reference_doc) = args.reference_doc {
        //     pandoc.add_option(pandoc::PandocOption::ReferenceDoc(reference_doc));
        // }

        // // Run in standalone
        // pandoc.add_option(pandoc::PandocOption::Standalone);

        // // I think this is failing because I'm not setting that this is a latex input
        // // so using a temporary file with the right extension does the trick.
        // //
        // // use tempfile::Builder to create a temp file with the right extension
        // // let f = tempfile::Builder::new().suffix(".tex").tempfile();
        // // if let Ok(mut f) = f {
        // //     // this is working, but the output file is putting the title in the header.
        // //     println!("Created temp file {}", f.path().display());
        // //     let _ = f.write_all(latex_template.as_bytes());
        // //     let pb = PathBuf::from(f.path());
        // //     let files = vec![pb];
        // //     pandoc.set_input(pandoc::InputKind::Files(files));

        // pandoc.set_input(pandoc::InputKind::Pipe(md.content.as_str().into()));

        // // Maybe split the latex into parts and add them to pandoc this way?
        // // let header = PathBuf::from("data/header.tex");
        // // pandoc.add_option(pandoc::PandocOption::IncludeBeforeBody(header));

        // // pandoc.add_option(pandoc::PandocOption::IncludeAfterBody(()))

        // let output_dir = args
        //     .output_dir
        //     .unwrap_or_else(|| PathBuf::from("."))
        //     .into_os_string()
        //     .into_string()
        //     .unwrap();

        // pandoc.set_output(OutputKind::File(
        //     format!("{}/{}.docx", output_dir, md.metadata.title).into(),
        // ));

        // let exec = pandoc.execute();
        // if let Ok(_exec) = exec {
        //     println!("Pandoc executed successfully.");
        // } else {
        //     println!("Pandoc failed to execute: {:?}", exec.err());
        // }
    }
    Ok(())
}

fn slurp(filename: String) -> String {
    let mut input: io::BufReader<File> =
        io::BufReader::new(File::open(filename).expect("didn't work"));
    let mut md = String::new();
    input.read_to_string(&mut md).expect("can't read string");
    md
}

/// Round up to the nearest 100 or 500 (depending on length)
/// Per Bill Shunn, round up to the nearest 100 words unless you're entering novella territory,
/// in which case round up to the nearest 500 words.
/// "The point of a word count is not to tell your editor the exact length of the manuscript,
/// but approximately how much space your story will take up in the publication."
///
/// Consider the case of less than 100 words, maybe print out <100? In which case, I'd need to return this as a string
fn round_up(wc: usize) -> usize {
    let mut wc = wc;
    if wc > 17500 {
        wc += 500;
        wc -= wc % 500;
        return wc;
    }
    wc += 100;
    wc -= wc % 100;
    wc
}
