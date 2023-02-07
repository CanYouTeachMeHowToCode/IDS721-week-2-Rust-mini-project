/*Command-line interface for Basic Calculator */
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yilun Wu",
    about = "Command-line interface for BasicCalculator"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yilun Wu")]
    BasicCalculator {
        #[clap(short, long)]
        input: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::BasicCalculator { input }) => {
            let res = project::calculate(input);
            println!("{res}");
        }
        None => {
            println!("No command given");
        }
    }
}
