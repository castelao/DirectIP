use assert_cmd::Command;

#[test]
fn missing_argument() {
    let mut cmd = Command::cargo_bin("directip-dump").unwrap();
    cmd.assert().failure();
}
