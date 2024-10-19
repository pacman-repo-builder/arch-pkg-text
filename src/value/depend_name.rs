use super::DependName;

impl<'a> DependName<'a> {
    /// Extract a valid dependency name from an input string.
    ///
    /// > Package names should only consist of lowercase alphanumerics and
    /// > the following characters: `@._+-` (at symbol, dot, underscore, plus, hyphen).
    /// > Names are not allowed to start with hyphens or dots.
    /// >
    /// > -- from <https://wiki.archlinux.org/title/PKGBUILD#pkgname>
    pub fn parse(input: &'a str) -> (Self, &'a str) {
        let stop = input
            .char_indices()
            .find(|&(index, char)| !DependName::is_valid_char(index, char));

        let Some((stop_index, _)) = stop else {
            return (DependName(input), "");
        };

        let content = &input[..stop_index];
        let rest = &input[stop_index..];
        (DependName(content), rest)
    }

    /// Check if a character belongs to a dependency name.
    fn is_valid_char(index: usize, char: char) -> bool {
        match (index, char) {
            // lowercase alphanumeric is always valid
            (_, 'a'..='z' | '0'..='9') => true,

            // hyphen and dot are forbidden as first character
            (0, '-' | '.') => false,

            // some special characters are allowed as non-first character
            (_, '@' | '.' | '_' | '+' | '-') => true,

            // otherwise, invalid
            (_, _) => false,
        }
    }
}
