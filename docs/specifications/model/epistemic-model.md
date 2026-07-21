---
title: "Epistemic Model"
summary: "Why the type field carries epistemic distinctions — what we know, what sources claim, what we concluded."
read_when:
  - Understanding why the type taxonomy exists
  - Deciding which type to assign to a new page
  - Explaining the design philosophy to a new contributor
status: ready
last_updated: "2025-07-17"
---

# Epistemic Model

The `type` field is the epistemic axis. It carries the distinction
between synthesized knowledge, source provenance, and reasoning output.
Folder structure is organizational — defined by the wiki owner, not by
the engine.

## The Three Epistemic Roles

```
concept        → what we know            (synthesized knowledge)
source types   → what each source claims (provenance)
query-result   → what we concluded       (reasoning output)
```

Each role answers a different question. None can substitute for another.

## Concepts — `type: concept`

**What we know — synthesized knowledge.**

One page per concept, continuously enriched across multiple sources. The
canonical answer to "what do we know about X?"

A concept page represents the *current state of knowledge*, synthesized
from all sources. A source page represents what *one document* said.

## Source Types

**What each source claims — provenance.**

One page per source document. Records what a specific paper, blog post,
or transcript claims, with what confidence, and where the gaps are.

Classify by the source material's nature, not its topic. A blog post
about academic research is `article`, not `paper`.

The same concept can be claimed by many sources with different
confidence levels and methodologies. Without source pages, you cannot
ask "which sources support this claim?"

For the full list of source types, see
[type-system.md](type-system.md).

## Query Results — `type: query-result`

**What we concluded — reasoning output.**

Saved conclusions drawn from concepts at a specific point in time, for a
specific question. Keeping them separate prevents conflating:

- "What does the wiki know about MoE?" (concept)
- "Does MoE scale efficiently?" (query-result — a conclusion)

Query results carry their source slugs — which concept and source pages
were used to produce the answer. This makes them auditable.

## Why Type, Not Folder

| Approach | Problem |
|----------|---------|
| Folder = epistemic role | A cooking wiki wants `recipes/`, `techniques/` — not `concepts/` |
| Folder = epistemic role | Type and folder say the same thing — redundant |
| Folder = epistemic role | Engine must enforce folder-type coupling — rigid |

With type as the axis:

- `wiki_list --type concept` works regardless of folder
- `wiki_search --type paper` works regardless of folder
- A wiki can organize by domain and still have clear epistemic
  distinctions via type

The physical layers (`inbox/`, `evidence/`, `wiki/`) are structural, not
epistemic. See [wiki-repository-layout.md](wiki-repository-layout.md).

## Why Separation Matters

The failure mode of naive RAG is collapsing these distinctions:

| Collapsed | Problem |
|-----------|---------|
| Sources merged into concepts | Cannot ask "which source claims this?" — provenance lost |
| Query results merged into concepts | Conclusions presented as facts — reasoning not auditable |
| `evidence/` indexed alongside pages | Unprocessed content pollutes search |

Each type distinction preserves something that matters for knowledge
quality.
