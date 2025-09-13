use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn runs_with_specific_year() {
    let mut cmd = Command::cargo_bin("rusti-cal").unwrap();
    cmd.arg("2025").assert().success().stdout(contains("2025"));
}

#[test]
fn week_numbers_not_affected_by_starting_day() {
    let mut cmd = Command::cargo_bin("rusti-cal").unwrap();
    cmd.args(["2023", "-w", "--starting-day", "1"])
        .assert()
        .success()
        .stdout(contains("38 18 19 20 21 22 23 24"));
}
