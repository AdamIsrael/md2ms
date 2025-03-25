use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

/// Round up to the nearest 100 or 500 (depending on length)
/// Per Bill Shunn, round up to the nearest 100 words unless you're entering novella territory,
/// in which case round up to the nearest 500 words.
/// "The point of a word count is not to tell your editor the exact length of the manuscript,
/// but approximately how much space your story will take up in the publication."
///
/// Consider the case of less than 100 words, maybe print out <100? In which case, I'd need to return this as a string
pub fn round_up(wc: usize) -> usize {
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

/// Read in the contents of the file to a String
pub fn slurp(filename: String) -> String {
    let mut input: io::BufReader<File> =
        io::BufReader::new(File::open(filename).expect("didn't work"));
    let mut md = String::new();
    input.read_to_string(&mut md).expect("can't read string");
    md
}

/// Get the filename relative to the base directory.
pub fn get_base_filename(basedir: String, path: String) -> String {
    let relative = path
        .replace(basedir.as_str(), "")
        .trim_start_matches('/')
        .to_string();
    relative
}

/// It takes a filename and returns the directory where the file is located.
pub fn get_file_basedir(file: String) -> String {
    let pb = PathBuf::from(file);
    for ancestor in pb.ancestors() {
        if ancestor.is_dir() {
            return ancestor.as_os_str().to_str().unwrap().to_string();
        }
    }
    return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_base_filename_self() {
        let path = "examples/";
        let base = get_base_filename("examples/".to_string(), path.to_string());
        assert_eq!(base, "");
    }
    #[test]
    fn test_get_base_filename_single_file() {
        let path = "examples/standalone.md";
        let base = get_base_filename("examples/".to_string(), path.to_string());
        assert_eq!(base, "standalone.md");
    }

    #[test]
    fn test_get_base_filename_nested() {
        let path = "examples/novella_with_parts/Act 1/Chapter 1/scene1.md";
        let base = get_base_filename("examples/novella_with_parts/".to_string(), path.to_string());
        assert_eq!(base, "Act 1/Chapter 1/scene1.md");
    }

    #[test]
    fn test_get_file_basedir() {
        let path = "examples/novella_with_parts/Act 1/Chapter 1/scene1.md";
        let basedir = get_file_basedir(path.to_string());
        assert_eq!(basedir, "examples/novella_with_parts/Act 1/Chapter 1");
    }
}
