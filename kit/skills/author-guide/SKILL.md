---
name: author-guide
description: Author a guide page in a Como KB space from the current conversation - research first, typed frontmatter, linked at birth, gate-clean. Use when the user asks to write up a how-to, runbook, or practice guide.
---

# Author a guide

Follow the Como authoring contract (`docs/guides/como-authoring.md` in
the llm-wiki repo). A guide operationalizes decisions — it says *how* to
do what was decided.

1. **Research before writing.** `wiki_search` the topic; `wiki_content_read`
   anything adjacent. If an existing guide covers it, propose updating
   that page instead of authoring a near-duplicate.
2. **Scaffold**: `wiki_content_new` under `guides/<topic-slug>`, then
   write the body to the returned path with frontmatter:

   ```yaml
   ---
   title: "<imperative title>"
   type: guide
   status: generated
   confidence: 0.3
   summary: "<one line: who does what, when>"
   relates_to: [<decision slugs/ids this operationalizes, related guides>]
   citations: [<path@commit refs for any evidence-derived claim>]
   ---
   ```

3. **Body shape**: why-in-one-paragraph, then numbered steps with real
   commands, then verification ("you know it worked when…"). Wikilink
   terms to their glossary entries and decisions by slug — plain
   `[[slug]]` only; `[[slug|alias]]` is not engine syntax and lints as
   a broken link.
4. **Link at birth**: run `wiki_suggest` on the new page; add what
   belongs to `relates_to` or body wikilinks. No orphans.
5. **Gate**: `wiki_ingest` the page (strict — a named-rule failure means
   fix and re-ingest, never work around); then `wiki_lint` and clear
   every error you introduced.
6. Report to the user: the page URI, what you linked it to, and that it
   awaits human promotion from `generated`.

Never set `status: active` and never raise `confidence` yourself —
promotion is the human reviewer's act.
