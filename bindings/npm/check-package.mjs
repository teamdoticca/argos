import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'

const report = JSON.parse(readFileSync(process.argv[2], 'utf8'))
assert.equal(report.length, 1)
const packed = report[0]
const manifest = JSON.parse(readFileSync(new URL('package.json', import.meta.url), 'utf8'))
assert.equal(packed.name, manifest.name)
assert.equal(packed.version, manifest.version)
const paths = new Set(packed.files.map(file => file.path))
for (const path of ['LICENSE', 'README.md', 'package.json', 'index.js', 'index.d.ts', 'binding.js', 'binding.d.ts']) {
  assert(paths.has(path), `Missing package file: ${path}`)
}
for (const path of paths) {
  assert(/^(LICENSE|README\.md|package\.json|index\.(js|d\.ts)|binding\.(js|d\.ts)|argos\.[a-z0-9-]+\.node)$/.test(path), `Unexpected package file: ${path}`)
}
console.log('Package metadata and contents verified')