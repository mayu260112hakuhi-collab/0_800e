use std::fs;
use std::path::Path;
use regex::Regex;

pub struct YaoyorozuEngine {
    pub is_streaming: bool,
    pub is_launcher: bool,
}

impl YaoyorozuEngine {
    pub fn new() -> Self {
        Self { 
            is_streaming: false,
            is_launcher: true, 
        }
    }

    pub fn parse(&self, content: &str) -> String {
        // 1. 【最優先】組込系をすべて実行（セミコロンなしでも動くように緩和）
        let mut result = self.process_all_includes(content);

        // 2. 「※」行の削除
        let comment_re = Regex::new(r"(?m)^\s*※.*$").unwrap();
        result = comment_re.replace_all(&result, "").to_string();

        // 3. 条件分岐（もし...）
        result = self.process_launcher_conditionals(&result);

        // 4. CSS/JS読込のタグ置換（ここもセミコロンなしを許容）
        let css_re = Regex::new(r#"CSS読込\("(.+?)"\);?"#).unwrap();
        result = css_re.replace_all(&result, |caps: &regex::Captures| {
            format!(r#"<link rel="stylesheet" href="800e://localhost/{}.css">"#, &caps[1])
        }).to_string();

        let js_re = Regex::new(r#"JS読込\("(.+?)"\);?"#).unwrap();
        result = js_re.replace_all(&result, |caps: &regex::Captures| {
            format!(r#"<script src="800e://localhost/{}.js"></script>"#, &caps[1])
        }).to_string();

        result
    }

    // 全ての組み込み命令を一括でループ処理する
    fn process_all_includes(&self, content: &str) -> String {
        let mut result = content.to_string();
        // 8g組込("...") または 組込("...") に反応
        let re = Regex::new(r#"(8g)?組込\("(.+?)"\);?"#).unwrap();
        
        let mut found = true;
        let mut loop_count = 0;
        
        while found && loop_count < 10 {
            found = false;
            let current = result.clone();
            let mut temp = current.clone();

            for cap in re.captures_iter(&current) {
                let is_8g_cmd = cap.get(1).is_some();
                let raw_path = &cap[2];
                
                // "../" などを削り、ui フォルダ起点のパスを作る
                let clean_path = raw_path.trim_start_matches("../").trim_start_matches("./");
                let mut file_path = Path::new("ui").join(clean_path);

                // ★ 拡張子がない場合の救済処置
                if file_path.extension().is_none() {
                    file_path.set_extension("8g"); // デフォルトで .8g を付ける
                }
                
                if let Ok(include_content) = fs::read_to_string(&file_path) {
                    temp = temp.replace(&cap[0], &include_content);
                    found = true;
                } else {
                    // .8g でダメなら .html も試してみる（予備）
                    file_path.set_extension("html");
                    if let Ok(include_content) = fs::read_to_string(&file_path) {
                        temp = temp.replace(&cap[0], &include_content);
                        found = true;
                    } else {
                        println!("【警告】やはりファイルが見つかりません: {:?}", file_path);
                    }
                }
            }
            result = temp;
            loop_count += 1;
        }
        result
    }
    fn process_launcher_conditionals(&self, content: &str) -> String {
        let re = Regex::new(r"(?s)もし\s*\(ここがランチャーなら\)\s*\{(.*?)\}").unwrap();
        re.replace_all(content, |caps: &regex::Captures| {
            if self.is_launcher { caps[1].trim().to_string() } else { "".to_string() }
        }).to_string()
    }
}