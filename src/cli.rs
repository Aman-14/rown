use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub url: String,

    #[arg(short, long)]
    pub parts: Option<usize>,

    #[arg(short, long)]
    pub output: String,
}

pub fn parse_arguments() -> Args {
    let args = Args::parse();
    return args;
}
