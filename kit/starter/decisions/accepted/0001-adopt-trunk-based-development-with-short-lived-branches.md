# ADR-0001: Adopt trunk-based development with short-lived branches

> State: Accepted

## Status

Accepted

## Stakeholders

| Role | Name |
|------|------|
| Tech lead (owner) | _your tech lead_ |
| Engineers (affected) | whole team |
| Release manager (informed) | _if applicable_ |

## Context and Problem Statement

The team's branching model determines how quickly work integrates and how
painful merges are. Long-lived feature branches accumulate divergence: merges
become risky events, code review arrives weeks after the design choices it
should have influenced, and "integration" becomes a phase instead of a habit.
CI signal on a stale branch tells you little about what the combined system
will do.

We need one explicit branching discipline so that tooling (CI triggers, branch
protection, release automation) and habits (review size, merge cadence) can be
built around it instead of around each engineer's personal workflow.

## Decision Drivers

- Integration risk should grow with *change size*, not with *elapsed time*
- Review feedback must arrive while the work is still cheap to change
- CI must validate the combination of everyone's work, continuously
- The model must work without specialized tooling — any forge's branch
  protection is enough
- Release cuts should be possible from a known-good, always-integrated line

## Considered Options

1. **Trunk-based development with short-lived branches** — all work branches
   off the main branch and merges back within a few days; the main branch is
   always releasable; anything bigger hides behind a feature flag.
2. **GitFlow** — long-lived `develop` + release branches, features merged into
   `develop`, periodic stabilization before release.
3. **Per-engineer fork-and-PR with no branch lifetime rule** — maximum
   autonomy; integration cadence left to each contributor.

## Decision Outcome

Chosen: **trunk-based development with short-lived branches**, because it is
the only option where the cost of integration stays proportional to the size
of the change. GitFlow institutionalizes divergence (its stabilization phase
exists *because* integration was deferred), and the no-rule option makes
integration cadence a personality trait rather than a team property.

Concretely:

- Branches live **at most a few working days** before merging to the main
  branch; if the work is bigger, slice it and ship the slices behind a flag.
- The main branch is protected: merges require a green CI run and review per
  the [ADR Review Process](guides/adr-review-process) quorum rules
  for decision PRs, or one approval for ordinary code.
- Incomplete features ship dark behind feature flags rather than waiting on a
  branch.

### Positive Consequences

- Merge conflicts shrink from events to noise; integration is continuous
- Reviews are small enough to be read properly, so review quality rises
- CI always reflects the true combined state of the system
- Release is a tag on a healthy main branch, not a stabilization project

### Negative Consequences

- Requires feature-flag discipline, and flags are debt: each needs an owner
  and a removal date, or the codebase fills with dead toggles
- Half-done work lands on the main branch (dark); the team must be comfortable
  with "merged" no longer meaning "released"
- Engineers used to long private branches lose a working style they may
  prefer; the first weeks need active coaching, not just a rule

## Rollout

1. Protect the main branch (require CI green + review) in the forge settings.
2. Agree the branch-lifetime norm (target: ≤3 working days) and the
   feature-flag convention, including flag ownership and removal dates.
3. Socialize slicing strategies for large work (branch by abstraction,
   keystone-last) in a short team session.
4. Revisit after one month: measure median branch lifetime and merge-conflict
   frequency; record findings in a follow-up to this ADR.

## Implementation

<!-- adroit:plan -->

**Implementation Plan: Adopt Trunk-Based Development with Short-Lived Branches**
====================================================================

### Checklist

#### Step 1: Protect the Main Branch (CI Green + Review) (~1 day)

*   Update forge settings to require CI green and review for main branch merges
*   Ensure reviewers understand the updated process and quorum rules

#### Step 2: Define Branch-Lifetime Norm and Feature-Flag Convention (~1 day)

*   Schedule a team session to agree on the target branch-lifetime norm (≤3 working days)
*   Document the feature-flag convention, including flag ownership and removal dates
*   Ensure all stakeholders understand the new process

#### Step 3: Implement Slicing Strategies for Large Work (~2 days)

*   Develop documentation on slicing strategies for large work (branch by abstraction, keystone-last)
*   Provide training to engineers on the new approach and best practices

#### Step 4: Revisit and Refine (~1 month)

*   Measure median branch lifetime and merge-conflict frequency
*   Record findings in a follow-up ADR
*   Refine process as needed based on data and feedback

### Components/Files Likely Touched

*   Forge settings (main branch protection, CI triggers)
*   Team documentation (branch-lifetime norm, feature-flag convention)
*   Engineer training materials (slicing strategies, quorum rules)

### Testing

*   Ensure CI green and review for main branch merges
*   Validate feature-flag discipline
*   Monitor merge-conflict frequency and adjust process as needed

### Rollout/Migration

*   Protect the main branch in the forge settings
*   Implement slicing strategies for large work
*   Revisit and refine the process after one month

### Risks to Watch

*   Feature-flag debt: ensure flag ownership and removal dates are managed effectively
*   Engineer resistance: provide coaching and training to support engineers' transition to short-lived branches
*   Merge-conflict frequency: monitor data and adjust the process as needed

<!-- /adroit:plan -->
