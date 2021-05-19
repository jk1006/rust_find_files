use regex::Regex;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Rust Find CLI", about = "CLI structure for rust find utility")]
struct CLI {
    #[structopt(short, long = "--dirs", help = "List of directories to search in.")]
    dirs: Vec<String>,
    #[structopt(short, long = "--patterns", help = "List of patterns to use")]
    patterns: Vec<String>,
    #[structopt(
        short,
        long = "--output",
        help = "Write results to an ouput file instead of stdout"
    )]
    output: Option<String>,
    #[structopt(
        short,
        long = "--size",
        help = "Match files above size",
        default_value = "0"
    )]
    size: usize,
}

pub struct MyFile {
    name: String,
    dir_in: String,
    size_bytes: u64,
}

impl MyFile {
    pub fn new(name: String, path: String, size: u64) -> Self {
        Self {
            name,
            dir_in: path,
            size_bytes: size,
        }
    }
}

fn main() -> Result<(), std::ffi::OsString> {
    let args = CLI::from_args();
    let mut files = files::read_files(&args.dirs)?;
    let mut patterns: Vec<Regex> = Vec::new();
    for pattern in &args.patterns {
        if let Ok(reg) = Regex::new(pattern) {
            patterns.push(reg);
        }
    }
    if patterns.len() > 0 {
        filter::filter_files(&mut files, patterns, &args.size);
    }
    for file in &files {
        println!("{:?}, {:?} KB", file.dir_in,  file.size_bytes);
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    CrosstermError(#[from] std::io::Error),
}

mod files {
    use super::MyFile;
    use std::path::Path;

    use std::fs::{self, DirEntry};
    pub fn read_files(dirs: &Vec<String>) -> Result<Vec<MyFile>, std::ffi::OsString> {
        let mut result: Vec<MyFile> = Vec::new();
        let mut dir_entries: Vec<DirEntry> = Vec::new();
        for dir in dirs {
            let _ = scan_dir(Path::new(dir), &mut dir_entries);
        }
        for file in dir_entries {
            let name = file.file_name().into_string()?;
            let path = file.path().into_os_string().into_string()?;
            let size = file.metadata().unwrap().len();
            result.push(MyFile::new(name, path, size));
        }
        Ok(result)
    }

    fn scan_dir(dir: &Path, files: &mut Vec<DirEntry>) -> Result<(), std::io::Error> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    let _ = scan_dir(&path, files);
                } else {
                    files.push(entry);
                }
            }
        }
        Ok(())
    }
}

mod filter {
    use super::MyFile;
    pub fn filter_files(files: &mut Vec<MyFile>, patterns: Vec<regex::Regex>, min_size: &usize) {
        for pattern in patterns {
            files.retain(|file| pattern.is_match(&file.name) && file.size_bytes as usize > *min_size);
        }
    }
}
