# ADR-0003: Pin and audit third-party dependencies in CI

> State: Accepted

## Status

Accepted

## Stakeholders

| Role | Name |
|------|------|
| Tech lead (owner) | _your tech lead_ |
| Security owner (consulted) | _whoever owns supply-chain risk_ |
| Engineers (affected) | whole team |

## Context and Problem Statement

Every modern codebase executes far more third-party code than first-party
code, and that code changes on other people's schedules. Two failure modes
follow. **Drift:** unpinned version ranges mean two builds of the same commit
can resolve different dependency trees, so "works on CI" and "works in
production" quietly diverge. **Exposure:** known vulnerabilities (and, less
often, actively malicious releases) enter through routine dependency
resolution, and without an automated check nobody notices until an audit — or
an incident.

Today, pinning and vulnerability review are left to individual judgment per
repo. The cost of that inconsistency lands during incidents and upgrades,
which is the worst possible time to discover what you actually depend on.

## Decision Drivers

- Builds must be reproducible: the same commit resolves the same dependency
  tree, every time, on every machine
- Known-vulnerable dependencies must surface automatically, not via periodic
  manual audits
- The policy must work across language ecosystems (lockfile formats and audit
  tooling differ, but every major ecosystem has both)
- Upgrades should be deliberate, reviewable diffs — routine and small, not
  rare and heroic
- CI cost must stay reasonable: the audit step cannot dominate pipeline time

## Considered Options

1. **Pin everything via lockfiles + an audit gate in CI** — commit the
   ecosystem's lockfile; CI fails on lockfile/manifest drift and on
   known-vulnerable dependencies above an agreed severity threshold.
2. **Floating ranges + periodic manual audits** — accept whatever resolution
   gives, review the tree quarterly.
3. **Vendor everything** — copy dependency source into the repo; maximum
   reproducibility and audit control, maximum upgrade friction.

## Decision Outcome

Chosen: **pin via lockfiles and gate on an automated audit in CI**, because it
buys nearly all of vendoring's reproducibility at a fraction of its friction,
and unlike periodic manual audits it catches vulnerable dependencies when they
*enter*, not months later. Floating ranges optimize for effortless upgrades by
making every build a small gamble; we prefer the gamble to be a reviewed diff.

Policy:

- Every repo commits its ecosystem's lockfile; CI verifies the lockfile is
  in sync with the manifest and installs strictly from it.
- CI runs the ecosystem's vulnerability audit on every merge to the main
  branch; findings at or above the agreed severity threshold fail the build.
- A documented exception path exists (time-boxed waiver recorded in the repo)
  for when a fix is not yet released upstream.
- Upgrades happen on a regular cadence (or via an update bot), as ordinary
  reviewed PRs.

### Positive Consequences

- Reproducible builds: the dependency tree is part of the reviewed history
- Vulnerabilities surface in the PR that would introduce them, with a clear
  failing check, instead of in an annual audit
- Upgrade diffs are visible and bisectable; "what changed?" has an answer
- Incident response improves: the exact tree for any release is knowable

### Negative Consequences

- Upgrade work becomes scheduled labor that someone must own; neglected
  lockfiles age into big-bang upgrades — the cadence is load-bearing
- Audit gates produce false positives and unfixable findings (no patched
  release yet); without a crisp waiver path they train people to bypass CI
- Slightly slower pipelines and one more tool per ecosystem to maintain

## Implementation

1. Inventory ecosystems in use; for each, identify the lockfile mechanism and
   audit tool.
2. Add the lockfile-sync check and audit step to the shared CI configuration,
   warn-only for two weeks to flush existing findings.
3. Agree the severity threshold and the waiver format; document both in a
   guide in this knowledge base.
4. Flip the audit step to blocking; schedule the upgrade cadence (e.g.
   monthly, or enable an update bot) with a named owner.
