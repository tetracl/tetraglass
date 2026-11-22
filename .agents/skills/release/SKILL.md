---
name: release
description: Prepares and publishes a release of the tetraglass crates to crates.io (version bump, CHANGELOG, package checks, tag, publish). Use only when the maintainer asks for a release, normally after a milestone's gate review is merged and its milestone issue is closed.
---

# Release

A release is a pull request that bumps the version, then a tag and a
publish that happen only on the maintainer's explicit instruction. Publishing
to crates.io cannot be undone.

## 1. Check

- The maintainer named the version, or agreed to the one you propose
  (0.x semver: a minor bump for new features or breaking changes).
- The milestone's gate record (`docs/reviews/M<n>.md`) is merged and the
  main branch's CI is green.
- `crates.io` does not already have this version:
  `curl -s https://crates.io/api/v1/crates/tetraglass-core` lists the
  published versions.

## 2. Release pull request

On a branch `release/v<x.y.z>` from the main branch:

1. Set the version in `[workspace.package]` and in every internal entry of
   `[workspace.dependencies]` in the root `Cargo.toml`.
2. Add a `CHANGELOG.md` section for the version (Keep a Changelog: Added,
   Changed, Fixed), written from the pull requests merged since the last
   tag: `gh pr list --state merged --search "merged:>=<date of last tag>"`.
3. Check that every published crate has a description, README, license,
   repository and documentation link.
4. Run the `full` check from `AGENTS.md`, then
   `cargo publish --workspace --dry-run --locked` and
   `cargo doc --workspace --no-deps` (no warnings).
5. Open a pull request titled `Release v<x.y.z>` with what the dry run
   packaged and anything the maintainer should look at.

## 3. Tag and publish, only when told

Stop after the pull request. When the maintainer has merged it and tells
you, in the session, to publish that version:

1. Update the main branch, check that the merge commit's CI is green and
   that the working tree is clean.
2. `git tag -a v<x.y.z> -m "Release v<x.y.z>"` on that commit, and
   `git push origin v<x.y.z>`. Never move or delete an existing tag.
3. `cargo publish --workspace --locked` from that commit. It publishes the
   crates in dependency order and waits for each to become available.
4. Check that crates.io lists the version for every crate, and report it.

If a publish fails partway, stop and report which crates were published.
Never yank a version without the maintainer's instruction.
