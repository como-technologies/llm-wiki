---
title: "Page Identity"
summary: "Stable, path-independent page ids — declaration, resolution order, uniqueness, and how id links survive file moves."
read_when:
  - Deciding how pages should reference each other
  - Reorganizing wiki files without breaking inbound links
  - Understanding how an id address resolves to a page
status: ready
last_updated: "2026-07-09"
---

# Page Identity

A page's default identity is its **slug** — the wiki-root-relative path
without extension (see [page-content.md](page-content.md)). A slug is a
good address but a fragile identity: moving or renaming the file changes
the slug, and every inbound link to the old slug dangles (`lint` reports
`broken-link` at error severity).

A page may therefore declare a second, **stable identity** in
frontmatter:

```yaml
---
id: 01ARZ3NDEKTSV4RRFFQ69G5FAV
title: "Use Postgres"
type: doc
---
```

## The id contract

- The `id` value is a **ULID** — 26 characters of Crockford base32,
  canonical uppercase (`^[0-9A-HJKMNP-TV-Z]{26}$`). Ids are opaque and
  tool-generated (`content new --id`, MCP `wiki_content_new` with
  `auto_id`), never hand-authored or meaningful.
- An id is **stable across file moves and renames**. Reorganizing pages
  on disk requires zero link rewrites when links target ids.
- Ids must be **unique within a space**. The filesystem no longer
  guarantees uniqueness once identity is declared, so the engine does:
  `lint` reports `duplicate-id` at error severity.
- Everything is **opt-in by presence** — a wiki with no `id` frontmatter
  behaves exactly as before, byte-for-byte, including JSON output shapes.

## Resolution order

Wherever a page address or link target is accepted — `wiki://` URIs,
bare addresses, `[[wikilinks]]`, and typed frontmatter edge fields
declared via `x-graph-edges` — resolution is:

1. **Exact slug match first.** If the address resolves to a file on
   disk, that page wins. A slug always shadows an id with the same
   spelling, so existing slug links behave exactly as before.
2. **Id lookup second.** If the token parses as a ULID, the search index
   maps it to the declaring page's slug, verified against disk.

Parsing is case-insensitive (`01arz…` resolves), but the index stores
and queries the canonical uppercase form. If the index maps an id to a
file that no longer exists, resolution fails with an explicit
stale-index error advising `llm-wiki index rebuild`. Under duplicate ids
resolution deterministically picks the lexicographically smallest slug
(and `lint` flags the duplication).

## Where ids appear

- **Inputs:** `content read`/`write`, `history`, `suggest`, `lint`
  link targets, graph edge targets, MCP `wiki_resolve`,
  `wiki_content_read`, and MCP resource reads all accept ids.
- **Outputs:** `search`, `list`, `export` entries, graph nodes, and MCP
  `wiki_resolve`/`wiki_content_new` responses carry an `id` field when
  the page declares one; the field is omitted otherwise.
- **Emitted URIs stay slug-based** (`wiki://<space>/<slug>`);
  `wiki://<space>/<id>` is accepted as input but never emitted.
- **Sections do not carry ids** — they are structural, addressed by
  their place in the tree.

## Lint rules

| Rule | Severity | Meaning |
|------|----------|---------|
| `duplicate-id` | error | Two or more pages declare the same id |
| `id-format` | warning | A declared id is not a valid ULID and can never be a link target |

A page whose only inbound links are id links is **not** an orphan.

## When to use id links

Use id links (`superseded_by: <ulid>`, `[[<ulid>]]`) for pages that are
expected to be reorganized — guides, glossary entries, anything whose
"right place" evolves. Slug links remain fine for immutable, flat
namespaces where the path is the contract (e.g. `decisions/adr-0001`).
