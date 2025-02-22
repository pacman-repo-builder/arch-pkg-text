use super::DependencyName;

impl<'a> DependencyName<&'a str> {
    /// Extract a valid dependency name from an input string.
    ///
    /// > Package names should only consist of lowercase alphanumerics and
    /// > the following characters: `@._+-` (at symbol, dot, underscore, plus, hyphen).
    /// > Names are not allowed to start with hyphens or dots.
    /// >
    /// > -- from <https://wiki.archlinux.org/title/PKGBUILD#pkgname>
    ///
    /// ```
    /// # use arch_pkg_text::value::DependencyName;
    /// # use pretty_assertions::assert_eq;
    /// let (name, rest) = DependencyName::parse("rustup>=1.27.0-1");
    /// assert_eq!(name, DependencyName("rustup"));
    /// assert_eq!(rest, ">=1.27.0-1");
    /// ```
    pub fn parse(input: &'a str) -> (Self, &'a str) {
        let stop = input
            .char_indices()
            .find(|&(index, char)| !DependencyName::is_valid_char(index, char));

        let Some((stop_index, _)) = stop else {
            return (DependencyName(input), "");
        };

        let content = &input[..stop_index];
        let rest = &input[stop_index..];
        (DependencyName(content), rest)
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
