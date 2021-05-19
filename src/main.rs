use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Rust Find CLI", about = "CLI structure for rust find utility")]
struct CLI {
    #[structopt(short, long = "--dirs", help = "List of directories to search in.")]
    dirs: Vec<String>,
    #[structopt(short, long = "--patterns", help = "List of patterns to use")]
    patterns: Vec<String>,
    #[structopt(short, long = "--output", help = "Write results to an ouput file instead of stdout")]
    output: Option<String>,
    #[structopt(short, long = "--size", help = "Match files above size", default_value = "0")]
    size: usize,
}

pub struct MyFile {
    name: String,
    dir_in: String,
    size_bytes: u64,
}

impl MyFile {
    pub fn new(name: String, path: String) -> Self {
        Self {
            name,
            dir_in: path,
            size_bytes: 0,
        }
    }
}

fn main() {
   let args = CLI::from_args(); 
   let files = files::read_files(&args.dirs);
   for file in files {
       println!("{:?}", file.name);
   }
}

mod files {
    use super::MyFile;
    use std::path::Path;
    use std::fs::{self, DirEntry};
    pub fn read_files(dirs: &Vec<String>) -> Vec<MyFile>{
        let mut result: Vec<MyFile> = Vec::new();
        let mut dir_entries: Vec<DirEntry> = Vec::new();
        for dir in dirs {
            let _ = scan_dir(Path::new(dir), & mut dir_entries);
        }
        for file in dir_entries {
            result.push(MyFile::new(file.file_name().into_string().unwrap(), file.path().into_os_string().into_string().unwrap()));
        }
        result
    }

    fn scan_dir(dir: &Path, files: & mut Vec<DirEntry>) {
        if dir.is_dir() {
            for entry in fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    scan_dir(&path, files);
                } else {
                   files.push(entry);
                }
            }
        } 

    }
} 
