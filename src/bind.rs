use crate::pattern::Pattern;

#[derive(Debug, PartialEq)]
pub struct Bind<'a, T> {
    pub name: &'static str,
    pub help: Option<&'static str>,
    pub required: bool,
    pub patterns: Vec<Pattern<'a>>,
    pub takes_parameter: bool,
    pub kind: T,
}

impl<'a, T> Bind<'a, T>
where
    T: Copy,
{
    pub fn matches(&self, s: &str) -> bool {
        for pattern in &self.patterns {
            if pattern.matches(s) {
                return true;
            }
        }
        return false;
    }
}
