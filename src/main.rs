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

struct MyFile {
    name: String,
    dir_in: String,
    size_bytes: u64,
}

fn main() {
   let args = CLI::from_args(); 
   for name in &args.dirs {
       println!("{}", name);
   }
}
