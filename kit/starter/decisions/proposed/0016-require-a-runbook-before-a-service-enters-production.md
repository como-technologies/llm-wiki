# ADR-0016: Require a runbook before a service enters production

> State: Proposed

## Status

Proposed
Created: 2026-06-12

## Stakeholders

Service owners (write and maintain the runbooks), on-call engineers (the
runbooks' primary readers), tech lead (owns the readiness bar).

## Context and Problem Statement

When a production service misbehaves at 03:00, the responder is rarely the
person who built it. What they need in that moment is not the architecture
diagram but the operational facts: how to tell the service is actually
unhealthy, how to restart it safely, where its logs and dashboards live,
what its known failure modes look like, and who to wake if the page is
beyond them. Today that knowledge lives in heads and chat history. Every
service that ships without writing it down converts a future incident from
a procedure into an investigation.

The cheapest moment to capture operational knowledge is before launch,
while the builders still remember everything. We need to decide whether a
runbook is a courtesy or a launch requirement.

## Decision Drivers

- Incident response time should not depend on whether the original author
  is awake
- The requirement must be cheap enough that it never becomes the reason a
  launch slips by weeks
- Runbooks rot: whatever we require must have a realistic maintenance
  story, not just a creation story
- Bring-your-own-stack: the rule must work whether "production" is a
  container platform, a fleet of VMs, or a single server

## Considered Options

1. **Runbook as a launch gate, from a minimal template** — a service may
   not take production traffic until a runbook exists covering: health
   checks (how to tell it's up), restart/rollback procedure, logs and
   dashboards locations, known failure modes, dependencies, escalation
   path. The template is one page; "minimal but present" is the bar, and
   the runbook is reviewed like code.
2. **Runbooks recommended, not required** — encourage them in onboarding
   docs and reviews. This is the status quo in most teams, and it produces
   runbooks only for the services that least need them (the ones their
   authors love).
3. **Centralized ops documentation maintained by a platform/ops function**
   — a dedicated owner keeps quality high, but it reintroduces a handoff
   (builders throw services over a wall), and small teams don't have the
   headcount.

## Decision Outcome

Chosen: **runbook as a launch gate, from a minimal template**, because it
is the only option that makes operational knowledge exist *for every
service* at the moment it is cheapest to write. The minimal template keeps
the gate from becoming a documentation project: one honest page beats a
beautiful wiki that doesn't exist.

The maintenance story is incident-driven: every incident review checks
"did the runbook help?" and files the gap as a runbook fix — the runbook
converges on reality precisely as fast as reality bites.

### Positive Consequences

- Any on-call engineer can execute first-line response for any service
  without waking its author
- Launch conversations gain a concrete, checkable readiness artifact
- Incident reviews get a standing, low-friction improvement target —
  runbooks get better exactly where they proved weak

### Negative Consequences

- A real gate means a launch can genuinely be blocked on documentation;
  the team must hold that line under delivery pressure or the rule is
  theater
- Runbooks for rarely-touched services will still rot between incidents;
  the gate guarantees existence, not freshness
- One more template to maintain, and pressure to bloat it must be resisted
  — every section added to the template taxes every future launch

## Rollout

1. Write the one-page runbook template and store it with the knowledge base's
   guides.
2. Backfill runbooks for the highest-traffic existing services first (the
   on-call rotation's pain ranking is the priority order).
3. Add the runbook check to the launch/readiness checklist.
4. Add "did the runbook help?" to the incident review questions; route
   gaps back as runbook fixes.
