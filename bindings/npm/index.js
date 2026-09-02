'use strict'

const native = require('./binding.js')

const { Workspace, computeDelta } = native

function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

/**
 * AsyncIterable watch helper (poll loop). Abort via AbortSignal.
 * @param {InstanceType<typeof Workspace>} workspace
 * @param {AbortSignal} [signal]
 */
async function* watch(workspace, signal) {
  workspace.watchStart()
  try {
    while (!signal?.aborted) {
      const events = workspace.watchPoll()
      if (Array.isArray(events)) {
        for (const ev of events) {
          yield ev
        }
      }
      await delay(50)
    }
  } finally {
    workspace.watchStop()
  }
}

module.exports = {
  Workspace,
  computeDelta,
  watch,
}
