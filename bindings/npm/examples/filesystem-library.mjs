/**
 * Minimal filesystem-mode example for an arbitrary media directory.
 * Usage: node examples/filesystem-library.mjs [libraryRoot]
 *
 * Demonstrates: open options, link/filter selection, coverage diagnostics,
 * change/rescan consumption, AbortSignal cancellation and close().
 */
import { createRequire } from 'node:module'
import { mkdirSync, writeFileSync, rmSync } from 'node:fs'
import { join } from 'node:path'
import { tmpdir } from 'node:os'

const require = createRequire(import.meta.url)
const { Workspace, watch } = require('../index.js')

const libraryRoot = process.argv[2] || join(tmpdir(), `argos-fs-demo-${process.pid}`)
mkdirSync(join(libraryRoot, 'Artist', 'Album'), { recursive: true })
writeFileSync(join(libraryRoot, 'Artist', 'Album', 'track.flac'), 'demo')

const ws = Workspace.open(
  libraryRoot,
  JSON.stringify({
    mode: 'filesystem',
    symlinkMode: 'nofollow',
    include: ['**/*.flac', '**/*.mp3'],
    exclude: ['**/*.partial', 'staging/**'],
    eventBufferCapacity: 8192,
  }),
)

try {
  const snap = ws.currentSnapshot
  console.log('mode:', snap.identity.mode)
  console.log('symlinkMode:', snap.filesystem.symlinkMode)
  console.log('scopes:', ws.listScopes())
  console.log('coverage:', snap.planning.coverage)
  console.log('filters:', snap.planning.filters)
  console.log('health:', ws.health())

  const ac = new AbortController()
  const timer = setTimeout(() => ac.abort(), 1500)

  // Touch a file so the watch loop has something observable in short demos.
  setTimeout(() => {
    writeFileSync(join(libraryRoot, 'Artist', 'Album', 'track.flac'), `demo-${Date.now()}`)
  }, 200)

  for await (const event of watch(ws, ac.signal)) {
    console.log('event:', JSON.stringify(event))
    if (event?.kind === 'scopeRescan' || event?.reason) {
      console.log('rescan/recovery — reconcile from currentSnapshot')
      console.log(ws.currentSnapshot.planning.coverage)
    }
  }
  clearTimeout(timer)
} finally {
  ws.close()
  if (!process.argv[2]) {
    rmSync(libraryRoot, { recursive: true, force: true })
  }
}
