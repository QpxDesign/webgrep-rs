use clap::ArgAction;
use clap::Parser;

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct ArgParser {
    #[arg(short = 'u', long = "url")]
    pub url: String,

    #[arg(short = 's', long = "search")]
    pub search: Option<String>,

    #[arg(short, long)]
    pub recursive: Option<i8>,

    #[arg(short = 'o', long = "samehost", action=ArgAction::SetTrue)]
    pub samehost: Option<bool>,

    #[arg(short = 'p', long = "pathcontains")]
    pub pathcontains: Option<String>,
}
