use clap;
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let cli = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(format!("{} <bttrswt@protonmail.com>", env!("CARGO_PKG_AUTHORS")).as_str())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(clap::Arg::with_name("input")
            .short('i')
            .long("input")
            .value_name("INPUT FILE")
            .takes_value(true)
            .help("Sets the input file to use")
            .required(true))
        .arg(clap::Arg::with_name("output")
            .short('o')
            .long("output")
            .value_name("OUTPUT FILE")
            .takes_value(true)
            .help("Sets the output file to use")
            .required(true))
        .arg(clap::Arg::with_name("namespace")
            .short('n')
            .long("namespace")
            .value_name("NAMESPACE")
            .takes_value(true)
            .help("Namespace wrapper")
            .required(false))
        .arg(clap::Arg::with_name("variable_name")
            .short('v')
            .long("variable")
            .value_name("VARIABLE NAME")
            .takes_value(true)
            .help("VARIABLE NAME")
            .required(false))
        .get_matches();

    let input_file = std::fs::read(cli.value_of("input").unwrap())?;
    let mut output_file = std::fs::File::create(cli.value_of("output").unwrap())?;

    output_file.write(b"#include <array>\n\n")?;
    
    if let Some(namespace) = cli.value_of("namespace") {
        output_file.write(&format!("namespace {} {{\n", namespace).into_bytes())?;
    }

    if let Some(variable_name) = cli.value_of("variable_name") {
        output_file.write(&format!("\tconstexpr std::array<unsigned char, {}> {} = {{\n\t", input_file.len(), variable_name).into_bytes())?;
    }
    else {
        output_file.write(&format!("\tconstexpr std::array<unsigned char, {}> {} = {{\n\t", input_file.len(), std::path::Path::new(&cli.value_of("input").unwrap().to_lowercase()).file_stem().unwrap().to_str().unwrap()).into_bytes())?;
    }
    
    for chunk in input_file.chunks(16) {
        for elem in chunk.iter() {
            output_file.write(&format!("\t0x{:02x},", elem).into_bytes())?;
        }
        output_file.write(b"\n\t")?;
    }

    output_file.write(b"};\n")?;

    if let Some(_) = cli.value_of("namespace") {
        output_file.write(b"}")?;
    } 

    Ok(())
}
