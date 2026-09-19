# Design

Use the existing protected PR and explicit workflow_dispatch publication flow. Keep versioned package artifacts immutable and publish through registry OIDC without token fallback. Reuse native matrices and consumer smokes. Do not change WorkspaceSnapshot or introduce product features.
