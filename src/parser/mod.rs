use crate::argument::Argument;
use crate::bind::Bind;

pub mod standard;

pub trait Parser<'a, T>
where
    T: Sized + Copy,
{
    type Err;
    fn parse(
        args: Vec<String>,
        binds: &'a Vec<&'a Bind<'a, T>>,
    ) -> Result<Vec<Argument<'a, T>>, Vec<Self::Err>>;
}
