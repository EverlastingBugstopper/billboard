use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_readme_deps() {
    version_sync::assert_markdown_deps_updated!("README.md");
}

#[test]
fn test_html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}

#[test]
fn test_eprint() {
    let mut cmd = Command::cargo_bin("examples/hello_world").unwrap();

    // running the CLI with no command returns to std err
    let result = cmd.assert();
    result.stderr(predicate::str::contains("Hello, World!"));
}
