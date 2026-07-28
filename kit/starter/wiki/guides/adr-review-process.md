---
title: "ADR Review Process"
type: guide
status: active
summary: "How the team creates, reviews, and decides ADRs: the two-stage workflow, roles, quorum, and the review window."
last_updated: 2026-07-28T00:00:00Z
relates_to:
  - glossary/decision-record
  - glossary/review-quorum
  - glossary/review-window
  - decisions/0002-require-adrs-for-cross-team-architectural-decisions
---

# ADR Review Process

How the team creates, reviews, and decides ADRs: who does what, in what
order, and on what clock. The numbers here (a review quorum of 3, a
3-business-day review window) are defaults calibrated for a team of
roughly 5–10 engineers. They are configuration, not doctrine — adjust
them in an ADR of your own if your team is a different shape.

## The two-stage workflow

A decision has two distinct phases with different needs. **Drafting**
wants low friction and fast iteration; **deciding** wants visibility,
quorum, and an audit trail. The workflow keeps them separate.

### Stage 1 — Propose (low friction)

1. Create the ADR — from your harness, or directly:

   ```sh
   adroit new "Title of the decision"        # CLI / Claude Code
   # or the `new` tool over `adroit mcp --allow-write` from an MCP-only harness
   ```

   It lands in the space's `decisions/` with the next reference and
   status `proposed` in its frontmatter.
2. Fill in the body — conversationally through your harness, with
   `adroit draft` (the interview) or `adroit compose` (instruction-driven
   revision), or in your editor. Run `adroit lint <n>` to catch
   unfinished sections before a human does.
3. Commit. **Proposed ADRs need no review** — the point is that ideas
   get captured the moment they exist, visible to the whole team in the
   knowledge base.

### Stage 2 — Decide (formal)

When the proposal is ready for a decision:

1. Optionally set a deadline first so the review has a clock:
   `adroit set-review <n> <YYYY-MM-DD>`.
2. Collect the review: quorum ([[glossary/review-quorum]]) within the
   window ([[glossary/review-window]]), in whatever review artifact your
   team uses — a decision pull request where the corpus lives in a repo,
   or an explicit sign-off recorded with the decision.
3. On acceptance, run the transition — it rewrites the status in the
   page's frontmatter, in place:

   ```sh
   adroit set-status <n> accepted    # or: rejected
   ```

Rejection uses the same mechanics — only the target status differs. Land
the rejection rationale in the ADR body (a short `## Rejection Rationale`
section) *before* the decision, so the record explains itself.

## Roles

- **Proposer** — drafts the ADR, shepherds it through review, and
  incorporates feedback. The proposer owns the clock.
- **Reviewers** — any team member. Read, question, push back, approve.
- **Approver** — records the transition once quorum is met. Usually the
  proposer; a lead if the proposer lacks the authority.

## Quorum and review window

- **Quorum:** at least **3 approvals** (excluding the proposer).
- **Review window:** at least **3 business days**, so the team can
  respond asynchronously. The proposer may shorten it for genuinely
  time-sensitive decisions by saying so — visibly — where the team plans
  work.
- **Escalation:** if the window passes without quorum, raise it at the
  next team sync. If discussion cannot align, the tech lead decides and
  records the rationale with the decision.

## Reviewer expectations

- Read the ADR before the window closes; if you need more time, say so
  rather than going silent.
- Push back constructively — a challenged ADR is a stronger ADR.
- **Approve explicitly. Silence is not consent.** Quorum counts
  approvals, not absences of objection.
- **Check the vocabulary.** When a change leans on a term of art, its
  first use links the glossary entry — and if the entry doesn't exist
  yet, the same change adds it. Flag a new term that redefines or
  shadows an existing entry.

## Superseding a decision

When a new decision replaces an old one: write the new ADR with its own,
differentiated title, take it through Stage 2 as usual, then link the
pair — `adroit supersede <new> <old>` writes both sides of the
frontmatter link ([[glossary/superseded]]). Files never move; status
lives in frontmatter.

## Keeping the corpus honest

Two gates hold the record to its shape, and they are the same gates
whether a human or an AI authored the change: the space's admission
hooks (strict schema validation at commit) and `adroit check`
(supersession integrity, identity, link health). If `check` fails after
a manual edit, fix it with the tooling (`relink`, `renumber`,
`set-status`) rather than by hand — the tools rewrite only what needs to
change.
