use clap::{Parser, Subcommand};

mod cmd;
mod errors;
mod object;
mod utils;
use cmd::{cat_file::cat_file_exec, hash_object::hash_object_exec, init::init_exec};

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
        Command::Init => init_exec(),
        Command::CatFile { pretty_print, hash } => cat_file_exec(pretty_print, hash),
        Command::HashObject { write, file } => hash_object_exec(write, file),
    }
}
