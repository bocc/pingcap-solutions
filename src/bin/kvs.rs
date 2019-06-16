#[macro_use]
extern crate clap;

use clap::{Arg, App, SubCommand, AppSettings};

// not nice, only for the purposes of this project
enum ArgsRecieved {
    Get { key: String },
    Set { key: String, value: String },
    Rm  { key: String }
}

fn main() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let matches = App::new(crate_name!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .about("Command line key-value store")
        .arg(Arg::with_name("version")
            .short("V")
            .help("Displays current version"))

        .subcommand(SubCommand::with_name("get")
            .arg(Arg::with_name("key")
                .index(1)
                .required(true)))

       .subcommand(SubCommand::with_name("set")
            .arg(Arg::with_name("key")
                .index(1)
                .required(true))
            .arg(Arg::with_name("value")
                .index(2)
                .required(true)))

        .subcommand(SubCommand::with_name("rm")
            .arg(Arg::with_name("key")
                .index(1)
                .required(true)))
        .get_matches();
    
    if matches.is_present("version") {
        println!("{}", VERSION);
    } else {
        let args: ArgsRecieved = match matches.subcommand() {
            ("get", Some(matches)) => {
                // println!("key: {}", matches.value_of("key").unwrap());

                ArgsRecieved::Get { 
                    key: matches.value_of("key").unwrap().to_string()
                }
            },
            ("set", Some(matches)) => {
                // println!("key: {}", matches.value_of("key").unwrap());
                // println!("value: {}", matches.value_of("value").unwrap());

                ArgsRecieved::Set { 
                    key: matches.value_of("key").unwrap().to_string(),
                    value: matches.value_of("value").unwrap().to_string()
                }
            },
            ("rm",  Some(matches)) => {
                // println!("key: {}", matches.value_of("key").unwrap());

                ArgsRecieved::Rm { 
                    key: matches.value_of("key").unwrap().to_string()
                }
            },
            _                      => std::process::exit(0),
        };
    }

}
