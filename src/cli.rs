use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub url: String,

    #[arg(short, long, default_value = "1")]
    pub parts: usize,

    #[arg(short, long)]
    pub output: String,
}

pub fn parse_arguments() -> Args {
    let args = Args::parse();
    return args;
}
