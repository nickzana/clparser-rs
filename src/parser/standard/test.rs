use crate::argument::Value;
use crate::bind::Bind;
use crate::parser::standard::{Argument, StandardParser};
use crate::parser::Parser;
use crate::pattern::Pattern;

#[derive(PartialEq, Debug, Clone, Copy)]
enum CLArguments {
    ParameterA,
    FlagF,
}

#[test]
fn test_parse_simple_parameter() {
    let bind: Bind<CLArguments> = Bind {
        name: "E",
        help: None,
        required: true,
        patterns: vec![Pattern::Flag(&'e')],
        kind: CLArguments::ParameterA,
        takes_parameter: true,
    };

    let args = vec!["-e".to_string(), "value".to_string()];
    let binds = vec![&bind];
    let result = StandardParser::parse(args, &binds);
    assert_eq!(
        Ok(vec![Argument {
            bind: &bind,
            value: Value::Parameters(vec!["value".to_string()]),
        }]),
        result,
    );
}

#[test]
fn test_parse_flag() {
    let bind: Bind<CLArguments> = Bind {
        name: "Flag",
        help: None,
        required: true,
        patterns: vec![Pattern::Flag(&'f')],
        kind: CLArguments::FlagF,
        takes_parameter: false,
    };

    let args = vec!["-f".to_string()];
    let binds = vec![&bind];
    let results = StandardParser::parse(args, &binds);
    assert_eq!(
        Ok(vec![Argument {
            bind: &bind,
            value: Value::Flag(true),
        }]),
        results,
    );
}

#[test]
fn test_multiple_parameters() {
    let flag: Bind<CLArguments> = Bind {
        name: "Flag",
        help: None,
        required: true,
        patterns: vec![Pattern::Flag(&'f')],
        kind: CLArguments::FlagF,
        takes_parameter: false,
    };

    let value: Bind<CLArguments> = Bind {
        name: "E",
        help: None,
        required: true,
        patterns: vec![Pattern::Flag(&'e')],
        kind: CLArguments::ParameterA,
        takes_parameter: true,
    };

    let args = vec!["-f".to_string(), "-e".to_string(), "value".to_string()];
    let binds = vec![&flag, &value];

    let results = StandardParser::parse(args, &binds);
    let expected = vec![
        Argument {
            bind: &flag,
            value: Value::Flag(true),
        },
        Argument {
            bind: &value,
            value: Value::Parameters(vec!["value".to_string()]),
        },
    ];

    match results {
        Ok(results) => {
            for result in results {
                assert!(expected.contains(&result))
            }
        }
        Err(_) => assert!(false),
    }
}
