# ADR-0001: The repository is github.com/tetracl/tetraglass

Status: Accepted
Date: 2026-11-22
Decision issue: none (decided in a session during the move to the agent workflow)
Raised by: migration to the agent workflow

## Context

D-017 and design section 12 name the repository `github.com/Tetraglass/tetraglass`,
and every crate's `repository` field, the mdBook settings and the Blender
add-on metadata use that URL. The repository's `origin` is
`github.com/tetracl/tetraglass`; GitHub redirects the old URL to it.
Releases 0.1.0 to 0.3.0 on crates.io carry the old URL.

## Options

1. Keep the design as written. Metadata keeps pointing at
   `github.com/Tetraglass/tetraglass` and depends on GitHub's redirect, which
   breaks if a repository of that name is ever created again.
2. Use `github.com/tetracl/tetraglass` everywhere. Published releases keep the
   old URL; the next release carries the new one.

## Recommendation

Option 2.

## Changes if accepted

- Design sections: 12 (Name) and 13 (Open questions), marked `(ADR-0001)`.
- Supersedes the repository part of D-017; the name and crate prefix stand.
- Files: `Cargo.toml` (`repository`), `docs/book/book.toml`,
  `blender/__init__.py`, `blender/blender_manifest.toml`.
- Interfaces and tests: none.
- Plans and tasks affected: none.

## Decision

> tetracl/tetraglass is correct, 2026-11-22, said to the agent in a session.
