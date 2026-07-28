---
title: "Como Authoring Contract"
summary: "The harness-agnostic rules for agents authoring content in a Como KB space: page classes, the decision boundary, citations, status vocabularies, and the confidence flow."
status: active
last_updated: "2026-07-28"
---

# Como Authoring Contract

This is the contract an AI agent follows when authoring content in a Como
KB space — regardless of harness. Claude Code, Claude Desktop, or any
MCP-capable client: the engine enforces *shape* deterministically (strict
schemas, admission hooks, lint), and this contract covers the *intent* the
engine cannot check. The packaged skills and harness configs live in
[`kit/`](../../kit/README.md); this page is what they all implement.

The decision of record behind this model is portfolio ADR-0010
(harness-first: the harness is the primary human UI, the KB is the
content product), plan of record
[portfolio#7](https://github.com/como-technologies/portfolio/issues/7).

## The five page classes

Every Como space is provisioned (`spaces create`) with the Como schema
library. Choose the class first — it decides the writer:

| Class | What it holds | Who writes it |
|---|---|---|
| `decision` | An ADR: context, options, outcome, consequences | **adroit only — never the agent directly** |
| `plan` | An implementation plan for a decision | **adroit only** (`plan --save` persists it inside the ADR) |
| `guide` | Step-by-step how-to that turns decisions into practice | the agent, through the engine seams |
| `glossary-entry` | One shared term, defined once | the agent, through the engine seams |
| `worked-example` | A concrete, end-to-end walkthrough with real commands | the agent, through the engine seams |

If content doesn't fit a class, stop and say so — do not force it into
the nearest schema or invent frontmatter keys. Custom types are a
`schema add` decision for a human, not an authoring workaround.

## The decision boundary (non-negotiable)

`decision` pages are owned by adroit: numbering, the `reference` display
identity (`ADR-NNNN`), stored plans, status transitions, and supersession
links are all head-owned. **Never write or edit `decision` frontmatter
directly** — adroit's writer destroys foreign keys and the engine will
refuse the page (this failure mode is evidence-backed; see the kb-spike
gate-5 finding). When the conversation produces a decision:

- In a harness with shell access (Claude Code): drive the adroit CLI —
  `adroit new`, `draft`, `compose`, `set-status`, `plan --save`.
- In an MCP-only harness (Claude Desktop): decisions are read-only until
  adroit's guarded MCP write slice lands (portfolio#7, wave 2). Author
  the surrounding guides and glossary entries; hand the decision text to
  a human or a shell-capable session.

## Born generated, promoted on review

Agent-authored pages declare what they are:

```yaml
status: generated
confidence: 0.3
```

Low declared confidence down-ranks the page in search and makes it
stale-eligible — deliberately. A human reviewer promotes it
(`status: active`, raise or remove `confidence`) when they accept it.
Never author a content page directly to `active`, and never raise your
own confidence: acceptance is a human act. The gates check shape;
truth arrives through this flow, not through the schema.

## Citations pin, links resolve

- **Evidence citations are pinned**: cite evidence files as
  `path@commit` git refs, so the claim stays checkable against the exact
  version it was extracted from, however the files are reorganized
  later. Until the citation link kind lands (llm-wiki#8), pinned refs go
  in a `citations:` frontmatter key, not in typed edge fields.
- **Page links are live**: link pages by slug (`[[decisions/adr-0004]]`,
  typed edge fields) — never pin a page link, and never link to raw
  file paths. Wikilink syntax is plain `[[slug]]` only — there is no
  `[[slug|alias]]` form; an aliased link parses as a broken destination
  (a session-verified gotcha, caught by `broken-link` lint). When the
  prose needs display text, restructure the sentence around the plain
  link instead.
- **No page is born an orphan.** Before ingesting, run `wiki_suggest`
  (shared tags, graph distance, content similarity, community) and link
  what belongs. An orphan lint warning on a page you just authored means
  the job isn't finished.

## Two status vocabularies, one substrate

- Content classes (`guide`, `glossary-entry`, `worked-example`):
  `active | draft | stub | generated` — the engine's lifecycle, and the
  one you author in (see the confidence flow above).
- `decision`: `proposed | accepted | rejected | deprecated | superseded`
  — adroit's lifecycle, transitioned only by adroit, only on human
  instruction.

Both vocabularies carry search weights provisioned into the space's
`wiki.toml`; a superseded decision still resolves but ranks far below
its accepted rival. Don't fight the ranking — fix the status.

## The write flow

The direct-write pattern (see
[writing-content.md](writing-content.md) for the general engine version):

```
1. wiki_content_new  → scaffolds frontmatter, returns the exact path
2. write the body to that path (your harness's file tool)
3. wiki_ingest       → strict validation; a failing page fails loudly
4. wiki_lint         → fix errors; treat fresh-page orphan warnings as unfinished linking
```

Strict ingest bails on the first error with a named rule —
fix-and-rerun, never work around. If validation rejects a page you
believe is right, the schema wins; surface the disagreement to a human
instead of mutating frontmatter until it passes.

## What the gates cannot catch

Schema validation cannot detect confident nonsense. Three habits carry
the truth burden: cite evidence (pinned) for every non-obvious claim,
keep `generated`/low-confidence until a human promotes, and prefer
questions to fabrication when the source material is thin. A page that
says less, honestly, beats a complete-looking page that guesses.
