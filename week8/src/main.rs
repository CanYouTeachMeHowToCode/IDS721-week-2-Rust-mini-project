/* Command-line interface for Strobogrammatic Number Determiner */
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yilun Wu",
    about = "Command-line interface for Strobogrammatic Number Determiner"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yilun Wu")]
    CheckStrobogrammatic {
        #[clap(short, long)]
        input: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::CheckStrobogrammatic { input }) => {
            let res = week8::is_strobogrammatic(input);
            println!("{res:?}");
        }
        None => {
            println!("No command given");
        }
    }
}
