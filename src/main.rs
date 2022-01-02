use clap::{arg, App, AppSettings};

fn main() {
    let matches = App::new("converter")
        .about("Converter CLI")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
        .subcommand(
            App::new("base64")
                .about("Convert a string to base64")
                .arg(arg!(<STRING> "The string to convert to base64"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("base64", sub_matches)) => {
            println!(
                "Converting {}",
                sub_matches.value_of("STRING").expect("required")
            );
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