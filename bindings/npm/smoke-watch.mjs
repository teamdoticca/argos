#!/usr/bin/env node
/**
 * Watch smoke: touch a file under a planned scope; expect ≥1 event.
 */
import { createRequire } from 'module'
import { dirname, join, resolve } from 'path'
import { fileURLToPath } from 'url'
import { existsSync, readFileSync, writeFileSync } from 'fs'

const require = createRequire(import.meta.url)
const { Workspace, watch } = require('./index.js')

const here = dirname(fileURLToPath(import.meta.url))
const fixture = resolve(here, '../../fixtures/small-pnpm')
const target = join(fixture, 'packages/app/src/index.ts')

if (!existsSync(target)) {
  console.error('WATCH SMOKE FAIL: missing', target)
  process.exit(1)
}

const ws = Workspace.open(fixture)
const scopes = ws.listScopes()
if (!Array.isArray(scopes) || scopes.length === 0) {
  console.error('WATCH SMOKE FAIL: empty scopes')
  process.exit(1)
}

const before = readFileSync(target, 'utf8')
const ac = new AbortController()
let saw = 0

const loop = (async () => {
  for await (const ev of watch(ws, ac.signal)) {
    console.log('event', JSON.stringify(ev))
    saw++
    if (saw > 0) {
      ac.abort()
      break
    }
  }
})()

await new Promise((r) => setTimeout(r, 200))
writeFileSync(target, `${before}\n// argos-smoke ${Date.now()}\n`)
await Promise.race([
  loop,
  new Promise((r) => setTimeout(r, 5000)),
])
writeFileSync(target, before)
ws.close()

if (saw < 1) {
  console.error('WATCH SMOKE FAIL: no events')
  process.exit(1)
}
console.log('WATCH SMOKE OK events=%d', saw)
