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
    ConvertTitleToNum {
        #[clap(short, long)]
        input: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::ConvertTitleToNum { input }) => {
            let res = week6::title_to_column_number(input);
            println!("{res:?}");
        }
        None => {
            println!("No command given");
        }
    }
}
