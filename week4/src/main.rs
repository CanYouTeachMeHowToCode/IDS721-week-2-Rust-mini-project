/*Command-line interface for Longest Parlindromic Substring Finder */
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yilun Wu",
    about = "Command-line interface for Longest Parlindromic Substring Finder"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yilun Wu")]
    LongestParlindromicSubstring {
        #[clap(short, long)]
        input: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::LongestParlindromicSubstring { input }) => {
            let res = week4::longest_palindromic_substring(&input);
            println!("{res}");
        }
        None => {
            println!("No command given");
        }
    }
}
