use clap::{App, Arg};

fn main() {
    let matches = App::new("My RPN Program.")
        .version("1.0.0")
        .author("Your Name")
        .about("Super awsome sample RPN calculator")
        .arg(
            Arg::new("formula_file")
                .about("Formulas written in RPN")
                .value_name("File")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .about("Set the level of verbosity")
                .short('v')
                .long("verbose")
                .required(false),
        )
        .get_matches();

    match matches.value_of("formula_file") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }

    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified?: {}", verbose);
}
