"""Stable page identity — id declaration, resolution, and move survival.

Pages are created at runtime (not in the shared fixtures) so the
zero-change guarantee for id-free wikis stays covered by the existing
suites.
"""

import json
import subprocess

ULID_B = "01ARZ3NDEKTSV4RRFFQ69G5FAV"
ULID_C = "01BX5ZZKBKACTAV9WEVGEMMVRZ"
ULID_MISSING = "01CFGH0000000000000000ZZZZ"


def _git(env, *args):
    subprocess.run(
        [
            "git", "-C", str(env.research),
            "-c", "user.name=test", "-c", "user.email=test@test.com",
            "-c", "commit.gpgsign=false",
            *args,
        ],
        check=True,
    )


def _write_page(env, rel_path, title, page_id=None, body="Body.\n"):
    path = env.research_wiki / rel_path
    path.parent.mkdir(parents=True, exist_ok=True)
    id_line = f"id: {page_id}\n" if page_id else ""
    path.write_text(
        f"---\ntitle: \"{title}\"\n{id_line}type: concept\nstatus: active\n---\n\n{body}"
    )


def _commit_and_rebuild(env, message):
    _git(env, "add", ".")
    _git(env, "commit", "-qm", message)
    env.run("index", "rebuild", "--wiki", "research")


def _broken_links(env):
    """broken-link findings for pages this suite creates (the shared
    fixture wiki intentionally contains unrelated broken links)."""
    result = env.run(
        "lint", "--wiki", "research", "--rules", "broken-link",
        "--format", "json", check=False,
    )
    data = json.loads(result.stdout)
    return [
        f
        for f in data["findings"]
        if f["rule"] == "broken-link"
        and f["slug"].startswith(("decisions/", "guides/"))
    ]


def test_id_link_survives_move(wiki_env):
    """The acceptance test: move a page, its id links keep resolving."""
    _write_page(wiki_env, "decisions/target.md", "Target", page_id=ULID_B)
    _write_page(
        wiki_env, "decisions/linker.md", "Linker",
        body=f"See [[{ULID_B}]].\n",
    )
    _commit_and_rebuild(wiki_env, "add id-linked pages")
    assert _broken_links(wiki_env) == []

    # Move the target to a new directory and name
    (wiki_env.research_wiki / "guides").mkdir(exist_ok=True)
    _git(wiki_env, "mv", "wiki/decisions/target.md", "wiki/guides/target-moved.md")
    _commit_and_rebuild(wiki_env, "move target")

    assert _broken_links(wiki_env) == [], "id link must survive the move"

    # And the id still reads the moved page
    result = wiki_env.run("content", "read", ULID_B, "--wiki", "research")
    assert "Target" in result.stdout


def test_mixed_slug_and_id_links_after_move(wiki_env):
    """Slug link to a moved page dangles; id link to a moved page does not."""
    _write_page(wiki_env, "decisions/by-slug.md", "BySlug")
    _write_page(wiki_env, "decisions/by-id.md", "ById", page_id=ULID_C)
    _write_page(
        wiki_env, "decisions/linker.md", "Linker",
        body=f"See [[decisions/by-slug]] and [[{ULID_C}]].\n",
    )
    _commit_and_rebuild(wiki_env, "add mixed-link pages")
    assert _broken_links(wiki_env) == []

    _git(wiki_env, "mv", "wiki/decisions/by-slug.md", "wiki/decisions/by-slug-moved.md")
    _git(wiki_env, "mv", "wiki/decisions/by-id.md", "wiki/decisions/by-id-moved.md")
    _commit_and_rebuild(wiki_env, "move both")

    broken = _broken_links(wiki_env)
    assert len(broken) == 1, f"exactly the slug link must dangle: {broken}"
    assert "decisions/by-slug" in broken[0]["message"]


def test_duplicate_id_is_error(wiki_env):
    _write_page(wiki_env, "decisions/x.md", "X", page_id=ULID_B)
    _write_page(wiki_env, "decisions/y.md", "Y", page_id=ULID_B)
    _commit_and_rebuild(wiki_env, "add duplicate ids")

    result = wiki_env.run(
        "lint", "--wiki", "research", "--rules", "duplicate-id",
        "--format", "json", check=False,
    )
    data = json.loads(result.stdout)
    dups = [f for f in data["findings"] if f["rule"] == "duplicate-id"]
    assert len(dups) == 2
    assert all(f["severity"] == "error" for f in dups)


def test_id_format_warning(wiki_env):
    _write_page(wiki_env, "decisions/bad.md", "Bad", page_id="not-a-ulid")
    _commit_and_rebuild(wiki_env, "add malformed id")

    result = wiki_env.run(
        "lint", "--wiki", "research", "--rules", "id-format",
        "--format", "json", check=False,
    )
    data = json.loads(result.stdout)
    findings = [f for f in data["findings"] if f["rule"] == "id-format"]
    assert len(findings) == 1
    assert findings[0]["severity"] == "warning"


def test_unknown_id_is_broken_link(wiki_env):
    _write_page(
        wiki_env, "decisions/dangling.md", "Dangling",
        body=f"See [[{ULID_MISSING}]].\n",
    )
    _commit_and_rebuild(wiki_env, "add dangling id link")

    broken = _broken_links(wiki_env)
    assert any(ULID_MISSING in f["message"] for f in broken)


def test_content_new_with_id_flag(wiki_env):
    result = wiki_env.run(
        "content", "new", "decisions/fresh", "--id", "--wiki", "research"
    )
    assert "(id: " in result.stdout
    ulid = result.stdout.split("(id: ")[1].strip().rstrip(")")
    assert len(ulid) == 26

    content = (wiki_env.research_wiki / "decisions/fresh.md").read_text()
    assert f"id: {ulid}" in content


def test_content_new_rejects_invalid_id(wiki_env):
    result = wiki_env.run(
        "content", "new", "decisions/nope", "--id", "not-a-ulid",
        "--wiki", "research", check=False,
    )
    assert result.returncode != 0
    assert "ULID" in result.stdout + result.stderr


def test_list_surfaces_id_only_when_declared(wiki_env):
    _write_page(wiki_env, "decisions/tagged.md", "Tagged", page_id=ULID_B)
    _commit_and_rebuild(wiki_env, "add tagged page")

    pages = wiki_env.json("list", "--wiki", "research")["pages"]
    tagged = next(p for p in pages if p["slug"] == "decisions/tagged")
    assert tagged["id"] == ULID_B
    for p in pages:
        if p["slug"] != "decisions/tagged":
            assert "id" not in p, f"id must be omitted when absent: {p['slug']}"
