#![feature(split_inclusive)]
pub mod bind;
pub mod parser;
pub mod pattern;

use bind::Bind;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Flag(bool),
    Parameters(Vec<String>),
}

#[derive(Debug, PartialEq)]
pub struct Argument<'a, T>
where
    T: Sized + Clone,
{
    pub bind: &'a Bind<'a, T>,
    pub value: Value,
}

impl<'a, T> Argument<'a, T>
where
    T: Sized + Clone,
{
    fn new(bind: &'a Bind<'a, T>, parameters: Vec<String>) -> Argument<'a, T> {
        let value = match bind.takes_parameter {
            true => Value::Parameters(parameters),
            false => Value::Flag(true),
        };
        Self { bind, value }
    }
}
