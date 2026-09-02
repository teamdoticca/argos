#!/usr/bin/env node
/** Fail publish if required platform natives are missing. */
import { existsSync } from 'fs'

const required = [
  'argos.win32-x64-msvc.node',
  'argos.linux-x64-gnu.node',
  'argos.darwin-arm64.node',
]

const missing = required.filter((f) => !existsSync(f))
if (missing.length) {
  console.error('prepublish check failed; missing natives:')
  for (const f of missing) console.error(' -', f)
  console.error('Build/stage all three platform addons before npm publish.')
  process.exit(1)
}
console.log('prepublish check ok:', required.join(', '))
