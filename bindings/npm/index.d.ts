import { Workspace, computeDelta } from './binding'

export { Workspace, computeDelta }

export declare function watch(
  workspace: Workspace,
  signal?: AbortSignal,
): AsyncGenerator<object, void, unknown>
