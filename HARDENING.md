<!-- markdownlint-disable -->

# Hardening Report: auguwu--clippy-action/1.2.1

> This file was generated automatically by the hardening agent.

**Policy SHA:** `d636be7e43ef829af6e853da6b3c7566db9f72fe`

**Test Policy SHA:** `843adf9e4b8f85d0c08b27b9d0b09dd094b54702`

**Harden Agent Version:** `2`

Action **auguwu--clippy-action/1.2.1** was hardened automatically. 2 finding(s) were identified and resolved across 1 iteration(s).

## Findings Fixed

### unpinned-uses (severity: high)

Multiple workflow files reference GitHub Actions using mutable tag or branch refs instead of immutable full 40-character SHA commit hashes. This exposes the workflow to supply-chain attacks if the referenced action is compromised or the tag is moved.

CI.yaml failing refs:
- `actions/checkout@v3` (tag)
- `actions/setup-node@v3` (tag)
- `auguwu/node-pm-action@master` (branch)

Dogfeed.yaml failing refs:
- `actions/checkout@v3` (tag)
- `dtolnay/rust-toolchain@master` (branch)

Locations:

- `.github/workflows/CI.yaml:46`
- `.github/workflows/CI.yaml:50`
- `.github/workflows/CI.yaml:54`
- `.github/workflows/Dogfeed.yaml:47`
- `.github/workflows/Dogfeed.yaml:51`

### missing-permissions (severity: medium)

CI.yaml has no top-level `permissions:` key and its only job (`lint`) also has no job-level `permissions:` key. Without explicit permissions, the workflow inherits the repository's default token permissions, which may be broader than necessary (e.g., write access to contents). A minimal explicit `permissions:` block should be added.

Locations:

- `.github/workflows/CI.yaml:1`

## Iteration Notes

### Iteration 1

**Fixes applied:** unpinned-uses, missing-permissions

**Notes:**

Fixed all 5 unpinned action references by pinning them to full 40-character commit SHAs (with original tag/branch preserved as comments). Added `permissions: {}` top-level block to CI.yaml to enforce least-privilege. Dogfeed.yaml already had a `permissions: checks: write` block which is appropriate for its use case.

