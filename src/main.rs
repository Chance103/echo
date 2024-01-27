use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    input: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    if let Some(input) = cli.input {
        println!("{input}");
    }
}
