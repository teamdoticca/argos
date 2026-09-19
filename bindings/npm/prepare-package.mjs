import { copyFileSync } from 'node:fs'

for (const name of ['LICENSE']) {
  copyFileSync(new URL(`../../${name}`, import.meta.url), new URL(name, import.meta.url))
}