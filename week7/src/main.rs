/* Command-line interface for EXCEL Title Converter */
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yilun Wu",
    about = "Command-line interface for EXCEL Title Converter"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yilun Wu")]
    FindAnagrams {
        #[clap(short, long)]
        input: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::FindAnagrams { input }) => {
            let res = week7::find_anagrams(input);
            println!("{res:?}");
        }
        None => {
            println!("No command given");
        }
    }
}
