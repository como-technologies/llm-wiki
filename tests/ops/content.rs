use super::helpers::setup_wiki;
use llm_wiki::engine::WikiEngine;
use llm_wiki::ops;

// ── Content ───────────────────────────────────────────────────────────────────

#[test]
fn content_read_page() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    match ops::content_read(&engine, "concepts/moe", None, false, false).unwrap() {
        ops::ContentReadResult::Page(content) => {
            assert!(content.contains("Mixture of Experts"));
        }
        _ => panic!("expected Page"),
    }
}

#[test]
fn content_read_no_frontmatter() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    match ops::content_read(&engine, "concepts/moe", None, true, false).unwrap() {
        ops::ContentReadResult::Page(content) => {
            assert!(!content.contains("title:"));
            assert!(content.contains("Mixture of Experts"));
        }
        _ => panic!("expected Page"),
    }
}

#[test]
fn content_write_and_read_back() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    let body = "---\ntitle: \"New\"\ntype: page\n---\n\nHello.\n";
    let result = ops::content_write(&engine, "new-page", None, body).unwrap();
    assert_eq!(result.bytes_written, body.len());

    match ops::content_read(&engine, "new-page", None, false, false).unwrap() {
        ops::ContentReadResult::Page(content) => assert!(content.contains("Hello.")),
        _ => panic!("expected Page"),
    }
}

#[test]
fn content_new_page() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    let result = ops::content_new(
        &engine,
        "concepts/new-concept",
        None,
        false,
        false,
        None,
        None,
        None,
    )
    .unwrap();
    assert!(result.uri.starts_with("wiki://test/concepts/new-concept"));
    assert_eq!(result.slug, "concepts/new-concept");
    assert!(!result.bundle);
    assert!(result.path.exists());
    assert!(result.path.to_string_lossy().ends_with(".md"));
}

#[test]
fn content_new_section() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    let result = ops::content_new(&engine, "topics", None, true, false, None, None, None).unwrap();
    assert!(result.uri.contains("topics"));
}

#[test]
fn content_new_bundle_result_has_path_and_wiki_root() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    let result = ops::content_new(
        &engine,
        "concepts/bundled",
        None,
        false,
        true,
        None,
        None,
        None,
    )
    .unwrap();
    assert!(result.bundle);
    assert!(result.path.ends_with("index.md"));
    assert!(result.path.exists());
    assert!(result.wiki_root.is_dir());
}

// ── Stable page id resolution ─────────────────────────────────────────────────

const STABLE_ID: &str = "01ARZ3NDEKTSV4RRFFQ69G5FAV";

/// setup_wiki plus a page that declares a stable id.
fn setup_wiki_with_id(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
    let config_path = setup_wiki(dir, name);
    let wiki_root = dir.join(name).join("wiki");
    std::fs::write(
        wiki_root.join("concepts/stable.md"),
        format!(
            "---\ntitle: \"Stable\"\nid: {STABLE_ID}\ntype: concept\nstatus: active\n---\n\nStable page body.\n"
        ),
    )
    .unwrap();
    llm_wiki::git::commit(&dir.join(name), "add stable page").unwrap();
    config_path
}

#[test]
fn content_read_by_bare_id() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki_with_id(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    match ops::content_read(&engine, STABLE_ID, None, false, false).unwrap() {
        ops::ContentReadResult::Page(content) => {
            assert!(content.contains("Stable page body"));
        }
        _ => panic!("expected Page"),
    }
}

#[test]
fn content_read_by_wiki_uri_id() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki_with_id(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    let uri = format!("wiki://test/{STABLE_ID}");
    match ops::content_read(&engine, &uri, None, false, false).unwrap() {
        ops::ContentReadResult::Page(content) => {
            assert!(content.contains("Stable page body"));
        }
        _ => panic!("expected Page"),
    }
}

#[test]
fn content_read_by_id_lowercase_input() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki_with_id(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    let lower = STABLE_ID.to_lowercase();
    match ops::content_read(&engine, &lower, None, false, false).unwrap() {
        ops::ContentReadResult::Page(content) => {
            assert!(content.contains("Stable page body"));
        }
        _ => panic!("expected Page"),
    }
}

#[test]
fn id_resolution_survives_file_move() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki_with_id(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();

    // Move the page to a different directory and name, then re-index.
    let wiki_root = dir.path().join("test").join("wiki");
    std::fs::create_dir_all(wiki_root.join("guides")).unwrap();
    std::fs::rename(
        wiki_root.join("concepts/stable.md"),
        wiki_root.join("guides/stable-renamed.md"),
    )
    .unwrap();
    llm_wiki::git::commit(&dir.path().join("test"), "move stable page").unwrap();
    manager.refresh_index("test").unwrap();

    let engine = manager.state.read().unwrap();
    match ops::content_read(&engine, STABLE_ID, None, false, false).unwrap() {
        ops::ContentReadResult::Page(content) => {
            assert!(
                content.contains("Stable page body"),
                "id must resolve to the moved page"
            );
        }
        _ => panic!("expected Page"),
    }
}

#[test]
fn slug_wins_over_id_with_same_spelling() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki(dir.path(), "test");
    let wiki_root = dir.path().join("test").join("wiki");

    // A page whose slug is spelled exactly like a ULID...
    std::fs::write(
        wiki_root.join(format!("{STABLE_ID}.md")),
        "---\ntitle: \"Slug Page\"\ntype: page\nstatus: active\n---\n\nI am the slug page.\n",
    )
    .unwrap();
    // ...and a different page declaring that spelling as its id.
    std::fs::write(
        wiki_root.join("concepts/claimant.md"),
        format!(
            "---\ntitle: \"Claimant\"\nid: {STABLE_ID}\ntype: concept\nstatus: active\n---\n\nI am the id page.\n"
        ),
    )
    .unwrap();
    llm_wiki::git::commit(&dir.path().join("test"), "add pages").unwrap();

    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    match ops::content_read(&engine, STABLE_ID, None, false, false).unwrap() {
        ops::ContentReadResult::Page(content) => {
            assert!(
                content.contains("I am the slug page"),
                "an on-disk slug must shadow an id with the same spelling"
            );
        }
        _ => panic!("expected Page"),
    }
}

#[test]
fn id_pointing_at_missing_file_reports_stale_index() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki_with_id(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();

    // Delete the file without re-indexing — the index still maps the id.
    let wiki_root = dir.path().join("test").join("wiki");
    std::fs::remove_file(wiki_root.join("concepts/stable.md")).unwrap();

    let engine = manager.state.read().unwrap();
    let err = ops::content_read(&engine, STABLE_ID, None, false, false).unwrap_err();
    assert!(
        err.to_string().contains("stale"),
        "expected stale-index error, got: {err}"
    );
}

#[test]
fn unknown_id_reports_page_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki_with_id(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    let err =
        ops::content_read(&engine, "01BX5ZZKBKACTAV9WEVGEMMVRZ", None, false, false).unwrap_err();
    assert!(
        err.to_string().contains("not found"),
        "expected page-not-found error, got: {err}"
    );
}

#[test]
fn backlinks_include_pages_linking_by_id() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki_with_id(dir.path(), "test");
    let wiki_root = dir.path().join("test").join("wiki");
    std::fs::write(
        wiki_root.join("concepts/id-linker.md"),
        format!(
            "---\ntitle: \"IdLinker\"\ntype: concept\nstatus: active\n---\n\nSee [[{STABLE_ID}]].\n"
        ),
    )
    .unwrap();
    llm_wiki::git::commit(&dir.path().join("test"), "add id linker").unwrap();

    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    let refs = ops::backlinks_for(&engine, "test", "concepts/stable").unwrap();
    assert!(
        refs.iter().any(|r| r.slug == "concepts/id-linker"),
        "id-based link must appear in the target's backlinks: {refs:?}"
    );
}

#[test]
fn content_new_with_explicit_id_writes_frontmatter() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    let id = ulid::Ulid::from_string(STABLE_ID).unwrap();
    let result = ops::content_new(
        &engine,
        "concepts/with-id",
        None,
        false,
        false,
        None,
        None,
        Some(id),
    )
    .unwrap();
    assert_eq!(result.id, Some(id));

    let content = std::fs::read_to_string(&result.path).unwrap();
    let page = llm_wiki::frontmatter::parse(&content);
    assert_eq!(page.id(), Some(id));
}

#[test]
fn content_new_rejects_id_on_section() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    let id = ulid::Ulid::from_string(STABLE_ID).unwrap();
    let err =
        ops::content_new(&engine, "topics", None, true, false, None, None, Some(id)).unwrap_err();
    assert!(err.to_string().contains("section"));
}

#[test]
fn content_write_by_id_edits_declaring_page() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki_with_id(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    let body = format!(
        "---\ntitle: \"Stable\"\nid: {STABLE_ID}\ntype: concept\nstatus: active\n---\n\nRewritten body.\n"
    );
    let result = ops::content_write(&engine, STABLE_ID, None, &body).unwrap();
    assert!(result.path.ends_with("concepts/stable.md"));

    match ops::content_read(&engine, "concepts/stable", None, false, false).unwrap() {
        ops::ContentReadResult::Page(content) => assert!(content.contains("Rewritten body")),
        _ => panic!("expected Page"),
    }
}

#[test]
fn content_commit_all() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    // Write a new file so there's something to commit
    ops::content_write(
        &engine,
        "scratch",
        None,
        "---\ntitle: \"Scratch\"\ntype: page\n---\n\ntemp\n",
    )
    .unwrap();

    let hash = ops::content_commit(&engine, "test", &[], true, Some("test commit")).unwrap();
    assert!(!hash.is_empty());
}
