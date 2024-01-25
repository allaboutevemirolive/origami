use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use std::{
    self,
    fs::{self, File},
    io::Write,
};

fn main() {
    let read_f: String = String::from_utf8_lossy(&fs::read("../data/base64.txt").unwrap())
        .parse()
        .unwrap();

    // let bytes = general_purpose::STANDARD
    //     .decode(read_f)
    //     .unwrap();
    // println!("{:?}", bytes);

    let bytes = general_purpose::STANDARD.decode(read_f).unwrap();
    // Convert the decoded bytes to a UTF-8 string
    if let Ok(decoded_str) = String::from_utf8(bytes) {
        // println!("{}", decoded_str);

        let output = "../data/base64_decode.txt";

        let mut output_writer = File::create(output).unwrap();

        output_writer.write_all(&decoded_str.into_bytes()).unwrap();
    } else {
        println!("Failed to convert decoded bytes to UTF-8 string");
    }

    let bytes_url = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
        .decode("aGVsbG8gaW50ZXJuZXR-Cg")
        .unwrap();
    // println!("{:?}", bytes_url);

    if let Ok(decoded_str_url) = String::from_utf8(bytes_url) {
        println!("{}", decoded_str_url);
    } else {
        println!("Failed to convert decoded bytes to UTF-8 string");
    }
}
