// tests/test.rs
extern crate my_project;
use my_project::add; // 导入add函数
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_add_basic() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn test_add_negative() {
    assert_eq!(add(-1, -2), -3);
}

#[test]
fn test_main() {
    // 创建一个子进程，模拟程序运行
    let mut child = Command::new("cargo")
        .args(["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start child process");

    // 获取子进程的标准输入句柄
    let child_stdin = child.stdin.as_mut().expect("Failed to open stdin");

    // 向子进程写入输入
    write!(child_stdin, "10\n").expect("Failed to write to stdin");

    // 获取子进程的输出
    let output = child.wait_with_output().expect("Failed to read stdout");

    // 将输出转换为字符串
    let output_str = String::from_utf8_lossy(&output.stdout);

    // 检查输出是否包含 "The square of 10 is 100"
    assert!(output_str.contains("The square of 10 is 100"));
}
