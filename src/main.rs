use clap::{App, Arg};
use thunderbird_email_backup::run;
use thunderbird_email_backup::runtime::Operation;

fn main() {
    let matches = App::new("Thunderbird email backup")
        .version("0.1")
        .author("Richard Bradshaw")
        .about("Easily back up the email in a thunderbird profile folder")
        .arg(
            Arg::with_name("profile")
                .long("profile")
                .takes_value(true)
                .required(true)
                .help("The thunderbird profile to use. Use 'ls ~/.thunderbird' to find this"),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .takes_value(true)
                .required(true)
                .help("The path to output to"),
        )
        .get_matches();
    let profile = matches.value_of("profile").unwrap();
    let output = matches.value_of("output").unwrap();
    let operation = Operation::Backup(profile, output);
    match run(operation) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("An error occurred: {}", error);
            panic!();
        }
    }
}
