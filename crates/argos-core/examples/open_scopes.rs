//! Smoke: print scopes for a workspace root.
//! `cargo run -p argos-core --example open_scopes -- Z:\Doticca\Mnemon`

use argos_core::{list_scopes, OpenOptions, Workspace};

fn main() {
    let root = std::env::args()
        .nth(1)
        .expect("usage: open_scopes <workspace-root>");
    let ws = Workspace::open(&root, OpenOptions::default()).expect("open");
    let snap = ws.current_snapshot();
    println!("nodes={}", snap.model.nodes.len());
    for n in &snap.model.nodes {
        if matches!(n.kind, argos_core::NodeKind::OsArtifact) {
            continue;
        }
        println!(
            "node id={} provider={} root={}",
            n.id,
            n.source.provider,
            n.root.display()
        );
    }
    let scopes = list_scopes(&snap);
    println!("scopes={}", scopes.len());
    for s in scopes {
        println!(
            "scope package={} root={} watch={:?}",
            s.package, s.root, s.watch
        );
    }
}
