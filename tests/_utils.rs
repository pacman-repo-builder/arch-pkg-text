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

pub fn insert_line_under(input: &str, search: &str, value: &str) -> String {
    let mut lines = input.lines();
    let mut output = String::with_capacity(input.len() + value.len());
    for line in lines.by_ref() {
        output.push_str(line);
        output.push('\n');
        if line.contains(search) {
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
