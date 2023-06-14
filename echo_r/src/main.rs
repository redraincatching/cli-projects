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

    // so we've got arguments, how do we extract them from matches?
    // since text was defined to accept multiple arguments, we can use either of these:
        // - ArgMatches::values_of, which returns Option<Values>
        // - ArgMatches::values_of_lossy, which returns Option<Vec<String>>
    // we'll use the second, since we want to return strings at the end

    let omit_newline = matches.is_present("omit_newline");  // saves option as boolean
    let text = matches.values_of_lossy("text").unwrap();    // we can use unwrap without fearing a panic, because the app will error if the argument doesn't exist

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\r\n" }); // using Vec::join to concat the Strings
    // note the use of carriage return, because windows is a special little snowflake
}
