// ... 既存のインポート ...
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();
    // 複数の窓を管理するために共有のイベントループターゲットが必要になる場合がありますが、
    // まずはシンプルに「メイン窓」を作ります。
    let window = WindowBuilder::new().with_title("八百万ランチャー").build(&event_loop)?;

    let yao_engine = YaoyorozuEngine::new();
    let raw_html = fs::read_to_string("ui/index.8g").unwrap_or_default();
    let parsed_html = yao_engine.parse(&raw_html);

    // WebViewの中でIPCを受け取る設定
    let _webview = WebViewBuilder::new(&window)
        .with_custom_protocol("800e".into(), move |request| {
            /* 前回のプロトコル処理... */
        })
        .with_ipc_handler(|_window, msg| {
    if msg.starts_with("open_window:") {
        let parts: Vec<&str> = msg.split(':').collect();
        let title = parts[1];
        let path = parts[2];
        let width = parts[3].parse::<f64>().unwrap_or(800.0);
        let height = parts[4].parse::<f64>().unwrap_or(600.0);

        println!("新しい窓「{}」をパス「{}」で召喚します", title, path);
        // ここで再帰的に新しい窓を作る関数を呼び出す
    }
})
        .with_html(&parsed_html)?
        .build()?;

    // ... event_loop.run ...
}