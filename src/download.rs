use std::{
    fs::File,
    io::{self, Seek},
    sync::{Arc, Mutex},
    thread,
};

use reqwest::{blocking::Client, header::CONTENT_LENGTH};

pub struct DownloadArgs {
    pub client: Client,
    pub url: String,
    pub parts: Option<usize>,
    pub file_name: String,
}

fn get_content_length(client: &Client, url: &str) -> Option<usize> {
    let res = client
        .head(url)
        .send()
        .expect("Something went wrong with head URL");

    let content_length: Option<usize> = res
        .headers()
        .get(CONTENT_LENGTH)
        .and_then(|x| x.to_str().ok())
        .and_then(|x| x.parse().ok());

    return content_length;
}

pub fn download(
    DownloadArgs {
        client,
        url,
        parts,
        file_name,
    }: DownloadArgs,
) {
    // pub fn download(client: Client, url: &str, parts: Option<u32>) {
    let cl = get_content_length(&client, &url)
        .expect("Failed to retrieve content length from the server");

    println!("Content-length - {}", cl);

    let parts = parts.unwrap_or(1);
    let delta: usize = cl / parts;

    let temp_files = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for i in 0..parts {
        let idx = i.clone();
        let client = client.clone();
        let url = url.to_string();
        let temp_files = Arc::clone(&temp_files);

        let handle = thread::spawn(move || {
            let range_start = idx * delta;

            let range_end = if idx == (parts - 1) {
                cl - 1 // 0 to length - 1
            } else {
                (i + 1) * delta - 1
            };

            let mut tmpfile = tempfile::tempfile().expect("Cannot create temp file");
            client
                .get(url)
                .header("range", format!("bytes={}-{}", range_start, range_end))
                .send()
                .expect("Failed to send request")
                .copy_to(&mut tmpfile)
                .expect("Failed to copy to write");

            // tmpfile.write_all(&contents).ok();
            let mut temp_files = temp_files
                .lock()
                .expect("Unable to lock temp files inside theads");
            temp_files.push((tmpfile, i));
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().expect("Failed to join thread");
    }

    let mut temp_files = temp_files
        .lock()
        .expect("Unable to lock temp files in main thread");

    temp_files.sort_by_key(|(_, i)| *i);

    println!("{:?}", temp_files);

    let mut writer = File::create(&file_name).expect("Cannot create file");
    for (file, _) in temp_files.iter() {
        let mut file = file;
        file.seek(io::SeekFrom::Start(0))
            .expect("Cannot seek the file");
        io::copy(&mut file, &mut writer).expect("Cannot write to main file");
    }

    println!("Download completed. Final file: {}", file_name);
}
