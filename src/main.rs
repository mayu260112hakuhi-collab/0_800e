mod engine;

use std::fs;
use std::borrow::Cow;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;
use engine::YaoyorozuEngine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();
    
    // 窓の作成
    let window = WindowBuilder::new()
        .with_title("八百万ランチャー (0_800E)")
        .with_inner_size(winit::dpi::LogicalSize::new(900.0, 600.0))
        .build(&event_loop)?;

    let yao_engine = YaoyorozuEngine::new();
    
    // index.8g の読み込みと解析
    let raw_html = fs::read_to_string("ui/index.8g").unwrap_or_else(|_| "<h1>File Not Found</h1>".into());
    let parsed_html = yao_engine.parse(&raw_html);

    // WebViewの構築
    let _webview = WebViewBuilder::new(&window)
        // カスタムプロトコル 800e:// の処理
        .with_custom_protocol("800e".into(), move |request| {
            let path = request.uri().path();
            let decoded = urlencoding::decode(path).unwrap_or(Cow::from(path));
            
            // パスの組み立て (先頭のスラッシュを除去して ui フォルダと結合)
            let file_path = std::path::PathBuf::from("ui").join(decoded.trim_start_matches('/'));

            // ★ デバッグログ：今どのファイルを見ようとしているかターミナルに表示
            println!("探しているファイル: {:?}", file_path);

            if let Ok(content) = fs::read(&file_path) {
                // 拡張子から MIMEタイプを判定
                let extension = file_path.extension().and_then(|s| s.to_str()).unwrap_or("");
                let mime = match extension {
                    "css" => "text/css",
                    "js"  => "text/javascript",
                    "png" => "image/png",
                    "jpg" | "jpeg" => "image/jpeg",
                    "svg" => "image/svg+xml",
                    _     => "text/html",
                };

                wry::http::Response::builder()
                    .header("content-type", mime)
                    .header("cache-control", "no-cache") // キャッシュさせない設定
                    .body(Cow::from(content))
                    .unwrap()
            } else {
                println!("【エラー】ファイルが見つかりません: {:?}", file_path);
                wry::http::Response::builder()
                    .status(404)
                    .body(Cow::from("Not Found".as_bytes()))
                    .unwrap()
            }
        })
        // JavaScript からのメッセージ受信
        .with_ipc_handler(|msg| {
            if msg == "launch_stream" {
                println!("【八百万エンジン】配信コアの起動命令を受信しました。");
            }
        })
        .with_html(&parsed_html)?
        .build()?;

    // イベントループの開始
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = event {
            *control_flow = ControlFlow::Exit;
        }
    });
}