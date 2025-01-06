use clap::Parser;

const KRATER_ASCII: &str = r#"
                           .      .
        krater              )    (
 _ _ _ _ _ _ _ _ _ _ _ _ _ _(.--.)
( ) ) ) ) ) ) ) ) ) ) ) ) ) ( -_-) ==~
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>->>\/"#;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, before_help = KRATER_ASCII)]
struct Cli {
    /// Email address to search
    #[arg(long)]
    email: Option<Vec<String>>,
}

fn main() {
    let cli = Cli::parse();

    // Handle emails if provided
    if let Some(ref emails) = cli.email {
        for email in emails {
            println!("Will search for email: {}", email);
        }
    }

    // Ensure at least one search parameter was provided
    if cli.email.is_none() {
        eprintln!("Error: At least one search parameter must be provided");
        std::process::exit(1);
    }
}