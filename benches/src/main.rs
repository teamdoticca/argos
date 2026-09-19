//! Argos benchmark harness — discovery / snapshot / rebuild timings.
//!
//! Metrics: Discovery Time, Snapshot Build, Rebuild, Active Watch Resources (scopes count).

use argos_core::{discover_topology, OpenOptions, Workspace};
use std::env;
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    let root = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("fixtures/small-pnpm"));

    println!("fixture={}", root.display());
    println!("os={}", std::env::consts::OS);

    let t0 = Instant::now();
    let topo = discover_topology(&root).expect("discover");
    let discovery_ms = t0.elapsed().as_secs_f64() * 1000.0;
    println!("discovery_ms={discovery_ms:.3}");
    println!("nodes={}", topo.nodes.len());

    let t1 = Instant::now();
    let ws = Workspace::open(&root, OpenOptions::default()).expect("open");
    let open_ms = t1.elapsed().as_secs_f64() * 1000.0;
    let snap = ws.current_snapshot();
    println!("open_snapshot_ms={open_ms:.3}");
    println!("scopes={}", snap.planning.scopes.len());
    println!(
        "active_watch_resources={}",
        snap.planning
            .scopes
            .iter()
            .map(|s| s.watch.len())
            .sum::<usize>()
    );

    let t2 = Instant::now();
    let _ = ws.rebuild_snapshot().expect("rebuild");
    let rebuild_ms = t2.elapsed().as_secs_f64() * 1000.0;
    println!("rebuild_ms={rebuild_ms:.3}");

    // Watch start timing is measured when a backend is available (optional).
    println!("watch_start_ms=n/a (optional; use FFI watch for platform timing)");
    println!("recovery_ms=n/a (synthetic overflow path covered in unit/integration)");
}
