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
    /// Insert a line above a line.
    fn insert_line_above(&self, search: impl FnMut(&str) -> bool, value: &str) -> String;
    /// Insert a line under a line.
    fn insert_line_under(&self, search: impl FnMut(&str) -> bool, value: &str) -> String;
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

    fn insert_line_above(&self, mut search: impl FnMut(&str) -> bool, value: &str) -> String {
        let mut lines = self.lines();
        let mut output = String::with_capacity(self.len() + value.len());
        let mut push = |line: &str| {
            output.push_str(line);
            output.push('\n');
        };
        for line in lines.by_ref() {
            if search(line) {
                push(value);
                push(line);
                break;
            }
            push(line);
        }
        for line in lines {
            push(line);
        }
        output
    }

    fn insert_line_under(&self, mut search: impl FnMut(&str) -> bool, value: &str) -> String {
        let mut lines = self.lines();
        let mut output = String::with_capacity(self.len() + value.len());
        for line in lines.by_ref() {
            output.push_str(line);
            output.push('\n');
            if search(line) {
                output.push_str(value);
                output.push('\n');
                break;
            }
        }
        for line in lines {
            output.push_str(line);
            output.push('\n');
        }
        output
    }
}
