use regex::Regex;
use std::fs::{File, OpenOptions, self}; // 文件系统操作
use std::io::{BufReader, BufWriter, Read, Write, self, Error, ErrorKind};
use std::path::{Path, PathBuf};

/// 读取指定文件的所有内容并返回一个字符串结果
pub fn read_file(filename: &str) -> Result<String, std::io::Error> {
    // 打开文件
    let file = File::open(filename)?;
    // 创建一个缓冲读取器以高效读取文件
    let mut reader = BufReader::new(file);
    // 读取所有内容到一个新字符串中
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    Ok(content)
}

// 正则表达式 替换文本中的内容
fn replace_with_user_input(
    _file_content: &str,
    _output_file_name: &str,
    user_input: &str,
) -> std::io::Result<()> {

    // 创建一个正则表达式，并设置替换内容
    let re = Regex::new(r"_+[a-z0-9]+;").unwrap();
    let re2 = Regex::new(r"_+[a-z0-9]+::").unwrap();
    let replaced_content = re.replace_all(&_file_content, format!("_{};", user_input));
    let replaced_content2 = re2.replace_all(&replaced_content, format!("_{}::", user_input));

    // 写回文件
    let mut output_file = File::create(_output_file_name)?;
    output_file.write_all(replaced_content2.as_bytes())?;

    Ok(())
}

/// 将新的内容写入到指定文件中，如果文件存在，则替换原有内容
pub fn modify_and_save(filename: &str, new_content: &str) -> std::io::Result<()> {
    // 使用 OpenOptions 设置打开方式：写入模式和截断已有内容
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename)?;

    // 创建一个缓冲写入器以写入文件
    let mut writer = BufWriter::new(file);
    // 将新内容写入文件
    writer.write_all(new_content.as_bytes())?;

    // 确保所有数据都被刷新到磁盘
    writer.flush()?;

    Ok(())
}

// 复制文件的函数 返回一个需要处理的 result
fn copy_file(src_path: &str, dest_path: &str) -> Result<(), Error> {
    fs::copy(src_path, dest_path).map_err(|_| {
        Error::new(
            ErrorKind::Other,
            format!("Failed to copy file from {} to {}", src_path, dest_path),
        )
    })?;
    Ok(())
}

fn check_src (current_dir:PathBuf) {
    let path = current_dir;
    let src_path = path.join("src");

    if src_path.exists() && src_path.is_dir() {
        println!("当前文件中有 src 文件夹，但是也请再次确认是正确的目录");
    } else {
        println!("当前文件中没有 src 文件夹， 请确实是正确的目录!");
        panic!("当前文件路径错误");
    }
}

// main.rs - 主入口点
fn main() -> std::io::Result<()> {
    let current_dir = std::env::current_dir()?;
    println!("当前的工作目录是: {}", current_dir.display());
    check_src(current_dir.clone());
    // 源文件文件名
    let _filename = format!("{}/src/main.rs", current_dir.display());
    // 输出文件名
    let _out_file_name = format!("{}/src/main.rs", current_dir.display());
    // 备份文件名
    let _backup_file_name = format!("{}/src/backup_main.rs", current_dir.display());
    // 读取文件内容
    let _file_content = match read_file(&_filename) {
        // 假如读取文件成功返回 content 给 _file_content
        Ok(content) => {
            println!("读取文件成功");
            content
        }
        Err(e) => panic!("读取文件出错: {}", e),
    };

    // 备份原文件
    match copy_file(&_filename, &_backup_file_name) {
        Ok(()) => println!("备份文件成功！"),
        Err(_) => println!("备份文件时发生错误！"),
    }

    let mut _user_input = String::new();
    println!("请输入新的名称:");
    io::stdin().read_line(&mut _user_input).expect("输入时出错");
    println!("你输入的内容： {}", _user_input);

    // 移除用户输入的内容中的换行符
    if let Some('\n') = _user_input.chars().last() {
        _user_input.pop();
    } else if let Some('\r') = _user_input.chars().last() {
        _user_input.pop();
    }

    // 替换文本
    let _new_content = match replace_with_user_input(&_file_content, &_out_file_name, &_user_input) {
        Ok(r) => r,
        Err(e) => panic!("替换文本时出错, {} ", e),
    };

    // 创建新的 test 文件在 testfiles 文件夹下
    let _out_file_name = format!("{}/src/main.rs", current_dir.display());
    let _new_test_file_name = format!("{}/src/testfiles/_{}.rs", current_dir.display(),_user_input );
    let mut _new_test_file = File::create(_new_test_file_name)?;
    _new_test_file.write_all("pub fn main () {\n}".as_bytes())?;

    let _new_content = read_file(&_out_file_name).expect("读取输出文件时出错");
    println!("前后文件是否相同：{}", _file_content == _new_content);


    Ok(())
}
