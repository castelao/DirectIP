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
        .arg("tests/data/sample_mt.isbd")
        .assert();
    assert.success().stdout("MT\n");
}
