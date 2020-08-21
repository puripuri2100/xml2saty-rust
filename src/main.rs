extern crate clap;

use clap::{App, Arg};
//use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};

pub mod config;
pub mod xmlparse;
//pub mod parser;

fn write_file(file_name: String, text: String) -> () {
  let mut file = File::create(file_name).unwrap();
  file.write_all(text.as_bytes()).unwrap();
}

fn main() {
  let app = App::new("xml2saty-rust")
    .version("0.0.1")
    .arg(
      Arg::with_name("file")
        .help("Specify XML file")
        .short("f")
        .long("file")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("text")
        .help("Give XML text")
        .short("t")
        .long("text")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("output")
        .help("Specify output file")
        .short("o")
        .long("output")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("config")
        .help("Specify config file")
        .short("c")
        .long("config")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("package")
        .help("Output as package file")
        .short("p")
        .long("package")
        .takes_value(true),
    );

  let matches = app.get_matches();

  let mut file_name = String::new();
  //  let mut xml_text = String::new();
  let mut output_name = String::new();
  let mut config_name = String::new();
  let mut package_name: Option<&str> = None;

  if let Some(file) = matches.value_of("file") {
    file_name = file.to_string();
    //    let text = fs::read_to_string(file).unwrap();
    //    xml_text = text;
    print!("Value for file: {}\n", file);
  }

  if let Some(text) = matches.value_of("text") {
    //    xml_text = text.to_string();
    print!("Value for text: {}\n", text);
  }

  if let Some(output) = matches.value_of("output") {
    output_name = output.to_string();
    print!("Value for output: {}\n", output);
  }

  if let Some(config) = matches.value_of("config") {
    config_name = config.to_string();
    print!("Value for config: {}\n", config);
  }

  if let Some(package) = matches.value_of("package") {
    package_name = Some(package);
    print!("Value for package: {}\n", package);
  }

  let file_xml_text = BufReader::new(File::open(&mut file_name).unwrap());

  let config_data = config::parse(&mut config_name);
  let satysfi_text_raw = xmlparse::xml2string(file_xml_text, &config_data);
  let satysfi_text = config::package(&mut package_name, satysfi_text_raw);
  let header_text = config::header(config_data);

  let _ = write_file(output_name, format!("{}\n{}", header_text, satysfi_text));
}
