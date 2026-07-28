---
name: author-glossary
description: Author glossary-entry pages in a Como KB space - one term per page, defined once, linked from birth. Use when a conversation surfaces a shared term worth pinning down, or the user asks to define terms.
---

# Author a glossary entry

One term, one page: the title is the term, the body is the definition.
Follow the Como authoring contract (`docs/guides/como-authoring.md`).

1. **Check it isn't defined**: `wiki_search` the term and its synonyms.
   If an entry exists, update or alias it — never author a second
   definition of the same concept.
2. **Scaffold**: `wiki_content_new` under `glossary/<term-slug>`, write
   the body to the returned path:

   ```yaml
   ---
   title: "<The Term>"
   type: glossary-entry
   status: generated
   confidence: 0.3
   summary: "<the one-line definition>"
   aliases: [<synonyms and alternate spellings, if any>]
   relates_to: [<the decision or guide that gives the term its meaning here>]
   ---
   ```

3. **Body**: 2–5 sentences. What it is, what it is not (when confusable
   with a neighbor), and one usage example. Wikilink other defined terms.
4. **Link at birth**: `wiki_suggest`; every entry should relate to at
   least the page that made the term matter. No orphans.
5. **Gate**: `wiki_ingest`, then `wiki_lint`; clear introduced errors.
6. Report the URI(s) and note the entries await human promotion.

Batch-friendly: when defining several terms from one conversation,
scaffold and write them all, then ingest together — one commit, one
review unit.
