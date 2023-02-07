/*Command-line interface for Basic Calculator */
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yilun Wu",
    about = "Command-line interface for Integer to Roman Converter"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yilun Wu")]
    IntegerToRoman {
        #[clap(short, long)]
        input: i32,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::IntegerToRoman { input }) => {
            let res = project::int_to_roman(input);
            println!("{res}");
        }
        None => {
            println!("No command given");
        }
    }
}
