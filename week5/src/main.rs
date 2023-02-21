/* Command-line interface for Longest IP Address Restorer */
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yilun Wu",
    about = "Command-line interface for Longest IP Address Restorer"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yilun Wu")]
    RestoreIPAddresses {
        #[clap(short, long)]
        input: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::RestoreIPAddresses { input }) => {
            let res = week5::restore_ip_addresses(input);
            println!("{res:#?}");
        }
        None => {
            println!("No command given");
        }
    }
}
