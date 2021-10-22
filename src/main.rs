use clap::{App, Arg};
use facto::Factoring;

mod verbosity_tracker;

fn factor_silent(n: facto::Integer) {
    let f = n.clone().factor();
    let f_str = f
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}: {}", n, f_str);
}

fn factor_verbose(n: facto::Integer, historic: bool) {
    let v = verbosity_tracker::VerboseFactoring::new(n.clone(), historic);
    let f = n.clone().factor_events(v);
    let f_str = f
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}: {}", n, f_str);
}

fn main() {
    let matches = App::new("facto")
        .version("0.1.0")
        .author("Lukas Wölfer <lukas.woelfer@rwth-aachen.de>")
        .about("Integer factorization")
        .arg(
            Arg::with_name("input")
                .value_name("INPUT")
                .help("Numbers to factor")
                .multiple(true)
                .required(true),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Output factors as soon as they are detected"),
        )
        .get_matches();

    for p in matches.values_of("input").unwrap() {
        match facto::Integer::parse(p.clone()) {
            Ok(p) => match matches.occurrences_of("verbosity") {
                0 => factor_silent(p.into()),
                1 => factor_verbose(p.into(), false),
                2 => factor_verbose(p.into(), true),
                3.. => println!("Verbose level too high"),
            },
            Err(_) => println!("Could not parse {}", p),
        }
    }
}
