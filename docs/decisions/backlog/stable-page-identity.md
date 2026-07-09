---
title: "Stable page identity via frontmatter ULIDs"
summary: "Add an optional, opt-in stable page id (ULID) so links can survive file moves; slug-first resolution keeps all existing behavior."
status: proposed
date: "2026-07-09"
---

# Stable page identity via frontmatter ULIDs

## Context

Page identity was the on-disk path: slug = path = address, and both
`[[wikilinks]]` and typed edge fields (`superseded_by`, `sources`, …)
store slugs. Moving or renaming a file broke every inbound link — `lint`
caught it as a `broken-link` error, but nothing could resolve or rewrite
it, and no move/rename/alias command existed. For a git-backed wiki
meant to live for years and be reorganized, every reorg was a link-rot
event: the anti-rot contract fought the filesystem.

A downstream evaluation (como-technologies/kb-spike#1) confirmed this
live against v0.4.1 and needed a path-independent identity contract:
a page has a stable opaque id, links resolve by id, and pages can be
reorganized with zero link rewrites.

## Decision

Add an **optional `id` frontmatter field — always a ULID, always
tool-generated** — indexed alongside the slug, with **slug-first,
id-second** resolution at every place an address or link target is
accepted.

Key choices:

- **Additive, presence-based opt-in.** No config keys. A wiki without
  ids behaves byte-for-byte as before (JSON fields are omitted, not
  null). Slug links keep working forever.
- **Strict ULID, no hand-authored formats.** Human-meaningful ids would
  recreate the path/meaning coupling this exists to remove. A ULID can
  also never collide with a real slug in practice, and slug-first order
  makes any theoretical overlap deterministic.
- **Resolution through the index**, disk-verified, with an explicit
  stale-index error rather than a silent miss. `Slug::resolve` stays a
  pure filesystem probe; the id hook lives in
  `EngineState::resolve_address` where the index is available.
- **Uniqueness becomes an engine guarantee** (`duplicate-id` lint error)
  since the filesystem's one-file-per-path no longer provides it.
- **Emitted URIs stay slug-based.** Ids are accepted as input everywhere
  and surfaced as a separate output field.

## Consequences

- `git mv` a page and its id links keep resolving after re-index; only
  slug links to the old path dangle.
- New lint rules `duplicate-id` (error) and `id-format` (warning).
- The graph snapshot name was bumped (`wiki-graph-v2`) because
  `PageNode` gained the id field and old bincode snapshots are
  layout-incompatible; one cold rebuild per space on upgrade.
- Schema files gained an `id` property, which changes the schema hash
  and triggers one automatic partial rebuild on first mount.

See [specifications/model/page-identity.md](../../specifications/model/page-identity.md)
for the full contract.
