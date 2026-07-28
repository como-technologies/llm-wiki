---
name: research
description: Answer a question from the Como KB with citations - search, read, graph-walk, and answer only from what the space actually says. Use when the user asks what the KB knows, what was decided, or why.
---

# Research the knowledge base

Answer from the space, not from memory. The KB is the record; your
training data is not.

1. **Search wide**: `wiki_search` the question's terms (try the
   `format: "llms"` output for compact results); check both statuses —
   an accepted decision outranks a superseded one, but the superseded
   trail often answers "why".
2. **Read what you cite**: `wiki_content_read` every page you will rely
   on. Follow `relates_to` edges and backlinks (`wiki_graph` when the
   neighborhood matters).
3. **Answer with the record's voice**: state what the pages say, cite
   each claim with its page slug (and pinned `citations:` refs when the
   page itself derives from evidence). Distinguish plainly between what
   the KB states and what you are inferring across pages.
4. **Say what's missing.** If the KB doesn't answer the question, say so
   — and offer to capture the gap (a `stub` page, or a drafted guide via
   the author-guide skill) rather than filling it from your own priors.
5. For decision content, prefer adroit's read surface when available
   (`adroit show/list/ask` on the CLI) — it is the decision head and
   carries the corpus semantics.

Never present an ungrounded answer as the KB's position. "The space
doesn't record this" is a correct, useful answer.
