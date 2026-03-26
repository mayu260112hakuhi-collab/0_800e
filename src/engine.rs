use std::fs;
use std::path::Path;
use regex::Regex;

pub struct YaoyorozuEngine {
    pub is_streaming: bool,
}

impl YaoyorozuEngine {
    pub fn parse(&self, content: &str) -> String {
        let mut result = content.to_string();

        // 1. 基本の解析（組込、もし、コメント）
        result = self.process_basic_syntax(&result);

        // 2. 特殊命令の翻訳：窓召喚("タイトル", "パス", 横, 縦)
        // これを JS の window.ipc.postMessage に変換して、後で main.rs が拾えるようにする
        let window_re = Regex::new(r#"窓召喚\("(.+?)",\s*"(.+?)",\s*(\d+),\s*(\d+)\);"#).unwrap();
        result = window_re.replace_all(&result, |caps: &regex::Captures| {
            format!(
                r#"<script>function 窓起動() {{ window.ipc.postMessage("open_window:{}:{}:{}:{}"); }}</script>
                   <div onclick="窓起動()">"#, 
                &caps[1], &caps[2], &caps[3], &caps[4]
            )
        }).to_string();

        result
    }
}