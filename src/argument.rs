use crate::bind::Bind;
use std::string::ToString;

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
    pub fn new(bind: &'a Bind<'a, T>, parameters: Vec<String>) -> Argument<'a, T> {
        let value = match bind.takes_parameter {
            true => Value::Parameters(parameters),
            false => Value::Flag(true),
        };
        Self { bind, value }
    }
}

impl<'a, T> ToString for Argument<'a, T>
where
    T: Sized + Clone,
{
    fn to_string(&self) -> String {
        self.bind.name.to_string()
    }
}
