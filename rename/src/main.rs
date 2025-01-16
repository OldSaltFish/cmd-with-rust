use std::env;
use std::fs::{self, rename};
use std::io::{stdin, stdout, Write};
use uuid::Uuid;

fn main() {
    println!("欢迎使用文件重命名工具！");

    // 确认用户是否继续
    if !confirm_continue("是否继续重命名？按 q 退出，其他键继续") {
        println!("操作已取消。");
        return;
    }

    // 获取前缀和后缀
    let prefix = prompt_input("请输入文件前缀：");
    let extension = prompt_input("请输入文件扩展名（不含点）：");

    // 获取当前工作目录
    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("无法获取当前工作目录: {}", e);
            return;
        }
    };

    // 获取所有文件并临时重命名
    let files = match fs::read_dir(current_dir) {
        Ok(entries) => entries.filter_map(Result::ok).collect::<Vec<_>>(),
        Err(e) => {
            eprintln!("读取文件时出错: {}", e);
            return;
        }
    };

    let mut temp_names = Vec::new();
    for file in &files {
        let path = file.path();
        if path.is_file() {
            let temp_name = format!("{}.tmp", Uuid::new_v4());
            if let Err(e) = rename(&path, &temp_name) {
                eprintln!("临时重命名文件 {} 失败: {}", path.display(), e);
                continue;
            }
            temp_names.push((path, temp_name));
        }
    }

    // 最终重命名
    for (i, (original_path, temp_name)) in temp_names.iter().enumerate() {
        let new_name = format!("{}{}.{}", prefix, i + 1, extension);
        if let Err(e) = rename(temp_name, &new_name) {
            eprintln!("重命名文件 {} 到 {} 失败: {}", temp_name, new_name, e);
            continue;
        }
        println!("已重命名: {} -> {}", original_path.display(), new_name);
    }

    println!("重命名完成！");
}

fn confirm_continue(prompt: &str) -> bool {
    print!("{} (q/Q 退出): ", prompt);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    !input.trim().eq_ignore_ascii_case("q")
}

fn prompt_input(prompt: &str) -> String {
    print!("{}: ", prompt);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}