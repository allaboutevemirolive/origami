use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};

fn main() {
    let input_path = "../data/vpngate_data.csv";

    let output = "../data/output.csv";

    let mut output_writer: Box<dyn Write> = Box::new(BufWriter::new(File::create(output).unwrap()));

    let input_file = File::open(input_path).expect("Failed to open input file");

    let mut input_reader = BufReader::new(input_file);

    parser::data_iter(&mut input_reader, &mut output_writer);
}
