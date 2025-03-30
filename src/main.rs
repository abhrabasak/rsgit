use std::fs;

use clap::{Parser, Subcommand};

mod utils;
use utils::ObjectType;

#[derive(Parser, Debug)]
struct Cli {
    /// The command to run
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init,
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,
        /// The hash of the object to display
        hash: String,
    },
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Command::Init => init(),
        Command::CatFile { pretty_print, hash } => cat_file(pretty_print, hash),
    }
}

fn init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized git directory");
}

fn cat_file(pretty_print: &bool, hash: &str) {
    let object_path = utils::object_hash_to_path(hash);
    let decoded_content = utils::zlib_decode_file_to_string(&object_path);

    let (object_type, content) = utils::object_parse(&decoded_content);

    if *pretty_print {
        match object_type {
            ObjectType::Blob => print!("{}", content),
            _ => println!("Other object type: {:?}", object_type),
        }
    }
}
