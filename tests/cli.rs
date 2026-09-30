use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn runs_with_specific_year() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_rusti-cal"));
    cmd.arg("2025").assert().success().stdout(contains("2025"));
}

#[test]
fn week_numbers_not_affected_by_starting_day() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_rusti-cal"));
    cmd.args(["2023", "-w", "--starting-day", "1"])
        .assert()
        .success()
        .stdout(contains("38 18 19 20 21 22 23 24"));
}

#[test]
fn rejects_invalid_years() {
    for year in ["0", "4294967296", "invalid"] {
        Command::new(env!("CARGO_BIN_EXE_rusti-cal"))
            .arg(year)
            .assert()
            .failure()
            .stderr(contains("year must be a positive 32-bit integer"));
    }
}

#[test]
fn rejects_invalid_starting_days() {
    for day in ["7", "4294967295", "invalid"] {
        Command::new(env!("CARGO_BIN_EXE_rusti-cal"))
            .args(["2025", "--starting-day", day])
            .assert()
            .failure()
            .stderr(contains("starting day must be between"));
    }
}

#[test]
fn supports_boundary_years_and_all_starting_days() {
    for year in ["1", "9999", "10000", "4294967295"] {
        for day in 0..7 {
            Command::new(env!("CARGO_BIN_EXE_rusti-cal"))
                .args([year, "--starting-day", &day.to_string(), "-w", "-c"])
                .assert()
                .success();
        }
    }
}
