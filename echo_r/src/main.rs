use clap::{Arg, App};   // bringing in the App and Arg structs

fn main() {
    let matches = App::new("echo_r")
        .version("0.1.0")
        .author("redraincatching")
        .about("rust clone of echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("input text")
                .required(true)
                .min_values(1)
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("do not print newline")
                .takes_value(false)
        )
        .get_matches();
    // we can prefix a variable with an underscore so that the compiler knows it's not unused
    // now the echo_r program can be run with the -h or -V tags

    // so now we have two arguments, one that is optional and removes a trailing newline, and one that is required (the input text)

    println!("{:#?}", matches);
}
