# The Como authoring kit

Everything needed to point an AI harness at a Como KB space and author
content that lands in the right shape. The contract the kit implements is
[docs/guides/como-authoring.md](../docs/guides/como-authoring.md); the
decision behind the model is portfolio ADR-0010 (harness-first), plan of
record [portfolio#7](https://github.com/como-technologies/portfolio/issues/7).

The engine stays a dumb pipe — no LLM inside. The harness brings the
model; this kit brings the instructions and wiring.

## Layout

```
kit/
  README.md            this file
  skills/              Como skills (Claude Code format; the procedures)
    author-guide/
    author-glossary/
    research/
    lint-and-fix/
  claude-code/
    .mcp.json          MCP server registration (llm-wiki over stdio)
    CLAUDE.md          template dropped into a space for any session there
  claude-desktop/
    README.md          config snippet + current limits (MCP-only harness)
  worked-example/
    session.md         a real captured authoring session, gates and all
```

## Setup — Claude Code

1. Build or install `llm-wiki` (suite convention: build from source at
   HEAD) and have `adroit` on PATH (decisions route through it).
2. Create the space: `llm-wiki spaces create <dir> --name <name>` —
   provisioning installs the Como schema library, strict validation,
   admission hooks, and search weights; no flags needed.
3. Copy `kit/claude-code/.mcp.json` and `kit/claude-code/CLAUDE.md` into
   the space root, and `kit/skills/` to `<space>/.claude/skills/`.
4. Open Claude Code in the space and talk. The worked example shows a
   full session.

## Setup — Claude Desktop

See [claude-desktop/README.md](claude-desktop/README.md). Content classes
(`guide`, `glossary-entry`, `worked-example`) work today over MCP;
`decision` authoring needs adroit and arrives with its guarded MCP write
slice (portfolio#7, wave 2) — until then decisions are read-only there.

## Why the suite's CI gates don't use `spaces create`

Recorded here per portfolio#7 (wave 1): the sibling repos' `adr-check`
gates hand-scaffold a two-line `wiki.toml` instead of calling
`llm-wiki spaces create`, deliberately. Those gates exist to validate
each repo's committed decision corpus through adroit and must stay fast
and dependency-light — requiring an llm-wiki build in every sibling's CI
buys no additional assertion there, since the corpus check is adroit's.
Provisioning is exercised where it matters: this repo's own tests
(`tests/spaces.rs`) cover the schema library, hooks, strictness, and
weights on every run, and every kit session starts with a real
`spaces create`. If a sibling gate ever needs admission-hook coverage,
that is the moment to revisit — not before.
