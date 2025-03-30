use std::fs;

use clap::{Parser, Subcommand};

mod object;
mod utils;
use object::ObjectType;

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

fn hash_object(write: &bool, file: &str) {
    let file_content = utils::file_read(file);

    if *write {
        let hash = utils::object_write(ObjectType::Blob, &file_content);
        print!("{}", hash);
    } else {
        let hash = utils::sha1_hash(&file_content);
        print!("{}", hash);
    }
}
