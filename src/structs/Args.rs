use clap::ArgAction;
use clap::Parser;

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct ArgParser {
    #[arg(short = 'u', long = "url")]
    pub url: String,

    #[arg(short = 's', long = "search")]
    pub search: Option<String>,

    #[arg(short, long, action=ArgAction::SetTrue)]
    pub recursive: Option<bool>,
}
