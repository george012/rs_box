use std::fs;
use toml::Value;
pub fn get_version() -> Result<String, Box<dyn std::error::Error>>{
    // 读取当前目录下的 Cargo.toml 文件
    let contents = fs::read_to_string("Cargo.toml")?;

    // 解析 TOML 内容
    let parsed = contents.parse::<Value>()?;

    // 尝试从解析后的 TOML 数据中获取版本号
    let version = parsed
        .get("package")
        .and_then(|pkg| pkg.get("version"))
        .and_then(|v| v.as_str())
        .ok_or("Version not found or is not a string")?
        .to_string();

    Ok(version)
}