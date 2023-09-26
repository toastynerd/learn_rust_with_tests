use std::process::Command;

#[test]
fn test_hello() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hello_world"])
        .output()
        .expect("failed to execute process");

    assert_eq!(output.status.success(), true);
    assert_eq!(output.stdout, b"Hello, world!\n");
}
