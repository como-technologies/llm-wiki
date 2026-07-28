# Claude Desktop setup (MCP-only harness)

Claude Desktop reaches the space through MCP alone — no shell. Content
classes work through llm-wiki's server; decisions work through adroit's
guarded MCP write slice (its ADR-0021), started with `--allow-write`.

## Config

Add both servers to `claude_desktop_config.json` (Settings → Developer →
Edit Config), using absolute binary paths:

```json
{
  "mcpServers": {
    "llm-wiki": {
      "command": "/absolute/path/to/llm-wiki",
      "args": ["serve"]
    },
    "adroit": {
      "command": "/absolute/path/to/adroit",
      "args": ["mcp", "--allow-write"],
      "env": { "ADROIT_DIR": "/absolute/path/to/your-space" }
    }
  }
}
```

`llm-wiki serve` speaks MCP over stdio and exposes all 23 tools plus
`wiki://` resources; scope its registry with
`"env": {"LLM_WIKI_CONFIG": "/path/to/config.toml"}` when the machine
hosts spaces that shouldn't be visible to this client. `adroit mcp
--allow-write` projects the read verbs plus the guarded write slice —
`new`, `compose`, `set-status`, and `plan --save` — each announced
`destructiveHint: true`, so Desktop asks you before any of them runs.
Omit `--allow-write` for a read-only adroit server.

## What works today, plainly

| Activity | Status |
|---|---|
| Research, search, graph, read (all classes) | works |
| Author `guide` / `glossary-entry` / `worked-example` | works — same contract as everywhere: `generated` + low confidence, linked at birth, ingest + lint |
| Author and transition `decision` pages | works via adroit's write slice — you approve each destructive tool call, which is exactly where acceptance lives. `compose` (AI body revision) additionally needs adroit's own provider configured server-side (`ai.enabled`; local ollama works). `draft`'s interactive interview and the forge integrations stay CLI-only by design |

Paste the relevant sections of the Como authoring contract
(`docs/guides/como-authoring.md`) into your project instructions — a
Desktop project has no CLAUDE.md discovery, so the contract must arrive
via the project's custom instructions.
