pub const COMPLEX: &str = include_str!("fixtures/complex/.SRCINFO");
pub const SIMPLE: &str = include_str!("fixtures/simple/.SRCINFO");
pub const HAS_EMPTY_VALUES: &str = include_str!("fixtures/has-empty-values/.SRCINFO");
pub const MULTIPLE_CHECKSUM_TYPES: &str = include_str!("fixtures/multiple-checksum-types/.SRCINFO");

pub fn remove_indent(input: &str) -> String {
    let mut output = String::with_capacity(input.len());

    for line in input.lines() {
        output.push_str(line.trim_start());
        output.push('\n');
    }

    output.shrink_to_fit();
    output
}

pub fn uneven_indent(input: &str) -> String {
    let indent_types = ["  ", "\t", " \t \t", "", "    "].into_iter().cycle();
    let mut output = String::new();

    for (line, indent) in input.lines().zip(indent_types) {
        output.push_str(indent);
        output.push_str(line.trim_start());
        output.push('\n');
    }

    output.shrink_to_fit();
    output
}

pub fn trailing_whitespaces(input: &str) -> String {
    let mut output = String::new();

    for line in input.lines() {
        output.push_str(line.trim_start());
        output.push_str(" \n");
    }

    output.shrink_to_fit();
    output
}
