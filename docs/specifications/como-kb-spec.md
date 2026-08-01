---
title: "The Como KB specification"
summary: "The Como knowledge base contract — typed pages, admission model, anti-rot, and the substrate/head seam — layered on the llm-wiki engine."
read_when:
  - Understanding how the Como portfolio tools use llm-wiki as their substrate
  - Checking the admission model, page-type contracts, or the substrate/head split
status: ready
last_updated: "2026-08-01"
---

# The Como KB specification

The knowledge base that makes the portfolio's prescriptions evidenced:
typed pages, stable identity, mechanical anti-rot, and a git-native
admission model. Produced by
the [kb-spike](https://github.com/como-technologies/kb-spike) evaluation
(ten findings, each closed with live-verified evidence) and adopted by
portfolio
[ADR-0006](https://github.com/como-technologies/portfolio/blob/main/docs/src/adr/accepted/0006-adopt-llm-wiki-engine-como-fork-as-the-knowledge-base-substrate.md).
Substrate: `llm-wiki` — this repo, the Como KB product (portfolio
[ADR-0008](https://github.com/como-technologies/portfolio/blob/main/docs/src/adr/accepted/0008-build-the-kb-product-in-the-fork-itself-developing-on-main.md)):
a fork of `llm-wiki-engine` developed on `main`. At this stage instances are
**ephemeral and built from source at HEAD of `main`** (portfolio
[ADR-0009](https://github.com/como-technologies/portfolio/blob/main/docs/src/adr/accepted/0009-iterate-ephemeral-first-disposable-kb-instances-built-from-source-at-head.md));
release-tag pinning returns when a persistent KB is declared. This document
is the spec's interim home — once a persistent production KB exists, the
spec migrates into it as typed pages and this document becomes a
projection.

---

## Part I — How content moves

The mental model: **git history is the admission log. Validation is the
pre-commit constraint. Everything downstream — the indexer, the LLM
librarian — is a consumer of that log with a per-instance cursor, catching
up idempotently.** The engine does zero inference; the librarian is a head.
The indexer ships today; the librarian is future state, specified here and
not yet built — every consumer that runs today is mechanical.

### Admission: two paths, one gate

- **Structured writers (the normal case).** Every portfolio tool — adroit,
  tuesday, pulse — owns its page type(s) and writes typed pages straight
  into `wiki/`, then commits. No LLM in the admission path; tools never
  block on a model. The gate is the pre-commit hook: strict schema
  validation — **a failing page fails the commit**, so invalid data never
  enters history. One substrate: the ADR corpus lives *in* the space, and
  adroit operates on it there.
- **Capture (`evidence/`).** Unstructured material — conversation exports,
  assessment dumps, third-party docs — lands as files and commits. Bulk
  load is just many files in one commit: one atomic transaction, no
  debouncing, no half-written-file edge cases.

### Downstream consumers

Each consumer records "processed up to commit X" as a git ref
(`refs/kb/<consumer>/<instance>`) and catches up to HEAD when prompted — a
post-commit hook, the next engine command, or an explicit catch-up. Failure
just leaves the cursor behind; any later prompt retries. Index failure is
lag, never loss.

1. **The indexer** (mechanical, fast): index the committed delta, advance
   `refs/kb/index/<instance>`.
2. **The librarian** (semantic, batched) — two triggers, one loop:
   - **Enhancement policy, keyed on page type**: a new `measure-report`
     page arrives → policy says link concepts, refresh trend pages, check
     for contradictions against accepted decisions.
   - **Evidence processing**: work the uncited-evidence queue — an evidence
     file is *processed* when a wiki page cites it (a query, not a
     directory). Classify → extract claims → **reconcile** → author typed
     pages, each citing its sources as **pinned `path@commit` git refs**:
     the page extracted claims from *that version*, and the blob stays
     resolvable however tools reorganize evidence later.

### Reconcile is the ballgame

Before writing, the librarian queries the wiki (search + graph over MCP):
does this concept already exist (merge by id, not near-duplicate)? does a
claim contradict an accepted decision (surface it; perhaps propose
`supersedes`)? what should this page link to (never born an orphan)?
Contradiction and supersession judgments queue for human review; routine
extract→type→link runs unattended. The librarian's output goes through the
same admission gate as any tool — propose-then-verify, where the verifier is
deterministic (strict schema + lint), not another model.

### Invariants

- **Append-only history is law** — no rebase, no force-push, ever. It is
  what makes pinned citations and replay sound
  ([kb-spike#11](https://github.com/como-technologies/kb-spike/issues/11)
  tracks enforcement and the replay harness).
- **Idempotency** — pages upsert by declared id; consumers reprocess safely
  by cursor.
- **Confidence flow** — librarian pages are born `generated` with low
  `confidence` and promoted on review. Declared-low is down-ranked in
  search and stale-eligible; absent confidence is search-neutral and never
  stale — exactly the born-low → promoted lifecycle.
- **Replay** — resolve a page's pinned citations, re-run the pipeline on
  those exact blobs, diff the result.

---

## Part II — The contracts

Each contract cites its evidence in the spike's
[findings](https://github.com/como-technologies/kb-spike/tree/main/findings).

### 1. Identity & links

- A page carries a stable opaque **id**: a ULID, always tool-generated,
  never hand-authored.
- **Links resolve by id** — body `[[wikilinks]]` and typed edge fields
  alike; a page may be reorganized on disk with **zero link rewrites**.
  Slug/path is presentation and addressing convenience, not identity.
- **Id uniqueness is engine-enforced**: `duplicate-id` at error severity
  (CI-gating), `id-format` at warning. Resolution is slug-first, id-second;
  ids are opt-in by presence.
- Convention on top: immutable flat slugs for ADR paths
  (`decisions/adr-NNNN`, status in frontmatter only) — a naming convention,
  not a load-bearing constraint.
- Normative text: [`page-identity.md`](model/page-identity.md).
  Evidence: findings/issue-01.

### 2. Page types & schemas

- The six target classes — `decision`, `guide`, `glossary-entry`,
  `worked-example`, `plan`, and `measure-report` (the Measure heads'
  class, joined at portfolio#7 wave 4) — **ship in-engine as the Como
  schema library**
  (llm-wiki#14): `spaces create` installs them alongside the bundled types.
  Further custom types remain a `schema add` away, one JSON Schema per type.
- The `decision` type is derived from adroit's `Adr`/`Status` model
  (authored in the spike at `kb-spike/schemas/decision.json`, now moved
  into this repo's `schemas/decision.json`).
- Custom types set **`additionalProperties: false`** — unknown keys fail
  loudly — and every field a head writes (e.g. adroit's `reference`) is
  declared.

### 3. Validation & strictness

- `validation.type_strictness = "strict"`, always — `spaces create` writes
  it into the generated `wiki.toml` (llm-wiki#14), so the contract travels
  with the space.
- **The CI gate**: every frontmatter violation class — missing required
  field, unknown type, out-of-enum value, failed `if/then` conditional,
  unknown key — fails `ingest` with exit 1 and a named rule. `lint` exits 1
  on errors, 0 on warnings-only: errors gate, warnings advise. Strict
  ingest bails on the first error (fix-and-rerun semantics).
- **Registry integrity is engine-enforced**: a corrupt
  `schemas/*.json` refuses to mount the wiki, every command exits 1 naming
  the broken file. Evidence: findings/issue-02.

### 4. Status vocabulary

- **Two vocabularies, one substrate.** `decision` uses the adroit-aligned
  lifecycle `proposed | accepted | rejected | deprecated | superseded`;
  content types reuse the engine's `{active, draft, stub, generated}`.
- **State-coupled requirements live in the type schema**:
  `superseded ⇒ superseded_by` via `if/then` fails ingest when violated.
- **`[search.status]` carries both vocabularies** — custom keys rank
  exactly as configured (a superseded page scores 0.30× its accepted
  rival at the recommended weights); `spaces create` provisions both
  vocabularies into the space's `wiki.toml` (llm-wiki#14), and
  `config set search.status.<key>` adjusts them. Evidence: findings/issue-03.
- Resolved at the adroit retrofit (adroit ADR-0020): frontmatter is the
  sole source of truth — the body is prose only, with no `## Status`
  section; `status` serializes lowercase per the schema enum; the page
  persists the ULID `id` and the head-owned `reference`.

### 5. Anti-rot / lint

- **`decision`: rot is structural, never temporal.** CI gates on strict
  ingest + lint errors (broken-link, duplicate-id, missing-fields,
  unknown-type); orphan stays advisory; review-due for `proposed` is
  head-side. Contradiction detection is semantic → the librarian's
  reconcile step, not deterministic lint.
- **Staleness fires only on explicitly low-confidence pages** (intentional
  semantics): `stale` requires age AND declared
  `confidence < 0.4`. Decisions never declare confidence and are un-stale
  by design; guides get temporal staleness through the confidence flow
  (Part I). Evidence: findings/issue-04.

### 6. Substrate vs head

- **Numbering/sequence is the head's job.** The substrate stores exactly
  two identity fields per decision: `id` (stable ULID routing identity) and
  `reference` (head-owned display identity, `ADR-0006` — no resolution
  semantics). adroit owns allocation, gap/collision detection, and scheme
  choice. The engine never grows ADR-specific features.
- **The read contract**: typed pages addressable by ULID with full-fidelity
  `content read` (CLI and MCP both), plus the `export` machine seam. Heads
  own derived views — numbering/addressing, review-due, plan extraction
  from the marked `## Implementation` region, forge enrichment. The former
  export gap is closed:
  [llm-wiki#11](https://github.com/como-technologies/llm-wiki/issues/11)
  landed, so `export --format json` carries custom frontmatter and summary
  rows need no per-page reads. Evidence: findings/issue-10 (seam
  map); the numbering finding lives on
  [kb-spike#6](https://github.com/como-technologies/kb-spike/issues/6) itself.

### 7. Admission & evidence

- **The file is the API; a git commit is the unit of admission** — the
  transaction semantics of Part I. The two git hooks (`ingest --dry-run`
  pre-commit; `ingest` post-commit) and catch-up-on-read
  (`index.auto_rebuild`) are installed by `spaces create` (llm-wiki#14);
  hooks fire only for real `git` commits, never the engine's own libgit2
  commits, so the chain terminates by construction.
- **`evidence/` is the capture layer** (renaming the engine's `raw/`,
  [llm-wiki#10](https://github.com/como-technologies/llm-wiki/issues/10)):
  unstructured material only. Citations are **always pinned `path@commit`**;
  live references are for pages (ids), never evidence. Processed = cited;
  `inbox/` is not part of the contract. Interim rule until the citation
  link kind lands
  ([llm-wiki#8](https://github.com/como-technologies/llm-wiki/issues/8)):
  pinned refs live in a non-edge frontmatter key (`citations:`).
- Evidence: findings/issue-09 (including the hand-walked capture → cite →
  page round trip, with pinned resolution surviving reorganization).

### 8. Architecture: three layers
(portfolio [ADR-0008](https://github.com/como-technologies/portfolio/blob/main/docs/src/adr/accepted/0008-build-the-kb-product-in-the-fork-itself-developing-on-main.md))

- **`llm-wiki`** (this repo): the Como KB product — engine,
  Como schema library (the kb-spec §2 classes, shipped), provisioning
  (hooks, strict defaults, search weights — shipped), and ops, all developed
  on `main`, the only branch. **Pinning = HEAD of `main`, built from source**
  at this stage (portfolio
  [ADR-0009](https://github.com/como-technologies/portfolio/blob/main/docs/src/adr/accepted/0009-iterate-ephemeral-first-disposable-kb-instances-built-from-source-at-head.md));
  release tags return with the first persistent instance. The `upstream`
  remote exists for opportunistic cherry-picks; no discipline is owed to it.
  The spec's substrate-neutral contracts (above) are what keep the KB
  replaceable — not repo topology.
- **KB instances**: near-pure data spaces (`wiki/`, `evidence/`, `schemas/`
  as installed) created and managed by `llm-wiki`. At this stage instances
  are **ephemeral** (portfolio ADR-0009): stood up per-checkout by `just
  init`, derived from canonical content that stays in its repo of record,
  and destroyed freely. Como's first persistent KB is a deferred,
  deliberate decision.
- **The heads**: adroit, tuesday, pulse, and the librarian. Structured
  writers in; seam readers out — always against instances, via the seams.
  Current state, plainly: adroit is retrofitted and KB-only today (its
  ADR-0020); tuesday emits `measure-report` pages today
  (`tuesday-report --kb`); pulse's emitter arrives when it un-parks (its
  ADR-0010 sets that bar); the librarian is future state.

Open product work:
[llm-wiki issues](https://github.com/como-technologies/llm-wiki/issues).
Future work (tracked in
[portfolio#6](https://github.com/como-technologies/portfolio/issues/6) and
the llm-wiki backlog): append-only enforcement + replay; snapshot-based
index sync.
