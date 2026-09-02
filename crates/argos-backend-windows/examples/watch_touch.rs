//! Watch smoke for a path under a workspace.
//! `cargo run -p argos-backend-windows --example watch_touch -- Z:\Doticca\Mnemon src\frontend\mnemon-web\app\globals.css`

use argos_backend::WatchBackend;
use argos_backend_windows::WindowsBackend;
use argos_core::{list_scopes, OpenOptions, Workspace};
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() {
    let root = PathBuf::from(std::env::args().nth(1).expect("root"));
    let rel = std::env::args().nth(2).expect("relative file to touch");
    let target = root.join(&rel);

    let ws = Workspace::open(&root, OpenOptions::default()).expect("open");
    let snap = ws.current_snapshot();
    let scopes = list_scopes(&snap).to_vec();
    println!("scopes={}", scopes.len());
    assert!(!scopes.is_empty(), "empty scopes — discovery failed");

    let mut backend = WindowsBackend::new();
    backend.start(&ws, &scopes).expect("watch start");

    // Drain startup noise.
    thread::sleep(Duration::from_millis(200));
    let _ = backend.poll();

    let before = fs::read_to_string(&target).unwrap_or_default();
    let marker = format!(
        "\n/* argos-smoke {} */\n",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0)
    );
    fs::write(&target, format!("{before}{marker}")).expect("touch write");
    thread::sleep(Duration::from_millis(300));
    let mut saw = 0usize;
    for _ in 0..40 {
        let events = backend.poll().unwrap_or_default();
        for e in &events {
            println!("event={e:?}");
            saw += 1;
        }
        if saw > 0 {
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }
    let _ = fs::write(&target, before);
    backend.stop().ok();
    assert!(saw > 0, "expected at least one file event after touch of {rel}");
    println!("ok events={saw}");
}
