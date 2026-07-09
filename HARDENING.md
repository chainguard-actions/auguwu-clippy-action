<!-- markdownlint-disable -->

# Hardening Report: auguwu--clippy-action/1.5.0

> This file was generated automatically by the hardening agent.

**Policy SHA:** `d636be7e43ef829af6e853da6b3c7566db9f72fe`

**Test Policy SHA:** `843adf9e4b8f85d0c08b27b9d0b09dd094b54702`

**Harden Agent Version:** `1`

Action **auguwu--clippy-action/1.5.0** was hardened automatically. 3 finding(s) were identified and resolved across 1 iteration(s).

## Findings Fixed

### unpinned-uses (severity: high)

CI.yaml uses non-SHA pinned action references: actions/checkout@v6 (lines 46, 57), oven-sh/setup-bun@v2 (lines 47, 58), EndBug/add-and-commit@v9 (line 62). None are pinned to a full 40-character commit SHA.

Locations:

- `.github/workflows/CI.yaml:46`
- `.github/workflows/CI.yaml:47`
- `.github/workflows/CI.yaml:57`
- `.github/workflows/CI.yaml:58`
- `.github/workflows/CI.yaml:62`

### unpinned-uses (severity: high)

Dogfeed.yaml uses non-SHA pinned action references: actions/checkout@v6 (line 47), dtolnay/rust-toolchain@master (line 48). None are pinned to a full 40-character commit SHA. Using @master is especially risky as it tracks a mutable branch.

Locations:

- `.github/workflows/Dogfeed.yaml:47`
- `.github/workflows/Dogfeed.yaml:48`

### missing-permissions (severity: medium)

CI.yaml has no top-level permissions: key and neither of its jobs (ci, build) define job-level permissions. Without explicit permissions, the workflow inherits the default repository permissions, which may be overly broad.

Locations:

- `.github/workflows/CI.yaml:1`

## Iteration Notes

### Iteration 1

**Fixes applied:** unpinned-uses, missing-permissions

**Notes:**

CI.yaml: Pinned actions/checkout@v6 to SHA df4cb1c069e1874edd31b4311f1884172cec0e10, oven-sh/setup-bun@v2 to SHA 0c5077e51419868618aeaa5fe8019c62421857d6, and EndBug/add-and-commit@v9 to SHA a94899bca583c204427a224a7af87c02f9b325d5. Added top-level `permissions: contents: read` and a job-level `permissions: contents: write` override for the build job (which commits artifacts). Dogfeed.yaml: Pinned actions/checkout@v6 to SHA df4cb1c069e1874edd31b4311f1884172cec0e10 and dtolnay/rust-toolchain@master to SHA fa04a1451ff1842e2626ccb99004d0195b455a88. Dogfeed.yaml already had a top-level permissions block (checks: write) so no change needed there.

