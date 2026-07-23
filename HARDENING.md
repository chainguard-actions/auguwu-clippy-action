<!-- markdownlint-disable -->

# Hardening Report: auguwu--clippy-action/1.5.0

> This file was generated automatically by the hardening agent.

**Policy SHA:** `d636be7e43ef829af6e853da6b3c7566db9f72fe`

**Test Policy SHA:** `843adf9e4b8f85d0c08b27b9d0b09dd094b54702`

**Harden Agent Version:** `2`

Action **auguwu--clippy-action/1.5.0** was hardened automatically. 2 finding(s) were identified and resolved across 1 iteration(s).

## Findings Fixed

### unpinned-uses (severity: high)

Multiple workflow steps use mutable tag or branch refs instead of pinned full 40-character SHA commit hashes, making the workflow vulnerable to supply-chain attacks if the referenced tag is moved or the action is compromised.

In CI.yaml: `actions/checkout@v6`, `oven-sh/setup-bun@v2`, `EndBug/add-and-commit@v9`.
In Dogfeed.yaml: `actions/checkout@v6`, `dtolnay/rust-toolchain@master` (branch ref).

All of these should be replaced with their full SHA digests, e.g. `actions/checkout@<40-char-sha> # v6`.

Locations:

- `.github/workflows/CI.yaml:46`
- `.github/workflows/CI.yaml:47`
- `.github/workflows/CI.yaml:54`
- `.github/workflows/CI.yaml:55`
- `.github/workflows/CI.yaml:59`
- `.github/workflows/Dogfeed.yaml:43`
- `.github/workflows/Dogfeed.yaml:44`

### missing-permissions (severity: medium)

The workflow file CI.yaml has no top-level `permissions:` key, and neither the `ci` job nor the `build` job defines its own `permissions:` block. Without explicit permissions, the workflow inherits the default repository permissions (which may be read/write), violating the principle of least privilege. A top-level `permissions:` block with minimal specific scopes (e.g. `contents: read`) should be added.

Locations:

- `.github/workflows/CI.yaml:1`

## Iteration Notes

### Iteration 1

**Fixes applied:** unpinned-uses, missing-permissions

**Notes:**

CI.yaml: Added top-level `permissions: contents: read` block for least-privilege. The `build` job (which commits artifacts) gets a job-level `permissions: contents: write` override. All 5 unpinned action refs in CI.yaml pinned to full SHAs (actions/checkout@d23441a4, oven-sh/setup-bun@0c5077e5, EndBug/add-and-commit@a94899bc). Dogfeed.yaml: Pinned actions/checkout@d23441a4 and dtolnay/rust-toolchain@2c7215f1 to full SHAs. Dogfeed.yaml already had a `permissions: checks: write` block so no permissions fix was needed there.

