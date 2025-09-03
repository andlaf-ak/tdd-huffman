use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;
use tdd_huffman::{compress, decompress};

fn main() -> io::Result<()> {
    let matches = Command::new("huffman")
        .version("0.1.0")
        .about("Huffman compression utility")
        .arg(
            Arg::new("compress")
                .short('c')
                .long("compress")
                .help("Compress the input file")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("decompress"),
        )
        .arg(
            Arg::new("decompress")
                .short('d')
                .long("decompress")
                .help("Decompress the input file")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("compress"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file path")
                .required(false),
        )
        .arg(
            Arg::new("input")
                .value_name("INPUT")
                .help("Input file to process")
                .required(false)
                .index(1),
        )
        .get_matches();

    if matches.get_flag("compress") {
        let input_path = matches.get_one::<String>("input").ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Input file is required for compression",
            )
        })?;

        let output_path = matches.get_one::<String>("output").ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Output file (-o) is required for compression",
            )
        })?;

        compress_file(input_path, output_path)?;
    } else if matches.get_flag("decompress") {
        let input_path = matches.get_one::<String>("input").ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Input file is required for decompression",
            )
        })?;

        let output_path = matches.get_one::<String>("output").ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Output file (-o) is required for decompression",
            )
        })?;

        decompress_file(input_path, output_path)?;
    } else {
        // Show help if no arguments provided
        let mut cmd = Command::new("huffman")
            .version("0.1.0")
            .about("Huffman compression utility")
            .arg(
                Arg::new("compress")
                    .short('c')
                    .long("compress")
                    .help("Compress the input file")
                    .action(clap::ArgAction::SetTrue)
                    .conflicts_with("decompress"),
            )
            .arg(
                Arg::new("decompress")
                    .short('d')
                    .long("decompress")
                    .help("Decompress the input file")
                    .action(clap::ArgAction::SetTrue)
                    .conflicts_with("compress"),
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("FILE")
                    .help("Output file path")
                    .required(false),
            )
            .arg(
                Arg::new("input")
                    .value_name("INPUT")
                    .help("Input file to process")
                    .required(false)
                    .index(1),
            );
        cmd.print_help()?;
        println!();
    }

    Ok(())
}

fn compress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    if !Path::new(input_path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Input file '{input_path}' not found"),
        ));
    }

    let input_file = File::open(input_path)?;
    let input_reader = BufReader::new(input_file);

    let output_file = File::create(output_path)?;
    let mut output_writer = BufWriter::new(output_file);

    println!("Compressing '{input_path}' to '{output_path}'...");

    let input_size = std::fs::metadata(input_path)?.len();

    compress(input_reader, &mut output_writer)?;

    // Ensure all data is written to disk
    output_writer.flush()?;
    drop(output_writer);

    let output_size = std::fs::metadata(output_path)?.len();
    let compression_ratio = output_size as f64 / input_size as f64;

    println!("Compression completed!");
    println!("Original size: {input_size} bytes");
    println!("Compressed size: {output_size} bytes");
    println!("Compression ratio: {compression_ratio:.3}");

    Ok(())
}

fn decompress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    if !Path::new(input_path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Input file '{input_path}' not found"),
        ));
    }

    let input_file = File::open(input_path)?;
    let input_reader = BufReader::new(input_file);

    let output_file = File::create(output_path)?;
    let mut output_writer = BufWriter::new(output_file);

    println!("Decompressing '{input_path}' to '{output_path}'...");

    let input_size = std::fs::metadata(input_path)?.len();

    decompress(input_reader, &mut output_writer)?;

    // Ensure all data is written to disk
    output_writer.flush()?;
    drop(output_writer);

    let output_size = std::fs::metadata(output_path)?.len();

    println!("Decompression completed!");
    println!("Compressed size: {input_size} bytes");
    println!("Decompressed size: {output_size} bytes");

    Ok(())
}
