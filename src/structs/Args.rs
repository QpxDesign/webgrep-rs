use clap::ArgAction;
use clap::Parser;

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct ArgParser {
    #[arg(index = 1)]
    pub url: String,

    #[arg(index = 2)]
    pub search: Option<String>,

    #[arg(short, long)]
    pub recursive: Option<i8>,

    #[arg(short = 'o', long = "samehost", action=ArgAction::SetTrue)]
    pub samehost: Option<bool>,

    #[arg(short = 'p', long = "pathcontains")]
    pub pathcontains: Option<String>,

    #[arg(short = 'c', long = "chrome", action=ArgAction::SetTrue)]
    pub use_chrome: Option<bool>,
}
