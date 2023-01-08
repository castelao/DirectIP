use assert_cmd::Command;

#[test]
fn missing_argument() {
    let mut cmd = Command::cargo_bin("directip-dump").unwrap();
    cmd.assert().failure();
}

#[test]
fn direction_mt() {
    let mut cmd = Command::cargo_bin("directip-dump").unwrap();

    let assert = cmd
        .arg("--direction")
        .arg("tests/data/mt_confirmation.isbd")
        .assert();
    assert.success().stdout("MT\n");
}

#[test]
fn imei() {
    let mut cmd = Command::cargo_bin("directip-dump").unwrap();

    let assert = cmd
        .arg("--imei")
        .arg("tests/data/mt_confirmation.isbd")
        .assert();
    assert
        .success()
        .stdout("00:01:02:03:04:05:06:07:08:09:0a:0b:0c:0d:0e\n");
}
