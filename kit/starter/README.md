# Starter content

The content a fresh Como KB space starts from — so day one begins with a
working corpus, not a blank page. Distilled from a delivered client
engagement (content written fresh; no client material), formerly shipped
as a clonable template repository; the template retired when the
portfolio went harness-first (portfolio ADR-0010 — the archived repo
remains the historical record).

## What's here

```
decisions/            legacy-format ADR corpus, seeded via `adroit seed`
  accepted/           5 worked engineering decisions (trunk-based
                      development, ADR governance, dependency pinning
                      and audit, a shared glossary, automated testing —
                      3 carrying stored implementation plans)
  proposed/           an 11-record starter backlog (CI, code reviews,
                      refactoring, testing, monitoring, incident
                      management, feature flags, runbooks, secrets,
                      library versioning)
wiki/
  glossary/           10 typed glossary-entry pages — the decision
                      lifecycle vocabulary, defined once
  guides/             the ADR review process, as a typed guide page
```

The starter decisions are examples a team keeps, supersedes, or replaces
— superseding a starter opinion *is* the process working. Nothing in
them assumes a vendor, cloud, language, or CI system.

## Bootstrap a space with it

```sh
export LLM_WIKI_CONFIG="$DIR.registry.toml"   # ephemeral space => scoped registry;
                                              # rm both together, no global litter
llm-wiki spaces create "$DIR" --name myteam --set-default
adroit seed --from kit/starter/decisions --dir "$DIR"   # decisions: fresh ULIDs, healed links
cp -r kit/starter/wiki/. "$DIR/wiki/"                   # glossary + guides (typed pages)
llm-wiki ingest . --wiki myteam                         # admit through the strict gate
adroit check --dir "$DIR"                               # the semantic gate
```

Decisions ship in adroit's legacy bootstrap format on purpose: `seed`
allocates each space its own page identities and heals the corpus's
internal links, so two teams' spaces never share baked-in IDs. The
glossary and guide pages ship as typed pages and go straight through
ingest. Rehearsed on every change to this content: the sequence above
ends with `check` clean and `lint` at **zero errors** (orphan advisories
on not-yet-linked decisions are expected and are the lint doing its
job — linking them into your own pages is the point of the starter).

Then configure your harness ([kit/README.md](../README.md)) and author —
the starter corpus is the floor, not the ceiling.
