use download::DownloadArgs;
use reqwest::blocking::Client;
mod cli;
mod download;

fn main() {
    let args = cli::parse_arguments();
    let client = Client::new();

    download::download(DownloadArgs {
        client,
        url: args.url,
        file_name: args.output,
        parts: args.parts,
    });
}
