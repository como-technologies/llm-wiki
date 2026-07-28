---
title: "Stored plan"
type: glossary-entry
status: active
summary: "An implementation plan persisted inside an accepted decision by adroit; reading it back is provider-free and deterministic."
last_updated: 2026-07-28T00:00:00Z
relates_to:
  - glossary/decision-record
  - glossary/accepted
---

An implementation plan persisted into an [[glossary/accepted]]
[[glossary/decision-record]] by `adroit plan <n> --save`, as the
`adroit:plan`-marked `## Implementation` section. Reading it back with
`adroit plan <n> -o json` is provider-free and deterministic and reports
`"stored": true` — the contract downstream automation keys on.
