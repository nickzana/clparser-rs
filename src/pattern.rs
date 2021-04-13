#[derive(Debug, PartialEq)]
pub enum Pattern<'a> {
    Flag(&'a char), // Single "-" and single character pattern
    Short(&'a str), // Single "-" and multiple character pattern
    Long(&'a str),  // Two "-" and any length pattern
}

impl<'a> Pattern<'a> {
    // Determines whether a string matches the pattern
    // Ensure that all flags include their "-" in order to properly match
    // against a Pattern
    //
    pub fn matches(&self, s: &str) -> bool {
        match self {
            Self::Flag(c) => {
                if let Some(s) = s.strip_prefix("-") {
                    return s == c.to_string() && s.len() == 1;
                }
            }
            Self::Short(p) => {
                if let Some(s) = s.strip_prefix("-") {
                    return s == p.to_string();
                }
            }
            Self::Long(p) => {
                if let Some(s) = s.strip_prefix("--") {
                    return s == p.to_string();
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Pattern;
    #[test]
    fn test_flag() {
        let flag = Pattern::Flag(&'c');
        // Flags will match single characters with a single "-"
        assert!(flag.matches("-c"));
        // Flags will not match a single "-" with multiple characters
        assert!(!flag.matches("-cc"));
        // Flags will not match single characters with two "-"s
        assert!(!flag.matches("--c"));
    }

    #[test]
    fn test_short() {
        let short = Pattern::Short("word");
        // Short will match a word with a single "-"
        assert!(short.matches("-word"));
        // Shorts will not match a word with two "-"s
        assert!(!short.matches("--word"));
        // Shorts will not match a character with a single "-"
        assert!(!short.matches("-w"));
    }

    #[test]
    fn test_long() {
        let long = Pattern::Long("word");
        // Long will match a word with two "-"s
        assert!(long.matches("--word"));
        // Long will not match a word with a single "-"
        assert!(!long.matches("-word"));

        let long = Pattern::Long("c");
        // Long will match a character with a single "-"
        assert!(long.matches("--c"));
        // Long will not match a character with a single "-"
        assert!(!long.matches("-c"));
    }
}
