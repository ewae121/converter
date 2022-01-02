use clap::{arg, Arg, App, AppSettings};
use std::str;

extern crate base64;

fn to_base64(to_convert: &str) {
    println!("Converting {}", to_convert);
    let b64 = base64::encode(to_convert);
    println!("{}", b64);
}

fn from_base64(to_convert: &str) {
    println!("Converting {}", to_convert);
    let decoded = base64::decode(to_convert).unwrap();

    let s = match str::from_utf8(&decoded) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("{}", s);
}

fn main() {
    let matches = App::new("converter")
        .about("Converter CLI")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
        .subcommand(
            App::new("base64")
                .about("Convert a string to base64")
                .arg(Arg::new("encode")
                            .short('e')
                            .long("encode")
                            .help("Encode the provided string to base64")
                            .conflicts_with("decode")
                )
                .arg(Arg::new("decode")
                            .short('d')
                            .long("decode")
                            .help("Decode the provided base64")
                            .conflicts_with("encode")
                )
                .arg(arg!(<STRING> "The string to encode or decode"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("base64", sub_matches)) => {
            let value = sub_matches.value_of("STRING").expect("required");
            if sub_matches.is_present("decode") {
                from_base64(value);
            } else {
                to_base64(value);
            }
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .values_of_os("")
                .unwrap_or_default()
                .collect::<Vec<_>>();
            println!("Calling out to {:?} with {:?}", ext, args);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    // Continued program logic goes here...
}