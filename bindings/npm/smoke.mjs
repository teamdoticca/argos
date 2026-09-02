#!/usr/bin/env node
/**
 * Smoke: Open fixtures/small-pnpm → non-empty nodes and scopes.
 * Usage: node smoke.mjs [fixtureRoot]
 */
import { createRequire } from 'module'
import { dirname, join, resolve } from 'path'
import { fileURLToPath } from 'url'
import { existsSync } from 'fs'

const require = createRequire(import.meta.url)
const { Workspace } = require('./index.js')

const here = dirname(fileURLToPath(import.meta.url))
const defaultFixture = resolve(here, '../../fixtures/small-pnpm')
const root = resolve(process.argv[2] || defaultFixture)

if (!existsSync(root)) {
  console.error('SMOKE FAIL: fixture missing', root)
  process.exit(1)
}

console.log('SMOKE Workspace.open(%s)', root)
const ws = Workspace.open(root)
const snap = ws.currentSnapshot
if (!snap?.identity?.state) {
  console.error('SMOKE FAIL: missing identity.state')
  process.exit(1)
}
const nodes = ws.listNodes()
const scopes = ws.listScopes()
if (!Array.isArray(nodes) || nodes.length === 0) {
  console.error('SMOKE FAIL: empty nodes')
  process.exit(1)
}
if (!Array.isArray(scopes) || scopes.length === 0) {
  console.error('SMOKE FAIL: empty scopes')
  process.exit(1)
}
ws.close()
console.log('SMOKE OK state=%s nodes=%d scopes=%d', snap.identity.state, nodes.length, scopes.length)
