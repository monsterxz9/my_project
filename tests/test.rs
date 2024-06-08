#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn test_main() {
    // 模拟用户输入 10 并按回车键
    let mut input = String::from("10\n");

    // 模拟程序运行并捕获输出
    let output = std::process::Command::new("cargo")
        .args(["run"])
        .output()
        .expect("Failed to execute cargo run");

    // 检查输出是否包含 "The square of 10 is 100"
    assert!(output.stdout.contains(&b"The square of 10 is 100"[..]));
}
