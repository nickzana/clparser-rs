use crate::bind::Bind;
use crate::parser::Parser;
use crate::Argument;
use std::marker::PhantomData;

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq)]
pub enum ParseErr<T>
where
    T: Copy,
{
    UnknownArgument(String),
    MissingRequiredArgument(T),
    ArgumentMissingParameter(T),
}

// Sensible parsing defaults
#[derive(Debug)]
pub struct StandardParser<'a, T>
where
    T: Copy,
{
    _phantom_lifetime: PhantomData<Bind<'a, T>>,
}

impl<'a, T> StandardParser<'a, T>
where
    T: Copy,
{
    fn find_matching_bind(
        pattern: String,
        parameters: Vec<String>,
        binds: &'a Vec<&'a Bind<'a, T>>,
    ) -> Result<Argument<'a, T>, ParseErr<T>> {
        match binds
            .into_iter()
            .filter(|b| b.matches(&pattern))
            .collect::<Vec<&&Bind<T>>>()
            .first()
        {
            Some(b) => Ok(Argument::new(&b, parameters)),
            None => Err(ParseErr::UnknownArgument(pattern)),
        }
    }

    fn bind_arguments(
        mut args: Vec<String>,
        binds: &'a Vec<&'a Bind<'a, T>>,
    ) -> Vec<Result<Argument<'a, T>, ParseErr<T>>> {
        // split_inclusive puts matched element at end of preceding slice
        // Therefore, the argument parameters must precede the argument
        // Reversing the arguments does this
        args.reverse();
        args.split_inclusive_mut(|s| s.starts_with("-"))
            .filter_map(|slice| slice.split_last_mut())
            .map(|(pattern, parameters)| {
                Self::find_matching_bind(pattern.clone(), Vec::from(parameters), &binds)
            })
            .collect()
    }
}

impl<'a, T> Parser<'a, T> for StandardParser<'a, T>
where
    T: Copy,
{
    type Err = ParseErr<T>;

    fn parse(
        args: Vec<String>,
        binds: &'a Vec<&'a Bind<'a, T>>,
    ) -> Result<Vec<Argument<'a, T>>, Vec<ParseErr<T>>> {
        let args = Self::bind_arguments(args, binds);
        if args
            .iter()
            .filter(|arg| arg.is_err())
            .collect::<Vec<&Result<Argument<T>, ParseErr<T>>>>()
            .len()
            > 0
        {
            Err(args.into_iter().filter_map(|arg| arg.err()).collect())
        } else {
            Ok(args.into_iter().filter_map(|arg| arg.ok()).collect())
        }
    }
}
