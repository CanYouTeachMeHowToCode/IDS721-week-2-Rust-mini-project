/* Command-line interface for Gray Code Generator */
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yilun Wu",
    about = "Command-line interface for Gray Code Generator"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yilun Wu")]
    GenerateGrayCode {
        #[clap(short, long)]
        input: i32,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::GenerateGrayCode { input }) => {
            let res = week9::gray_code(input);
            println!("{res:?}");
        }
        None => {
            println!("No command given");
        }
    }
}
