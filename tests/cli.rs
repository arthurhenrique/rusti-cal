use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn runs_with_specific_year() {
    let mut cmd = Command::cargo_bin("rusti-cal").unwrap();
    cmd.arg("2025").assert().success().stdout(contains("2025"));
}
