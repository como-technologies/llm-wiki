# Claude Desktop setup (MCP-only harness)

Claude Desktop reaches the space through MCP alone — no shell. Content
classes work fully today; decision authoring waits on adroit's guarded
MCP write slice (portfolio#7, wave 2).

## Config

Add to `claude_desktop_config.json` (Settings → Developer → Edit Config),
using an absolute path to the `llm-wiki` binary:

```json
{
  "mcpServers": {
    "llm-wiki": {
      "command": "/absolute/path/to/llm-wiki",
      "args": ["serve"]
    }
  }
}
```

`llm-wiki serve` speaks MCP over stdio and exposes all 23 tools plus
`wiki://` resources. Scope the registry with
`"env": {"LLM_WIKI_CONFIG": "/path/to/config.toml"}` when the machine
hosts spaces that shouldn't be visible to this client.

## What works today, plainly

| Activity | Status |
|---|---|
| Research, search, graph, read (all classes) | works |
| Author `guide` / `glossary-entry` / `worked-example` | works — same contract as everywhere: `generated` + low confidence, linked at birth, ingest + lint |
| Author or transition `decision` pages | **not yet** — read-only until the adroit MCP write slice lands (wave 2); draft the text in conversation and hand it to a shell-capable session or a human |

Paste the relevant sections of the Como authoring contract
(`docs/guides/como-authoring.md`) into your project instructions — a
Desktop project has no CLAUDE.md discovery, so the contract must arrive
via the project's custom instructions.
