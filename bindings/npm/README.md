# Argos for Node.js

Workspace intelligence maintained by Doticca: discover packages, plan watch scopes, explain paths and find owners from an immutable WorkspaceSnapshot.

```sh
npm install @teamdoticca/argos
```

```js
import { Workspace, watch } from '@teamdoticca/argos'

const workspace = Workspace.open('/path/to/repo')
try {
  console.log(workspace.currentSnapshot)
  console.log(workspace.listScopes())
  for await (const event of watch(workspace, AbortSignal.timeout(30_000))) {
    console.log(workspace.getAffectedScopes(event))
  }
} finally {
  workspace.close()
}
```

Filesystem-only mode for an explicit media directory (CommonJS and ESM examples under `examples/`):

```js
const ws = Workspace.open(mediaRoot, JSON.stringify({
  mode: 'filesystem',
  symlinkMode: 'nofollow',
  include: ['**/*.flac'],
  exclude: ['**/*.partial'],
}))
console.log(ws.currentSnapshot.planning.coverage)
const ac = new AbortController()
try {
  for await (const event of watch(ws, ac.signal)) {
    if (event.reason) console.log('rescan', event)
  }
} finally {
  ws.close()
}
```

Run the workspace example as an ES module. Stop the watcher before closing the workspace. Use one iterator per workspace and serialize operations.

Shipped addons: Windows x64, Linux x64 glibc, macOS ARM64. Use maintained Node.js 22 or 24. Older versions allowed by the manifest are not maintained support targets. Alpine/musl and other architectures are not included.

Argos is an early-stage 0.x library; pin versions and review compatibility before upgrading. Watching is optional, not a durable event log or a security boundary.

- [Repository and full examples](https://github.com/teamdoticca/argos)
- [Compatibility and troubleshooting](https://github.com/teamdoticca/argos/blob/master/docs/COMPATIBILITY.md)
- [Contributing](https://github.com/teamdoticca/argos/blob/master/CONTRIBUTING.md)
- [Security reporting](https://github.com/teamdoticca/argos/blob/master/SECURITY.md)
- [Changelog](https://github.com/teamdoticca/argos/blob/master/CHANGELOG.md)

MIT licensed; license text is included in the package.
