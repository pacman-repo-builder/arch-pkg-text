pub const COMPLEX: &str = include_str!("fixtures/complex/.SRCINFO");
pub const SIMPLE: &str = include_str!("fixtures/simple/.SRCINFO");
pub const HAS_EMPTY_VALUES: &str = include_str!("fixtures/has-empty-values/.SRCINFO");
pub const MULTIPLE_CHECKSUM_TYPES: &str = include_str!("fixtures/multiple-checksum-types/.SRCINFO");

/// Convenient methods to manipulate a string.
pub trait StrUtils {
    /// Remove all indentations from every line in a string.
    fn without_indent(&self) -> String;
    /// Make the indentations of a string ugly, chaotic, and uneven.
    fn uneven_indent(&self) -> String;
    /// Add a single trailing whitespace to every line of a string.
    fn trailing_whitespaces(&self) -> String;
    /// Insert a given content above the first line that matches the predicate.
    fn insert_above_line(&self, predicate: impl FnMut(&str) -> bool, content: &str) -> String;
    /// Insert a given content below the first line that matches the predicate.
    fn insert_below_line(&self, predicate: impl FnMut(&str) -> bool, content: &str) -> String;
}

impl StrUtils for str {
    fn without_indent(&self) -> String {
        let mut output = String::with_capacity(self.len());

        for line in self.lines() {
            output.push_str(line.trim_start());
            output.push('\n');
        }

        output.shrink_to_fit();
        output
    }

    fn uneven_indent(&self) -> String {
        let indent_types = ["  ", "\t", " \t \t", "", "    "].into_iter().cycle();
        let mut output = String::new();

        for (line, indent) in self.lines().zip(indent_types) {
            output.push_str(indent);
            output.push_str(line.trim_start());
            output.push('\n');
        }

        output.shrink_to_fit();
        output
    }

    fn trailing_whitespaces(&self) -> String {
        let mut output = String::new();

        for line in self.lines() {
            output.push_str(line.trim_start());
            output.push_str(" \n");
        }

        output.shrink_to_fit();
        output
    }

    fn insert_above_line(&self, mut predicate: impl FnMut(&str) -> bool, content: &str) -> String {
        let mut lines = self.lines();
        let mut output = String::with_capacity(self.len() + content.len() + '\n'.len_utf8());
        let mut push = |line: &str| {
            output.push_str(line);
            output.push('\n');
        };
        for line in lines.by_ref() {
            if predicate(line) {
                push(content);
                push(line);
                break;
            }
            push(line);
        }
        lines.for_each(push);
        output
    }

    fn insert_below_line(&self, mut predicate: impl FnMut(&str) -> bool, content: &str) -> String {
        let mut lines = self.lines();
        let mut output = String::with_capacity(self.len() + content.len());
        let mut push = |line: &str| {
            output.push_str(line);
            output.push('\n');
        };
        for line in lines.by_ref() {
            push(line);
            if predicate(line) {
                push(content);
                break;
            }
        }
        lines.for_each(push);
        output
    }
}
