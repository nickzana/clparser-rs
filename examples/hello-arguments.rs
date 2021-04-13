extern crate clparser;

// Mandatory imports
use clparser::bind::Bind;
use clparser::pattern::Pattern;
// Used to extract parameters out of arguments
use clparser::argument::Value;
// Importing trait Parser is needed to use the "parse" method
// You may define your own struct that implements Parser if the StandardParser behavior
// is not suitable for your application
use clparser::parser::standard::{Parser, StandardParser};

// Define your own Arguments type with one entry for each argument in your application
// This type must implement Clone and Copy
#[derive(Clone, Copy)]
enum Arguments {
    BoldFlag,
    ItalicFlag,
    Output,
}

fn main() {
    // Define a vector of definitions that arguments passed into your program will "bind" to
    // The Bind type is generic over your Arguments type that defines all possible arguments
    let binds: Vec<Bind<Arguments>> = vec![
        Bind {
            name: "Bold",
            help: Some("Determines whether the outputted text is bold."),
            required: false,
            patterns: vec![
                Pattern::Flag(&'b'),
                Pattern::Short("bold"),
                Pattern::Long("bold"),
            ],
            kind: Arguments::BoldFlag,
            takes_parameter: false,
        },
        Bind {
            name: "Italic",
            help: Some("Determines whether the outputted text is italic."),
            required: false,
            patterns: vec![
                Pattern::Flag(&'i'),
                Pattern::Short("italic"),
                Pattern::Long("italic"),
                Pattern::Flag(&'s'),
                Pattern::Short("slanted"),
                Pattern::Long("slanted"),
            ],
            kind: Arguments::ItalicFlag,
            takes_parameter: false,
        },
        Bind {
            name: "Output",
            help: Some("The text to output to stdout."),
            required: true,
            patterns: vec![
                Pattern::Flag(&'o'),
                Pattern::Short("output"),
                Pattern::Long("output"),
            ],
            kind: Arguments::Output,
            takes_parameter: true,
        },
    ];

    let pointer = &binds.iter().collect();

    let args = std::env::args().collect::<Vec<String>>();

    // Using the StandardParser provides sensible defaults for parsing arguments
    // See documentation for the clparser::parser::Parser trait to define your own parsers
    let results = StandardParser::parse(args, pointer);

    // Check for any missing required arguments or input that cannot find a matching Bind
    // You may choose to warn users or stop execution based on the provided errors
    if let Err(errs) = results {
        println!("Error: incorrect or invalid arguments.");
        for err in errs {
            println!("{}", err.to_string())
        }
        panic!();
    }

    if let Ok(args) = results {
        let mut bold = false;
        let mut italic = false;
        let mut output: String = "".to_string();

        let _ = args.iter().map(|arg| match arg.bind.kind {
            Arguments::BoldFlag => bold = true,
            Arguments::ItalicFlag => italic = true,
            Arguments::Output => match &arg.value {
                Value::Flag(_) => println!("Missing output text."),
                Value::Parameters(s) => output = s[0].clone(),
            },
        });

        println!("\\e[1m]{}\\e[0m]", output);
    }
}
