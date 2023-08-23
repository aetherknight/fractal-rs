/// This focuses mostly on the CLI output for help/versioning. It doesn't try to do full runs,
/// since that happens in a windowed program.
#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .case("tests/cmd/*.trycmd")
        .insert_var("[VERSION]", env!("CARGO_PKG_VERSION")).unwrap()
        .insert_var("[AUTHOR]", env!("CARGO_PKG_AUTHORS")).unwrap();
}
