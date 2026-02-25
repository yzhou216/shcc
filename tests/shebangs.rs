use std::process::Command;

#[test]
fn hello() {
    let output = Command::new("tests/c/hello.c")
        .output()
        .expect("Failed to run C program");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout, "Hello, world!\n");
}
