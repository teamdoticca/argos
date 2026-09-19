import { spawnSync } from 'node:child_process'
import { createRequire } from 'node:module'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const require = createRequire(import.meta.url)
const cliPackagePath = require.resolve('@napi-rs/cli/package.json')
const cliPackage = require(cliPackagePath)
const cliPath = join(dirname(cliPackagePath), cliPackage.bin.napi)
const result = spawnSync(process.execPath, [
  cliPath, 'build', '--platform',
  '--manifest-path', '../../crates/argos-napi/Cargo.toml',
  '--output-dir', '.', '--js', 'binding.js', '--dts', 'binding.d.ts',
  ...process.argv.slice(2), '--', '--locked',
], { cwd: dirname(fileURLToPath(import.meta.url)), stdio: 'inherit' })

if (result.error) throw result.error
process.exit(result.status ?? 1)
