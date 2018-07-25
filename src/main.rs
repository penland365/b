extern crate clap;
#[macro_use]
extern crate version;

mod b64;
mod alphabets;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("b")
        .arg_from_usage("--license 'display the license file'")
        .author("Jeffrey Davis https://github.com/penland365")
        .version(version!())
        .subcommand(SubCommand::with_name("64")
            .about("Base 64 transcoding")
            .author("Jeffrey Davis penland365@gmail.com")
            .version(version!())
            .subcommand(SubCommand::with_name("encode")
                            .about("Base 64 encoding")
                            .author("Jeffrey Davis penland365@gmail.com")
                            .version(version!())
                            .arg(Arg::with_name("input")
                                    .takes_value(true)))
            .subcommand(SubCommand::with_name("decode")
                            .about("Base 64 decoding")
                            .author("Jeffrey Davis penland365@gmail.com")
                            .version(version!())
                            .arg(Arg::with_name("input")
                                    .takes_value(true))))
        .get_matches();

    match matches.subcommand() {
        ("64", Some(b64_matches)) => {
            match b64_matches.subcommand() {
                ("encode", Some(x)) => b64::handle_encoding(x),
                ("decode", Some(x)) => b64::handle_decoding(x),
                (_, _)     => unreachable!()
            }
        },
        (_, _) => println!("64 is all that is implemented right now")
    }
}
