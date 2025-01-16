// use std::fs::File;
use std::io::{self, Write};
use structopt::StructOpt;
#[derive(StructOpt)]
struct Cli {
    filename: String,
    url: String,
}
fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta http-equiv="refresh" content="0;url=http://{url}">
<title>Redirecting</title>
</head>
<body>
</body>
</html>"#,url=&args.url);
    let path = format!("{}.html",&args.filename);
    let mut file = std::fs::File::create(path)?;
    // 将字符串写入文件
    file.write_all(html.as_bytes())?;
    // 成功完成，返回 Ok(())
    Ok(())
}
