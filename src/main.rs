extern crate clap;
extern crate url;

#[cfg(test)]
mod main_test;

use std::io;

use clap::{App, Arg, ArgGroup, ArgMatches};
use url::form_urlencoded as form;

fn main() {
    let matches = app().get_matches();
    let mut stdout = io::stdout();
    run(matches, &mut stdout);
}

pub fn run<W: io::Write>(matches: ArgMatches, out: &mut W) {
    if matches.is_present("decode") {
        for input in matches.values_of("INPUT").unwrap() {
            decode(input.as_bytes(), out);
        }
    } else if matches.is_present("encode") {
        for input in matches.values_of("INPUT").unwrap() {
            encode(input.as_bytes(), out);
        }
    }
}

fn decode<W: io::Write>(input: &[u8], out: &mut W) {
    for (left, right) in form::parse(input) {
        if right.is_empty() {
            writeln!(out, "{}", left).unwrap();
        } else {
            writeln!(out, "{}={}", left, right).unwrap();
        }
    }
}

fn encode<W: io::Write>(input: &[u8], out: &mut W) {
    for encoded in form::byte_serialize(input) {
        write!(out, "{}", encoded).unwrap();
    }
    writeln!(out, "").unwrap();
}

pub fn app() -> App<'static, 'static> {
    App::new("urlq")
        .version("0.1.0")
        .author("James Hall")
        .about("Encode and decode URL queries")
        .arg(Arg::with_name("decode")
            .short("d")
            .long("decode")
            .help("Decode the URL query input"))
        .arg(Arg::with_name("encode")
            .short("e")
            .long("encode")
            .help("Encode the input into a URL query"))
        .group(ArgGroup::with_name("transform")
            .args(&["decode", "encode"])
            .required(true))
        .arg(Arg::with_name("INPUT")
            .multiple(true)
            .required(true))
}
