use clap::{App, AppSettings};

extern crate base64;

fn main() {
    let base64_app = converter::converter_base64::cli::add_commands();

    let matches = App::new("converter")
        .about("Converter CLI")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
        .subcommand(
            base64_app,
        )
        .get_matches();

    match matches.subcommand() {
        Some(("base64", sub_matches)) => {
            converter::converter_base64::cli::execute(sub_matches);
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