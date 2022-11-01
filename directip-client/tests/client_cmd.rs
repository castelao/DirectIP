use assert_cmd::Command;
use assert_fs::prelude::*;
// use predicates::prelude::*;
// use predicates::str::contains;

#[test]
// Missing msg-id
// Eventually this will be optional
fn missing_msg_id() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("directip-client")?;

    cmd.args(&["--server=127.0.0.1:10800"])
        .args(&["--imei=012345678901234"])
        .arg("42")
        .assert()
        .failure()
        //.stderr(contains("Missing msg-id"))
        ;

    Ok(())
}

#[test]
// Missing server
fn missing_server() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("directip-client")?;

    cmd.args(&["--imei=012345678901234"])
        .args(&["--msg-id=987"])
        .arg("42")
        .assert()
        .failure()
        //.stderr(contains("Missing server"))
        ;

    Ok(())
}

#[test]
// Missing IMEI
fn missing_imei() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("directip-client")?;

    cmd.args(&["--server=127.0.0.1:10800"])
        .args(&["--msg-id=987"])
        .arg("42")
        .assert()
        .failure()
        //.stderr(contains("Missing server"))
        ;

    Ok(())
}

#[test]
// An ASCII payload as an argument
fn ascii_inline() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("directip-client")?;

    cmd.args(&["--msg-id=987"])
        .args(&["--server=127.0.0.1:10800"])
        .args(&["--imei=012345678901234"])
        .args(&["--dry-run"])
        .arg("42")
        .assert()
        .success()
        //.stdout(contains("42"))
        ;

    Ok(())
}

#[test]
// An ASCII payload from a file
fn ascii_fromfile() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("payload.txt")?;
    file.write_str("Hello World")?;

    let mut cmd = Command::cargo_bin("directip-client")?;

    cmd.args(&["--msg-id=987"])
        .args(&["--server=127.0.0.1:10800"])
        .args(&["--imei=012345678901234"])
        .args(&["--from-file"])
        .args(&["--dry-run"])
        .arg(file.path())
        .assert()
        .success()
        //.stdout(contains("Hello World"))
        ;

    Ok(())
}

#[test]
// A binary payload from a file
fn binary_fromfile() -> Result<(), Box<dyn std::error::Error>> {
    let payload = vec![0x80, 0x90, 0xa0];
    // Confirm that it is an invalid UTF-8
    assert!(std::str::from_utf8(&payload).is_err());

    let file = assert_fs::NamedTempFile::new("payload.txt")?;
    file.write_binary(&payload)?;

    let mut cmd = Command::cargo_bin("directip-client")?;
    cmd.args(&["--msg-id=987"])
        .args(&["--server=127.0.0.1:10800"])
        .args(&["--imei=012345678901234"])
        .args(&["--encoding=binary"])
        .args(&["--from-file"])
        .args(&["--dry-run"])
        .arg(file.path())
        .assert()
        .success()
        //.stdout(contains("Hello World"))
        ;

    Ok(())
}

#[test]
// An ASCII payload from stdin
fn ascii_stdin() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("directip-client")?;

    cmd.args(&["--msg-id=987"])
        .args(&["--server=127.0.0.1:10800"])
        .args(&["--imei=012345678901234"])
        .args(&["--dry-run"])
        .write_stdin("42")
        .assert()
        .success()
        //.stdout(contains("42"))
        ;

    Ok(())
}

#[test]
// A binary payload from stdin
fn binary_stdin() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("directip-client")?;

    let payload = vec![0x80, 0x90, 0xa0];
    // Confirm that it is an invalid UTF-8
    assert!(std::str::from_utf8(&payload).is_err());

    cmd.args(&["--msg-id=987"])
        .args(&["--server=127.0.0.1:10800"])
        .args(&["--imei=012345678901234"])
        .args(&["--dry-run"])
        .write_stdin(payload)
        .assert()
        .success()
        //.stdout(contains("42"))
        ;

    Ok(())
}
