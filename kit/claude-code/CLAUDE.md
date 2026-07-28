# This directory is a Como KB space

You are working inside a knowledge-base space managed by `llm-wiki`
(strict validation, admission hooks, typed pages). Content is authored
through the engine's seams, never by dodging them.

## The rules that bind every session here

- **Follow the Como authoring contract** — `docs/guides/como-authoring.md`
  in the llm-wiki repo. Short version: pick the right page class; author
  content pages `status: generated` with low `confidence` and let humans
  promote; pin evidence citations as `path@commit` in `citations:`; no
  page is born an orphan.
- **The decision boundary**: never write or edit anything under
  `decisions/` directly, and never author `decision` frontmatter. All
  decision work goes through the `adroit` CLI (`new`, `draft`,
  `compose`, `set-status`, `plan --save`, `check`). If adroit is not on
  PATH, decisions are read-only in this session — say so rather than
  improvising.
- **Gates are the workflow, not an obstacle**: `wiki_ingest` after
  writing, `wiki_lint` before declaring done. A validation failure names
  its rule — fix the page, never the process. If you believe the schema
  is wrong, stop and tell the human; schemas change by decision.
- **Answer from the space**: for "what do we know / what was decided"
  questions, search and read the KB (the research skill) and cite pages.
  "The space doesn't record this" is a valid answer; silent invention is
  not.

## Skills

The Como skills in `.claude/skills/` carry the procedures: author-guide,
author-glossary, research, lint-and-fix. Prefer them over ad-hoc flows.
