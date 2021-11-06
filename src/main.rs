use std::fs::File;
use std::io::prelude::*;

use clap::App;

fn main() {
    // Parse CLI arguments.
    let yaml = clap::load_yaml!("clap.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Handle encoding.
    if let Some(matches) = matches.subcommand_matches("encode") {
        // Keep some statistics.
        let mut bytes_read: usize = 0;
        let mut bytes_written: usize = 0;

        let input_file = matches.value_of("INPUT").unwrap();
        let output_file = matches.value_of("OUTPUT").unwrap();

        // Read the input file.
        let mut input = File::open(input_file).unwrap();
        let mut data: Vec<u8> = Vec::new();
        match input.read_to_end(&mut data) {
            Err(why) => panic!("Cannot read: {}", why),
            Ok(n) => bytes_read += n,
        }

        let encoded = encode(data);

        // Write to the output file.
        let mut output = File::create(output_file).unwrap();
        match output.write_all(&encoded) {
            Err(why) => panic!("Cannot write: {}", why),
            Ok(_) => bytes_written += encoded.len(),
        }

        if matches.is_present("verbose") {
            let compression_ratio = 1.0 - (bytes_written as f64 / bytes_read as f64);
            eprintln!("Uncompressed file size: {}", bytes_read);
            eprintln!("Compressed file size: {}", bytes_written);
            eprintln!("Compression ratio: {:.2}%", compression_ratio * 100.0);
        }
    }

    // Handle decoding.
    if let Some(matches) = matches.subcommand_matches("decode") {
        let input_file = matches.value_of("INPUT").unwrap();
        let output_file = matches.value_of("OUTPUT").unwrap();

        let mut input = File::open(input_file).unwrap();
        let mut data: Vec<u8> = Vec::new();
        match input.read_to_end(&mut data) {
            Err(why) => panic!("Cannot read: {}", why),
            Ok(_) => (),
        }

        let decoded = decode(data);

        let mut output = File::create(output_file).unwrap();
        match output.write_all(&decoded) {
            Err(why) => panic!("Cannot write: {}", why),
            Ok(_) => (),
        }
    }
}

/// Encode data using run-length encoding.
fn encode(data: Vec<u8>) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut encoded: Vec<u8> = Vec::new();
    let mut previous: u8 = data[0];
    let mut len: u8 = 0;

    for current in data {
        if current == previous && len < u8::MAX {
            len += 1;
        } else {
            encoded.push(previous);

            if len > 1 {
                // Push the previous symbol a second time to indicate a repetition.
                encoded.push(previous);
                encoded.push(len - 2);
            }

            previous = current;
            len = 1;
        }
    }

    // Handle the last symbol.
    encoded.push(previous);
    if len > 1 {
        // Push the previous symbol a second time to indicate a repitition.
        encoded.push(previous);
        encoded.push(len - 2);
    }

    return encoded;
}

/// Decode data encoded with run-length encoding.
fn decode(data: Vec<u8>) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut decoded: Vec<u8> = Vec::new();

    // Let `previous` be some value **not equal** to data[0].
    let mut previous: u8 = if data[0] < u8::MAX {
        data[0] + 1
    } else {
        data[0] - 1
    };

    let mut i: usize = 0;
    while i < data.len() {
        let current = data[i];

        if current == previous {
            // A repetition is found! The next byte contains the number of repetitions.
            i += 1;
            let len = data[i];

            for _ in 0..len {
                decoded.push(previous);
            }
        }

        decoded.push(current);

        previous = data[i];
        i += 1;
    }

    return decoded;
}
