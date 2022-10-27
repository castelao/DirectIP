use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

use predicates::str::contains;

#[test]
fn missing_msg_id() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("directip-client")?;

    cmd.args(&["--server=127.0.0.1:10800"])
        .args(&["--imei=012345678901234"])
        .args(&["--dry-run"])
        .arg("Hello World")
        .assert()
        .failure()
        //.stderr(contains("Missing msg-id"))
        ;

    Ok(())
}

#[test]
fn inline_ascii() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("directip-client")?;

    cmd.args(&["--msg-id=987"])
        .args(&["--server=127.0.0.1:10800"])
        .args(&["--imei=012345678901234"])
        .args(&["--dry-run"])
        .arg("Hello World")
        .assert()
        .success()
        //.stdout(contains("Hello World"))
        ;

    Ok(())
}
