# ADR-0018: Version shared internal libraries with semantic versioning and changelogs

> State: Proposed

## Status

Proposed
Created: 2026-06-12

## Stakeholders

Library maintainers (publish versions and changelogs), consuming teams
(plan upgrades against them), tech lead (owns the convention).

## Context and Problem Statement

The moment a second team depends on an internal library, every change to
it is a contract negotiation — but today the contract is unwritten.
Consumers learn about breaking changes when their build fails, upgrade
notes live in commit messages nobody reads, and "what changed between the
version we run and HEAD?" requires a code archaeologist. The result is
predictable: consumers pin to old revisions and stop upgrading, the
library's maintainers lose their audience, and the shared code the library
exists to deduplicate gets quietly copied instead.

[ADR-0003](decisions/0003-pin-and-audit-third-party-dependencies-in-ci)
already requires every dependency to be pinned and auditable — which makes
*upgrading* the routine act. Internal libraries currently give upgraders
nothing to reason with. We need a versioning and change-communication
convention for code we share with ourselves.

## Decision Drivers

- A consumer must be able to judge upgrade risk from the version number
  alone, before reading a single diff
- Breaking changes must be loud, deliberate, and documented — never an
  ambient side effect of pulling latest
- The convention must be cheap enough that maintainers keep it on every
  release, not just the big ones
- It must compose with ADR-0003: pinned versions only help if there are
  meaningful versions to pin
- Language-ecosystem neutral: the rule is about numbers and notes, not
  about any particular registry or package manager

## Considered Options

1. **Semantic versioning + a kept changelog, tagged at the repository** —
   every release gets a `MAJOR.MINOR.PATCH` version where MAJOR signals
   breaking change, plus a human-written changelog entry (added/changed/
   fixed/removed, with migration notes for anything breaking); releases
   are git tags; consumers pin exact versions per ADR-0003.
2. **Live at HEAD** — all consumers track the latest revision; breakage is
   caught by the consumers' CI and fixed forward. Works inside a single
   well-tooled monorepo with atomic cross-cutting changes; without that
   tooling it just distributes the library's instability to every
   consumer's pipeline.
3. **Calendar versioning + changelog** — versions like `2026.06` are
   honest about *when* but silent about *how risky*; the one question an
   upgrader actually asks (will this break me?) goes unanswered.

## Decision Outcome

Chosen: **semantic versioning + a kept changelog**, because it encodes the
upgrader's risk question into the version number itself and pairs it with
the prose a migration actually needs. It is the convention the rest of the
ecosystem already speaks, so it costs the least to teach.

Concretely:

- Shared libraries release as git tags `vMAJOR.MINOR.PATCH`; anything that
  forces a consumer code change bumps MAJOR, new backward-compatible
  capability bumps MINOR, fixes bump PATCH.
- Every release appends a changelog entry; breaking entries include a
  migration note ("replace X with Y").
- Pre-1.0 libraries are explicitly experimental: minor bumps may break,
  and the changelog says so.
- Consumers pin exact versions (per ADR-0003) and upgrade deliberately;
  "deprecate, then remove one MAJOR later" is the polite path for retiring
  an API.

### Positive Consequences

- Upgrade risk is legible at a glance; routine PATCH/MINOR upgrades can
  flow with minimal ceremony while MAJOR gets real attention
- The changelog becomes the library's communication channel, replacing
  tribal release announcements
- Deprecation gains a working protocol instead of a surprise

### Negative Consequences

- Versioning discipline is judgment under pressure: the temptation to
  smuggle a small breaking change into a MINOR bump never goes away, and
  one such smuggle damages trust in the whole scheme
- Changelog writing is a manual tax on every release; generated-from-
  commits changelogs are cheaper but answer "what happened" rather than
  "what do I do"
- Old MAJOR versions invite support requests; the team must state how
  long (if at all) previous majors receive fixes

## Rollout

1. Inventory shared internal libraries; give each a current version tag
   and a changelog seeded with "everything before this tag is prehistory".
2. Document the bump rules and the migration-note requirement in the
   library template/contributing notes.
3. Add version-tag + changelog-entry checks to the libraries' release
   steps where the tooling makes it cheap.
4. Revisit after two release cycles: are consumers upgrading? Record
   findings in a follow-up to this ADR.
