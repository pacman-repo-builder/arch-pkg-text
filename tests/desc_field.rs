use arch_pkg_text::desc::{FieldName, ParseFieldError, ParseRawFieldError, ParsedField, RawField};
use pretty_assertions::assert_eq;

#[test]
fn parse_raw_field() {
    let parse = |input| {
        RawField::parse_raw(input)
            .ok()
            .map(|field| field.name_str())
    };
    assert_eq!(parse("%NAME%"), Some("NAME"));
    assert_eq!(parse("%OPTDEPENDS%"), Some("OPTDEPENDS"));
    assert_eq!(parse("%MD5SUM%"), Some("MD5SUM"));
    assert_eq!(parse("%SHA256SUM%"), Some("SHA256SUM"));
    assert_eq!(parse("%B2SUM%"), Some("B2SUM"));
    assert_eq!(parse("%123%"), Some("123"));
}

#[test]
fn parse_parsed_field() {
    let parse = |input| ParsedField::parse(input).ok().map(|field| *field.name());
    assert_eq!(parse("%NAME%"), Some(FieldName::Name));
    assert_eq!(parse("%ISIZE%"), Some(FieldName::InstalledSize));
    assert_eq!(parse("%MD5SUM%"), Some(FieldName::Md5Checksum));
    assert_eq!(parse("%SHA256SUM%"), Some(FieldName::Sha256Checksum));

    // syntactically valid, it is the name that isn't known
    let error = ParsedField::parse("%THISFIELDISUNKNOWN%").unwrap_err();
    assert!(matches!(error, ParseFieldError::Name(_)));
    let error = ParsedField::parse("%123%").unwrap_err();
    assert!(matches!(error, ParseFieldError::Name(_)));
}

#[test]
fn parse_raw_field_error() {
    let error = RawField::parse_raw("NAME%").unwrap_err();
    assert!(matches!(
        error,
        ParseRawFieldError::IncorrectStartingCharacter,
    ));
    assert_eq!(error.to_string(), "Input doesn't start with '%'");

    let error = RawField::parse_raw("%NAME").unwrap_err();
    assert!(matches!(
        error,
        ParseRawFieldError::IncorrectEndingCharacter,
    ));
    assert_eq!(error.to_string(), "Input doesn't end with '%'");

    let error = RawField::parse_raw("%%").unwrap_err();
    assert!(matches!(error, ParseRawFieldError::Empty));
    assert_eq!(error.to_string(), "Field name is empty");

    let error = RawField::parse_raw("%name%").unwrap_err();
    assert!(matches!(
        error,
        ParseRawFieldError::InvalidCharacter(0, 'n'),
    ));
    assert_eq!(error.to_string(), "Found invalid character 'n' at index 0");

    let error = RawField::parse_raw("%SHA-256%").unwrap_err();
    assert!(matches!(
        error,
        ParseRawFieldError::InvalidCharacter(3, '-'),
    ));
    assert_eq!(error.to_string(), "Found invalid character '-' at index 3");
}
