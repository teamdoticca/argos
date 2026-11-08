/**
 * CommonJS filesystem-mode example (Node 24).
 * Usage: node examples/filesystem-library.cjs [libraryRoot]
 */
'use strict'

const { mkdirSync, writeFileSync, rmSync } = require('node:fs')
const { join } = require('node:path')
const { tmpdir } = require('node:os')
const { Workspace, watch } = require('../index.js')

async function main() {
  const libraryRoot = process.argv[2] || join(tmpdir(), `argos-fs-cjs-${process.pid}`)
  mkdirSync(join(libraryRoot, 'Artist'), { recursive: true })
  writeFileSync(join(libraryRoot, 'Artist', 'song.flac'), 'demo')

  const ws = Workspace.open(
    libraryRoot,
    JSON.stringify({
      mode: 'filesystem',
      symlinkMode: 'followConfined',
      exclude: ['**/*.partial'],
    }),
  )

  try {
    const snap = ws.currentSnapshot
    console.log('mode', snap.identity.mode)
    console.log('coverage', snap.planning.coverage)
    console.log('scopes', ws.listScopes())

    const ac = new AbortController()
    setTimeout(() => ac.abort(), 1000)
    setTimeout(() => {
      writeFileSync(join(libraryRoot, 'Artist', 'song.flac'), `demo-${Date.now()}`)
    }, 100)

    for await (const event of watch(ws, ac.signal)) {
      console.log('event', event)
      console.log('affected', ws.getAffectedScopes(event))
    }
  } finally {
    ws.close()
    if (!process.argv[2]) rmSync(libraryRoot, { recursive: true, force: true })
  }
}

main().catch((err) => {
  console.error(err)
  process.exitCode = 1
})
