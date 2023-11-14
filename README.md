# Rown - File downloader written in rust

Rown is a command-line tool written in Rust for downloading files with the ability to download them in parallel using multiple parts.

## Usage

`cargo run -- --url https://example.com/video --parts 2 --output out.mp4 `

### Options

- `--url`: The URL of the file to be downloaded.
- `--parts` (optional): Number of parallel parts to download. Defaults to 1 if not specified.
- `--output`: The name of the output file.

## Example

`cargo run -- --url https://example.com/video --parts 2 --output out.mp4 `

This will download the file from the specified URL in four parallel parts and save it as `out.mp4`.

## Building

To build Rown, make sure you have Rust installed, and then run:

`cargo build --release`

## Running

After building, you can run Rown with:

`./target/release/rown --url https://example.com/video --parts 2 --output out.mp4 `
