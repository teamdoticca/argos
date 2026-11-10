#!/usr/bin/env bash
# Bookworm + Node 24: filesystem-mode followConfined must stay idle under read-only
# scans, then observe a real write. Builds a Bookworm-native linux-x64-gnu addon.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
IMAGE="argos-fs-bookworm:local"
VOLUME="argos-fs-bookworm-media"

echo "==> Building Bookworm validation image (Node 24 + Rust + build deps)"
docker build -t "$IMAGE" -f - "$ROOT" <<'EOF'
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
      ca-certificates curl build-essential python3 pkg-config \
    && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL https://deb.nodesource.com/setup_24.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /work
EOF

echo "==> Ensuring named volume ${VOLUME}"
docker volume create "$VOLUME" >/dev/null

echo "==> Access-noise idle + write detection on Bookworm"
docker run --rm \
  -v "$ROOT:/src:ro" \
  -v "${VOLUME}:/media/library" \
  "$IMAGE" \
  bash -lc '
    set -euo pipefail
    echo "node=$(node -v) glibc=$(ldd --version | head -1)"
    rm -rf /work/argos
    mkdir -p /work
    cp -a /src /work/argos
    rm -rf /work/argos/target /work/argos/bindings/npm/node_modules \
      /work/argos/bindings/npm/*.node /work/argos/bindings/npm/.napi-rs-filesystem-transaction.swp || true
    mkdir -p /media/library/Artist/Album
    for i in $(seq 0 49); do
      echo "audio$i" > "/media/library/Artist/Album/track$i.flac"
    done
    ln -sfn Album /media/library/Artist/AlbumLink || true
    cd /work/argos/bindings/npm
    npm install --ignore-scripts
    node build.mjs --release
    test -f argos.linux-x64-gnu.node
    node -e "
      const { Workspace } = require(\"./index.js\");
      const fs = require(\"fs\");
      const path = require(\"path\");

      function walk(dir, files) {
        for (const ent of fs.readdirSync(dir, { withFileTypes: true })) {
          const p = path.join(dir, ent.name);
          if (ent.isSymbolicLink()) continue;
          if (ent.isDirectory()) walk(p, files);
          else if (ent.isFile()) files.push(p);
        }
      }

      const ws = Workspace.open(\"/media/library\", JSON.stringify({
        mode: \"filesystem\",
        symlinkMode: \"followConfined\",
        eventBufferCapacity: 8,
        exclude: [\"**/*.partial\"]
      }));
      const snap = ws.currentSnapshot;
      if (snap.identity.mode !== \"filesystem\") throw new Error(\"mode\");
      if (snap.filesystem.symlinkMode !== \"followConfined\") throw new Error(\"symlinkMode\");
      console.log(\"ACCESS_NOISE_OPEN_OK\", JSON.stringify({
        mode: snap.identity.mode,
        symlinkMode: snap.filesystem.symlinkMode,
        scopes: ws.listScopes().length
      }));

      ws.watchStart();
      // Drain startup noise
      for (let i = 0; i < 10; i++) { ws.watchPoll(); }

      const bad = [];
      for (let round = 0; round < 5; round++) {
        const files = [];
        walk(\"/media/library\", files);
        for (const f of files) fs.readFileSync(f);
        const batch = ws.watchPoll() || [];
        for (const ev of batch) {
          const kind = ev.kind || ev;
          if ([\"fileCreated\",\"fileModified\",\"fileDeleted\",\"fileRenamed\",\"scopeRescan\",\"topologyChanged\"].includes(kind)) {
            bad.push(ev);
          }
        }
      }
      if (bad.length) {
        console.error(\"ACCESS_NOISE_FAIL\", JSON.stringify(bad.slice(0, 20)));
        ws.watchStop();
        ws.close();
        process.exit(2);
      }
      console.log(\"ACCESS_NOISE_IDLE_OK\");

      fs.writeFileSync(\"/media/library/Artist/Album/track0.flac\", \"mutated-\" + Date.now());
      let saw = false;
      const deadline = Date.now() + 5000;
      while (Date.now() < deadline) {
        const batch = ws.watchPoll() || [];
        if (batch.some(ev => {
          const kind = ev.kind || ev;
          return kind === \"fileModified\" || kind === \"fileCreated\" || kind === \"scopeRescan\";
        })) { saw = true; break; }
        const spin = Date.now() + 50;
        while (Date.now() < spin) { /* poll delay */ }
      }
      ws.watchStop();
      ws.close();
      if (!saw) {
        console.error(\"ACCESS_NOISE_WRITE_MISS\");
        process.exit(3);
      }
      console.log(\"ACCESS_NOISE_WRITE_OK\");
    "
  '

echo "==> Access-noise Bookworm validation succeeded"
