"""Stable page identity over MCP — resolve/read by id, auto_id on create."""

from conftest import SPACE_NAME

ULID = "01ARZ3NDEKTSV4RRFFQ69G5FAV"


async def test_resolve_and_read_by_id(wiki_env, mutable_mcp_env):
    page = wiki_env.research_wiki / "concepts" / "id-page.md"
    page.write_text(
        f'---\ntitle: "Id Page"\nid: {ULID}\ntype: concept\nstatus: active\n---\n\nId page body.\n'
    )
    await mutable_mcp_env.rebuild()

    resolved = await mutable_mcp_env.json(
        "wiki_resolve", {"uri": ULID, "wiki": SPACE_NAME}
    )
    assert resolved["slug"] == "concepts/id-page"
    assert resolved["exists"] is True
    assert resolved["id"] == ULID

    content = await mutable_mcp_env.call(
        "wiki_content_read", {"uri": ULID, "wiki": SPACE_NAME}
    )
    assert "Id page body" in content


async def test_resolve_without_id_omits_field(mcp_env):
    resolved = await mcp_env.json(
        "wiki_resolve", {"uri": "concepts/mixture-of-experts", "wiki": SPACE_NAME}
    )
    assert resolved["exists"] is True
    assert "id" not in resolved


async def test_content_new_auto_id(wiki_env, mutable_mcp_env):
    result = await mutable_mcp_env.json(
        "wiki_content_new",
        {"uri": "concepts/auto-id-page", "wiki": SPACE_NAME, "auto_id": True},
    )
    assert len(result["id"]) == 26

    content = (wiki_env.research_wiki / "concepts" / "auto-id-page.md").read_text()
    assert f"id: {result['id']}" in content


async def test_content_new_rejects_invalid_id(mutable_mcp_env):
    is_error, text = await mutable_mcp_env.call_raw(
        "wiki_content_new",
        {"uri": "concepts/bad-id-page", "wiki": SPACE_NAME, "id": "not-a-ulid"},
    )
    assert is_error
    assert "ULID" in text
