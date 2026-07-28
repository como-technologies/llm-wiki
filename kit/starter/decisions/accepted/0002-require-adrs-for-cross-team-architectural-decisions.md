# ADR-0002: Require ADRs for cross-team architectural decisions

> State: Accepted

## Status

Accepted

## Stakeholders

| Role | Name |
|------|------|
| Engineering leads (owners) | leads of each affected team |
| Architects / staff engineers (consulted) | _if applicable_ |
| Engineers (affected) | all teams in scope |

## Context and Problem Statement

Decisions that cross team boundaries — shared schemas, service contracts,
platform choices, deprecations another team must absorb — are exactly the ones
most likely to be made informally: in a meeting two teams attended, a chat
thread one team saw, or a document that lives in one team's folder. Months
later the decision is load-bearing, the rationale is folklore, and the teams
involved remember different versions of it.

Within a single team, informal decision-making is often fine — the people who
decided are the people who maintain. Across teams that symmetry breaks: the
deciders and the affected are different groups, and the affected need a
record they can find, challenge, and build against.

## Decision Drivers

- Cross-team decisions must be discoverable by people who weren't in the room
- The rationale and rejected alternatives matter as much as the outcome —
  they prevent re-litigating settled ground without new information
- The process must be lightweight enough that teams don't route around it
- Review must include every affected team, not just the proposing one
- There must be a single, unambiguous home for the record

## Considered Options

1. **Require an ADR in this corpus for every cross-team architectural
   decision**, reviewed under the standard process with reviewers drawn from
   each affected team.
2. **Per-team decision logs** — each team records what it decides in its own
   repo or wiki; cross-team decisions appear in several logs (or none).
3. **Decision-by-meeting with minutes** — a standing architecture meeting
   whose minutes are the record.

## Decision Outcome

Chosen: **require an ADR in this corpus for cross-team architectural
decisions**, because it is the only option that gives the *affected* teams the
same standing as the *proposing* team. Per-team logs scatter the record and
guarantee divergence between copies; meeting minutes capture attendance, not
reasoning, and are unreadable as a reference six months later.

Scope rule of thumb: if implementing or reversing the decision would require
work from more than one team, it needs an ADR here. Single-team decisions may
still use ADRs (encouraged), but are not required to.

The review for a cross-team ADR extends the standard
[ADR Review Process](guides/adr-review-process) in one way: the
quorum must include **at least one reviewer from each affected team**.

### Positive Consequences

- One findable record per decision, with rationale and rejected options
- Affected teams get a formal veto point *before* the decision hardens
- New joiners and adjacent teams can self-serve the "why" behind the
  architecture
- Settled decisions stop being re-argued from scratch — reopening one
  requires engaging the written record

### Negative Consequences

- Real overhead per decision: writing and reviewing an ADR is slower than
  deciding in a meeting, and the latency is felt most on urgent calls
- A scope boundary ("architectural", "cross-team") invites edge-case
  litigation; expect some decisions to be misfiled in both directions
- If leadership doesn't model the behavior, the requirement degrades into
  retroactive paperwork — ADRs written after the decision shipped

## Implementation

1. Accept this ADR through the standard review process with reviewers from
   every team in scope (this ADR is itself cross-team).
2. Add the scope rule of thumb to the
   corpus conventions and onboarding material.
3. For one quarter, have leads nominate in-flight cross-team decisions and
   shepherd each into an ADR — the corpus seeds fastest from live decisions.
4. Review after the quarter: count cross-team decisions made vs. recorded,
   and adjust the scope rule where it chafed.
