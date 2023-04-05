/* Command-line interface for Invalid Parentheses Remover */
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yilun Wu",
    about = "Command-line interface for Invalid Parentheses Remover"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yilun Wu")]
    RemoveInvalidParentheses {
        #[clap(short, long)]
        input: String,
    },
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_remove_invalid_parentheses() {
        assert_eq!(
            week10::remove_invalid_parentheses("()())()".to_string()),
            vec!["(())()".to_owned(), "()()()".to_owned()]
        );
    }

    #[test]
    fn test_remove_invalid_parentheses_02() {
        assert_eq!(
            week10::remove_invalid_parentheses("(a)())()".to_string()),
            vec!["(a())()".to_owned(), "(a)()()".to_owned()]
        );
    }

    #[test]
    fn test_remove_invalid_parentheses_03() {
        assert_eq!(
            week10::remove_invalid_parentheses(")(".to_string()),
            vec!["".to_owned()]
        );
    }
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::RemoveInvalidParentheses { input }) => {
            let res = week10::remove_invalid_parentheses(input);
            println!("{res:#?}");
        }
        None => {
            println!("No command given");
        }
    }
}
