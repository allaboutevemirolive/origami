use std::fs::File;
use reqwest;
use std::io::Write;

fn main() {
    let url = "https://www.vpngate.net/api/iphone/";

    let response = reqwest::blocking::get(url);

    match response {
        Ok(res) => {
            if res.status().is_success() {
                // Read the response body as bytes
                let bytes = res.bytes().expect("Failed to read response body");

                // Specify the filename to save the CSV data
                let filename = "../data/vpngate_data.csv";

                // Write the bytes to a file
                let mut file = File::create(filename).expect("Failed to create file");
                file.write_all(&bytes).expect("Failed to write to file");

                println!("CSV data downloaded and saved to {}", filename);
            } else {
                println!("Error: {}", res.status());
            }
        }
        Err(err) => {
            eprintln!("Failed to fetch data: {:?}", err);
        }
    }
}
