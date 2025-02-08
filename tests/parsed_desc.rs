use parse_arch_pkg_desc::{
    desc::Query,
    parse::{DescParseError, DescParseIssue, ParsedDesc},
    value::{Architecture, Dependency, Description, FileName, Name},
};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;

const TEXT: &str = include_str!("fixtures/gnome-shell.desc");

fn assert_query(querier: &ParsedDesc) {
    assert_eq!(querier.name(), Some(Name("gnome-shell")));

    assert_eq!(
        querier.file_name(),
        Some(FileName("gnome-shell-1:46.2-1-x86_64.pkg.tar.zst")),
    );

    let mut architecture = querier.architecture().unwrap().into_iter();
    assert_eq!(architecture.next(), Some(Architecture("x86_64")));
    assert_eq!(architecture.next(), None);

    assert_eq!(
        querier.description(),
        Some(Description("Next generation desktop shell")),
    );

    let mut make_dependencies = querier.make_dependencies().unwrap().into_iter();
    assert_eq!(make_dependencies.next(), Some(Dependency("asciidoc")));
    assert_eq!(
        make_dependencies.next(),
        Some(Dependency("bash-completion")),
    );
    assert_eq!(
        make_dependencies.next(),
        Some(Dependency("evolution-data-server")),
    );
    assert_eq!(make_dependencies.next(), Some(Dependency("gi-docgen")));
    assert_eq!(make_dependencies.next().map(|x| x.as_str()), Some("git"));
    assert_eq!(
        make_dependencies.next(),
        Some(Dependency("gnome-keybindings")),
    );
    assert_eq!(
        make_dependencies.next(),
        Some(Dependency("gobject-introspection")),
    );
    assert_eq!(make_dependencies.next(), Some(Dependency("meson")));
    assert_eq!(make_dependencies.next(), Some(Dependency("sassc")));
    assert_eq!(make_dependencies.next(), None);
}

#[test]
fn query() {
    let querier = ParsedDesc::parse(TEXT).unwrap();
    dbg!(&querier);
    assert_query(&querier);
}

#[test]
fn value_without_field() {
    let error = dbg!(ParsedDesc::parse("not a package description text")).unwrap_err();
    assert!(matches!(
        error,
        DescParseError::ValueWithoutField("not a package description text"),
    ));
    assert_eq!(
        error.to_string(),
        r#"Receive a value without field: "not a package description text""#,
    );

    let error = dbg!(ParsedDesc::parse("\nnot a package description text")).unwrap_err();
    assert!(matches!(error, DescParseError::ValueWithoutField("\n"),));
    assert_eq!(error.to_string(), r#"Receive a value without field: "\n""#,);
}

#[test]
fn empty_input() {
    let error = dbg!(ParsedDesc::parse("")).unwrap_err();
    assert!(matches!(error, DescParseError::EmptyInput,));
    assert_eq!(error.to_string(), "Input is empty");
}

#[test]
fn unknown_field() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct UnknownField<'a>(&'a str);

    fn stop_at_unknown_fields(issue: DescParseIssue) -> Result<(), UnknownField<'_>> {
        if let DescParseIssue::UnknownField(field) = issue {
            field.name_str().pipe(UnknownField).pipe(Err)
        } else {
            panic!("Unexpected issue: {issue:?}")
        }
    }

    let text = format!("{TEXT}\n%UNKNOWN%\nfoo\nbar\n");
    let (querier, error) =
        ParsedDesc::parse_with_issues(&text, stop_at_unknown_fields).into_partial();
    dbg!(&querier, &error);
    assert_eq!(error, Some(UnknownField("UNKNOWN")));
    assert_query(&querier);
}
