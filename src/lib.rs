pub mod converter_base64 {
    pub mod cli {
        use clap::{arg, Arg, App, AppSettings};

        enum Base64Action {
            Encode,
            Decode
        }

        pub fn add_commands() -> App<'static> {
            let base64 = App::new("base64")
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
                .setting(AppSettings::ArgRequiredElseHelp);
            base64
        }

        pub fn execute (sub_matches : &clap::ArgMatches) {
            let value = sub_matches.value_of("STRING").expect("required");

            let mut action = Base64Action::Encode;
            if sub_matches.is_present("decode") {
                action = Base64Action::Decode;    
            }

            match action {
                Base64Action::Decode => {
                    // from_base64(value);
                    let result = super::actions::decode(value);
                    println!("{}", result);
                },
                Base64Action::Encode => {
                    // to_base64(value);
                    let result = super::actions::encode(value);
                    println!("{}", result);
                }
            }
        }
    }

    pub mod actions {
        extern crate base64;

        use std::str;

        pub fn encode(value: &str) -> String {
            // println!("Converting {}", value);
            let b64 = base64::encode(value);
            b64
        }
        pub fn decode(value : &str) -> String {
            // println!("Converting {}", value);
            let decoded = base64::decode(value).unwrap();

            let s = match str::from_utf8(&decoded) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            let s = String::from(s);
            s
        }
    }
}