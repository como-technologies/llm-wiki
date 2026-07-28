# Worked example — the first captured authoring session

Captured 2026-07-28 from a real session: Claude Code as the harness,
`llm-wiki` at HEAD (built from source), `adroit` 0.2.0 (KB-only build).
This is the evidence behind the kit (portfolio#7, wave 1): one guide, two
glossary entries, and one decision authored conversationally into a fresh
space, every write behind the deterministic gates. Outputs are verbatim,
trimmed only of timestamps and long paths (`$WORK` = a scratch directory).

## 1. Provision — one command, zero flags

```console
$ export LLM_WIKI_CONFIG=$WORK/registry.toml       # scoped registry; nothing global touched
$ llm-wiki spaces create $WORK/demo-space --name demo --set-default
Created wiki "demo" at $WORK/demo-space
Registered in $WORK/registry.toml
Initial commit: create: demo
```

Provisioning verified: `wiki.toml` carries `type_strictness = "strict"`,
`.git/hooks/` has `pre-commit` + `post-commit`, and `schema list` shows
the Como classes (`decision`, `guide`, `glossary-entry`, `plan`,
`worked-example`) alongside the bundled types.

## 2. Scaffold with stable identity

```console
$ llm-wiki content new guides/stand-up-an-ephemeral-space \
    --name "Stand up an ephemeral KB space" --type guide --id
Created: wiki://demo/guides/stand-up-an-ephemeral-space (id: 01KYMVQ7FT78EF1YDQHZ5VYR17)
$ llm-wiki content new glossary/ephemeral-space --name "Ephemeral space" --type glossary-entry --id
Created: wiki://demo/glossary/ephemeral-space (id: 01KYMVQ7G0K47X9G4X1WQE6FFJ)
$ llm-wiki content new glossary/admission-gate --name "Admission gate" --type glossary-entry --id
Created: wiki://demo/glossary/admission-gate (id: 01KYMVQ7G6N6D8A8XVVC16AV4M)
```

The agent then wrote each body to the scaffolded path per the contract:
`status: generated`, `confidence: 0.3`, a one-line `summary`, and
`relates_to` links — content pages are born declaring what they are.

## 3. The gate catches what it should

The guide's first draft deliberately carried a frontmatter key outside
the schema (`category: operations`):

```console
$ llm-wiki ingest guides/stand-up-an-ephemeral-space.md --wiki demo
Error: schema validation failed: Additional properties are not allowed ('category' was unexpected)
exit: 1
```

Named rule, exit 1, page refused — `additionalProperties: false` doing
its job against an AI author. Key removed, re-ingested clean:

```console
$ llm-wiki ingest guides/stand-up-an-ephemeral-space.md --wiki demo
Ingested: 1 pages, 0 unchanged, 0 assets, 0 warnings, 0 redactions
Commit: 46ab262cc8dfadf9a9db0435af93154a3a62d83b
```

## 4. Lint caught a second, unplanned mistake

The agent had used `[[slug|alias]]` pipe-aliased wikilinks — not engine
syntax; the whole aliased string parses as a destination:

```console
$ llm-wiki lint --wiki demo
[error] guides/stand-up-an-ephemeral-space — broken link in body_links: glossary/admission-gate|admission hooks (broken-link)
[error] guides/stand-up-an-ephemeral-space — broken link in body_links: glossary/ephemeral-space|ephemeral space (broken-link)
```

Fixed to plain `[[slug]]` links, re-ingested, and the finding was fed
back into the kit (the contract guide and the author-guide skill now
state the rule). This is the loop working: the gates teach, the
instructions accumulate.

## 5. The decision routes through adroit — same space, same gates

```console
$ export ADROIT_DIR=$WORK/demo-space               # the space root; adroit is KB-only
$ adroit new "Author demo content only in disposable spaces" --no-edit
Created $WORK/demo-space/wiki/decisions/0001-author-demo-content-only-in-disposable-spaces.md
```

adroit owns the frontmatter (ULID `id`, `reference: ADR-0001`, lowercase
`status: proposed`); the body sections were filled as prose, frontmatter
untouched. Both gates pass, and the engine admits adroit's page —
one substrate, two writers, no translation:

```console
$ adroit check
OK: 1 ADRs, no problems
$ llm-wiki ingest . --wiki demo
Ingested: 6 pages, 0 unchanged, 1 assets, 0 warnings, 0 redactions
Commit: a489278b214df387d5a30c23017feca4ce8ca05c
```

The decision stays `proposed`: acceptance is a human act, and no human
accepted it in this session. That is the contract, not a gap.

## 6. Close the loop — link, lint, read back

The guide's `relates_to` gained the decision's slug (a guide
operationalizes a decision), then the final gate and the read side:

```console
$ llm-wiki lint --wiki demo
3 finding(s): 0 error(s), 3 warning(s)      # see the honest note below

$ llm-wiki search "disposable demo space" --wiki demo
slug:  decisions/0001-author-demo-content-only-in-disposable-spaces
title: Author demo content only in disposable spaces
score: 3.95

$ adroit show 1 -o json | jq -r '"\(.reference) | \(.title) | \(.status)"'
ADR-0001 | Author demo content only in disposable spaces | proposed
```

## The honest ledger

- **What the gates caught**: one deliberate schema violation (named
  rule, commit refused) and one real authoring mistake (aliased
  wikilinks → `broken-link` errors). Both fixed through the process,
  never around it.
- **What fed back into the kit**: the wikilink syntax rule (contract
  guide + author-guide skill updated in this same wave).
- **What we filed instead of hiding**: the three remaining warnings are
  `stale` findings on pages authored *minutes earlier* — the age half
  of the stale rule (age AND low confidence) appears absent, which
  would train authors to ignore the born-generated contract. Filed as
  [llm-wiki#15](https://github.com/como-technologies/llm-wiki/issues/15).
- **What this proves**: provision → scaffold → author → gate → link →
  read, all from one harness conversation, with the decision boundary
  held (adroit wrote the decision; the agent never touched `decisions/`
  frontmatter) and the space destroyed afterward — the transcript, not
  the space, is the artifact of record.
