# ADR-0015: Gate incomplete work behind feature flags with owners and expiry dates

> State: Proposed

## Status

Proposed
Created: 2026-06-12

## Stakeholders

Tech lead (owner), engineers (apply the convention daily), product owner
(decides when flagged features turn on for users).

## Context and Problem Statement

[ADR-0001](decisions/0001-adopt-trunk-based-development-with-short-lived-branches)
commits the team to trunk-based development and names its price: incomplete
work must land on the main branch *dark*, behind feature flags — and "flags
are debt: each needs an owner and a removal date, or the codebase fills with
dead toggles." That debt clause is currently a warning, not a policy. Teams
that adopt flags without a lifecycle rule reliably accumulate them: flags
whose owner has left, flags that have been on for a year, flags whose two
sides can no longer both compile. Each stale flag doubles a code path that
tests must cover and readers must understand.

We need an explicit convention for how a flag is born, who answers for it,
and when it dies — settled now, while the flag count is still small.

## Decision Drivers

- ADR-0001's branching model is already accepted; its flag discipline must
  be real for the model to keep working
- Every flag is a live fork in the code: the count must trend toward zero,
  not grow monotonically
- The convention must work with any flag mechanism — an env var, a config
  file, a flag service — because the knowledge base prescribes process, not tools
- Removal must be the default outcome, requiring no heroics or archaeology

## Considered Options

1. **Flags with mandatory metadata and expiry** — every flag is registered
   (a simple table in the repo is enough) with an owner, a purpose, and an
   expiry date; CI or a recurring review flags entries past expiry; the
   owner either removes the flag or renews it with a written reason.
2. **Flags as unmanaged convention** — keep using flags ad hoc; trust
   engineers to clean up. Cheapest today; this is the option that produces
   flag graveyards, because removal is nobody's job.
3. **No feature flags — only merge complete work** — eliminates flag debt
   by reintroducing long-lived branches, directly contradicting ADR-0001's
   accepted trade-off.

## Decision Outcome

Chosen: **flags with mandatory metadata and expiry**, because it keeps
ADR-0001's promise honest at the lowest sustainable cost. The register
makes flag debt visible and assigns it an owner; the expiry date makes
removal the default rather than an act of initiative.

Concretely:

- A flag may not ship without a register entry: name, owner, purpose,
  created date, expiry date (default: 90 days out).
- The register lives in the repository next to the code it describes, so
  a flag and its entry change in the same review.
- Past-expiry flags are surfaced automatically (a CI warning or a standing
  review agenda item — pick the cheapest mechanism your stack offers).
- Renewal is allowed but never silent: it updates the expiry and records
  why in the entry.
- Removing a flag removes both code paths and the entry in one change.

### Positive Consequences

- Flag debt becomes a visible, owned, bounded list instead of folklore
- Dead toggles stop accumulating; test matrices stay close to reality
- ADR-0001's "ship dark behind a flag" instruction now has a worked answer
  to "and then what?"

### Negative Consequences

- Registering and renewing flags is process overhead on every flagged
  change, and the team must resist registering flags retroactively
- A register that is not enforced (no CI check, no review cadence) decays
  into exactly the folklore it was meant to replace — the enforcement
  mechanism must be chosen and actually wired in during rollout
- Expiry pressure can tempt premature flag removal; renewals must be cheap
  enough that the honest move is easy

## Rollout

1. Agree the register format and location (one table, in-repo) and the
   default expiry window.
2. Backfill entries for every flag currently in the codebase — unknown
   owners become the tech lead's to reassign.
3. Wire the past-expiry surface (CI warning or recurring review item).
4. After one quarter, count flags removed vs. created and record findings
   in a follow-up to this ADR.
