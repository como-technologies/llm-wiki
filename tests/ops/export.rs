use super::helpers::setup_wiki;
use llm_wiki::engine::WikiEngine;
use llm_wiki::ops;
use llm_wiki::ops::{ExportFormat, ExportOptions};

// ── llms-txt ──────────────────────────────────────────────────────────────────

#[test]
fn export_llms_txt_writes_file_and_reports() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    let report = ops::export(
        &engine,
        &ExportOptions {
            wiki: "test".into(),
            path: Some("llms.txt".into()),
            format: ExportFormat::LlmsTxt,
            include_archived: false,
        },
    )
    .unwrap();

    assert_eq!(report.format, "llms-txt");
    assert!(report.pages_written >= 2);
    assert!(report.bytes > 0);

    let content = std::fs::read_to_string(&report.path).unwrap();
    assert!(content.contains("# test"));
    assert!(content.contains("## concept"));
    assert!(content.contains("wiki://test/concepts/moe"));
}

// ── llms-full ─────────────────────────────────────────────────────────────────

#[test]
fn export_llms_full_includes_page_bodies() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    let report = ops::export(
        &engine,
        &ExportOptions {
            wiki: "test".into(),
            path: Some("llms-full.txt".into()),
            format: ExportFormat::LlmsFull,
            include_archived: false,
        },
    )
    .unwrap();

    let content = std::fs::read_to_string(&report.path).unwrap();
    // Each page entry is preceded by ---
    assert!(content.contains("---\n\n# ["));
    // Body text from helpers::setup_wiki
    assert!(content.contains("Mixture of Experts"));
}

// ── json ──────────────────────────────────────────────────────────────────────

#[test]
fn export_json_produces_valid_json_array() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    let report = ops::export(
        &engine,
        &ExportOptions {
            wiki: "test".into(),
            path: Some("wiki.json".into()),
            format: ExportFormat::Json,
            include_archived: false,
        },
    )
    .unwrap();

    let content = std::fs::read_to_string(&report.path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(parsed.is_array());
    let arr = parsed.as_array().unwrap();
    assert!(arr.len() >= 2);
    let first = &arr[0];
    assert!(first.get("slug").is_some());
    assert!(first.get("title").is_some());
    assert!(first.get("type").is_some());
    assert!(first.get("body").is_some());
}

// ── path resolution ───────────────────────────────────────────────────────────

#[test]
fn export_default_path_resolves_to_wiki_root() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki(dir.path(), "test");
    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    let report = ops::export(
        &engine,
        &ExportOptions {
            wiki: "test".into(),
            path: None,
            format: ExportFormat::LlmsTxt,
            include_archived: false,
        },
    )
    .unwrap();

    // Default path is llms.txt relative to wiki root
    assert!(report.path.ends_with("llms.txt"));
    assert!(std::path::PathBuf::from(&report.path).exists());
}

// ── status filter ─────────────────────────────────────────────────────────────

#[test]
fn export_excludes_archived_by_default() {
    use llm_wiki::git;
    use std::fs;

    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki(dir.path(), "test");
    let wiki_path = dir.path().join("test");
    let wiki_root = wiki_path.join("wiki");

    // Add an archived page
    fs::write(
        wiki_root.join("concepts/archived-page.md"),
        "---\ntitle: \"Archived\"\ntype: concept\nstatus: archived\n---\n\nOld content.\n",
    )
    .unwrap();
    git::commit(&wiki_path, "add archived").unwrap();

    // Rebuild the index with the new page
    let manager = WikiEngine::build(&config_path).unwrap();
    {
        let wiki_name = "test".to_string();
        ops::index_rebuild(&manager, &wiki_name).unwrap();
    }
    let engine = manager.state.read().unwrap();

    let report = ops::export(
        &engine,
        &ExportOptions {
            wiki: "test".into(),
            path: Some("out.txt".into()),
            format: ExportFormat::LlmsTxt,
            include_archived: false,
        },
    )
    .unwrap();
    let content = std::fs::read_to_string(&report.path).unwrap();
    assert!(!content.contains("archived-page"));

    let report_all = ops::export(
        &engine,
        &ExportOptions {
            wiki: "test".into(),
            path: Some("out-all.txt".into()),
            format: ExportFormat::LlmsTxt,
            include_archived: true,
        },
    )
    .unwrap();
    let content_all = std::fs::read_to_string(&report_all.path).unwrap();
    assert!(content_all.contains("archived-page"));
}

// ── stable page id ────────────────────────────────────────────────────────────

#[test]
fn export_json_includes_id_only_when_declared() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = setup_wiki(dir.path(), "test");
    let wiki_root = dir.path().join("test").join("wiki");
    std::fs::write(
        wiki_root.join("concepts/identified.md"),
        "---\ntitle: \"Identified\"\nid: 01ARZ3NDEKTSV4RRFFQ69G5FAV\ntype: concept\nstatus: active\n---\n\nBody.\n",
    )
    .unwrap();
    llm_wiki::git::commit(&dir.path().join("test"), "add identified").unwrap();

    let manager = WikiEngine::build(&config_path).unwrap();
    let engine = manager.state.read().unwrap();

    let report = ops::export(
        &engine,
        &ExportOptions {
            wiki: "test".into(),
            path: Some("llms.json".into()),
            format: ExportFormat::Json,
            include_archived: false,
        },
    )
    .unwrap();

    let json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&report.path).unwrap()).unwrap();
    let pages = json.as_array().unwrap();
    let identified = pages
        .iter()
        .find(|p| p["slug"] == "concepts/identified")
        .unwrap();
    assert_eq!(identified["id"], "01ARZ3NDEKTSV4RRFFQ69G5FAV");
    let moe = pages.iter().find(|p| p["slug"] == "concepts/moe").unwrap();
    assert!(
        moe.get("id").is_none(),
        "id must be omitted when not declared"
    );
}
