# ADR-0004: Maintain a glossary of shared engineering terms in the knowledge base

> State: Accepted

## Status

Accepted

## Stakeholders

| Role | Name |
|------|------|
| knowledge base editor (owner) | _whoever curates this book_ |
| Engineering leads (consulted) | leads of each team in scope |
| Engineers (affected) | everyone who reads or writes ADRs and guides |

## Context and Problem Statement

The knowledge base's decisions and guides lean on terms that sound universal but
are not: "accepted", "service", "environment", "release", "incident",
"breaking change". Each team arrives with its own definitions, and the
differences surface at the worst moments — during a review where two people
argue past each other, or during an incident where "rollback" means three
different procedures to three different responders.

Today the definitions live nowhere. Some are implied by individual ADRs, some
by guides, most by oral tradition. A reader cannot tell whether a term in an
ADR is being used loosely or as a term of art, and authors have no anchor to
link to when precision matters.

## Decision Drivers

- One canonical definition per term, with a stable link target authors can
  reference from ADRs and guides
- Discoverability: a reader inside any chapter must be able to reach the
  definition in one click
- Low friction to extend — adding a term should be a small, reviewable diff,
  not a process
- The vocabulary must evolve with the corpus: terms enter when a decision or
  guide starts depending on them
- Zero new tooling; the knowledge base is a content product and stays one

## Considered Options

1. **A glossary page in this book**, one alphabetized list with anchor links,
   extended via ordinary reviewed PRs whenever a decision or guide introduces
   a term of art.
2. **Per-document definitions** — each ADR or guide defines the terms it
   uses, inline, where they first appear.
3. **An external wiki page** — keep the vocabulary in the team's general
   wiki, outside the book.

## Decision Outcome

Chosen: **a glossary page in this book**, because the vocabulary is part of
the knowledge base's contract with its readers and must version, review, and ship
with it. Per-document definitions guarantee drift — five documents define
"release" five ways and no diff ever shows the conflict. An external wiki
splits the product in two: the book would depend on an unreviewed,
separately-permissioned source that the book's CI can neither lint nor link
reliably.

Concretely:

- The book gains a top-level `Glossary` page (`src/glossary.md`), listed in
  the navigation after Guides.
- Each entry is a heading (so it has a stable anchor) followed by a
  definition of at most a short paragraph; entries stay alphabetized.
- ADRs and guides link to the glossary entry the first time they use a term
  of art, instead of redefining it.
- A term earns an entry when a decision or guide depends on its precise
  meaning — the glossary follows the corpus, it is not filled speculatively.

### Positive Consequences

- Reviews argue about the decision, not about what the words mean
- Authors get link targets, so ADRs become shorter and more precise
- Disagreements about a definition become visible, reviewable diffs against
  one canonical text
- New joiners get the local meaning of overloaded industry terms in one place

### Negative Consequences

- The page is curation debt: without an owner it ages into a list of stale
  definitions that misleads with authority — worse than no glossary
- Boundary disputes are now explicit; expect occasional slow reviews over a
  single sentence of definition
- Existing documents do not link to the glossary retroactively; the
  convention only pays off as documents are touched and updated

## Rollout

1. Add `src/glossary.md` with the page conventions above and the first
   entries: the terms this corpus already relies on (ADR statuses, "decision
   record", "guide", "review quorum").
2. Add the page to `src/SUMMARY.md` after the Guides section and rebuild the
   book.
3. Update the [ADR Review Process](guides/adr-review-process) guide:
   reviewers check that new terms of art either link to an existing entry or
   add one in the same change.
4. Name the glossary owner (the knowledge base editor by default) and fold a
   staleness pass into the corpus's regular review cadence.

## Implementation

<!-- adroit:plan -->

**Implementation Plan: Maintain a Glossary of Shared Engineering Terms in the knowledge base**

### Step 1: Create the Glossary Page (`src/glossary.md`)

* **Component:** `src/glossary.md`
* **Testing:** Ensure that the page is generated correctly during the build process and that it displays alphabetically.
* **Rollout:** Review the first set of entries manually to ensure accuracy.

### Step 2: Update Navigation and Build Process

* **Component:** `src/SUMMARY.md` and `build.gradle`
* **Testing:** Verify that the new page is included in the navigation bar after Guides.
* **Rollout:** Rebuild the book with the updated navigation.

### Step 3: Implement Linking to Glossary Entries

* **Component:** All ADRs and guides
* **Testing:** Ensure that links to glossary entries are correct and functional.
* **Rollout:** Review a sample set of documents to ensure that linking is working correctly.

### Step 4: Introduce Staleness Pass for Glossary Maintenance

* **Component:** `src/glossary.md` and `build.gradle`
* **Testing:** Verify that the staleness pass updates the page regularly.
* **Rollout:** Schedule regular reviews (e.g., quarterly) to update the glossary entries.

### Step 5: Update ADR Review Process

* **Component:** `../../guides/adr-review-process.md`
* **Testing:** Review the updated guide to ensure that it accurately reflects the new requirements.
* **Rollout:** Roll out the updated ADR review process.

### Risks and Mitigations:

* **Risk 1: Curation Debt**: Regularly review and update glossary entries to prevent staleness.
	+ Mitigation: Schedule regular reviews (e.g., quarterly) to maintain the accuracy of glossary entries.
* **Risk 2: Boundary Disputes**: Establish clear guidelines for reviewing and resolving disputes.
	+ Mitigation: Develop a process for addressing boundary disputes, such as requesting clarification from the glossary owner or other relevant stakeholders.
* **Risk 3: Incorrect Links**: Thoroughly test linking to ensure accuracy.
	+ Mitigation: Regularly review links to glossary entries to prevent errors.

<!-- /adroit:plan -->
