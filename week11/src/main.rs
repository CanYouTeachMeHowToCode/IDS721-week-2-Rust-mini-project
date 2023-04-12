/* Command-line interface for Unix Path Simplifier */
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yilun Wu",
    about = "Command-line interface for Unix Path Simplifier"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yilun Wu")]
    SimplifyPath {
        #[clap(short, long)]
        input: String,
    },
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_simplify_path() {
        assert_eq!(
            week11::simplify_path("/home/".to_string()),
            "/home".to_owned()
        );
    }

    #[test]
    fn test_simplify_path_02() {
        assert_eq!(week11::simplify_path("/../".to_string()), "/".to_owned());
    }

    #[test]
    fn test_simplify_path_03() {
        assert_eq!(
            week11::simplify_path("/home//rust/".to_string()),
            "/home/rust".to_owned()
        );
    }
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::SimplifyPath { input }) => {
            let res = week11::simplify_path(input);
            println!("{res:#?}");
        }
        None => {
            println!("No command given");
        }
    }
}
