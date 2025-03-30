use std::fs;

use clap::{Parser, Subcommand};

mod errors;
mod object;
mod utils;
use object::{GitObject, ObjectType};

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
    HashObject {
        #[clap(short = 'w')]
        write: bool,
        /// The file to hash
        file: String,
    },
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Command::Init => init(),
        Command::CatFile { pretty_print, hash } => cat_file(pretty_print, hash),
        Command::HashObject { write, file } => hash_object(write, file),
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
    let go = GitObject::load(hash).unwrap();
    if *pretty_print {
        go.cat();
    }
}

fn hash_object(write: &bool, file: &str) {
    let go = GitObject::create(ObjectType::Blob, file);
    let hash = if *write { go.store() } else { go.hash() };
    print!("{}", hash);
}
