use std::path::Path;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use toml;

use crate::config::{GlobalConfig, WikiEntry, load_global, save_global};
use crate::default_schemas::default_schemas;
use crate::git;

// ── CreateReport ──────────────────────────────────────────────────────────────

/// Outcome of a wiki space creation.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReport {
    /// Absolute path of the wiki directory.
    pub path: String,
    /// Registered name of the wiki.
    pub name: String,
    /// True if the directory was newly created.
    pub created: bool,
    /// True if the space was added to the global config.
    pub registered: bool,
    /// True if an initial git commit was made.
    pub committed: bool,
}

/// Outcome of registering an existing wiki space.
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterReport {
    /// Absolute path of the wiki directory.
    pub path: String,
    /// Registered name of the wiki.
    pub name: String,
    /// True if the space was added to the global config (false if already registered).
    pub registered: bool,
    /// Always false — register does not create directories.
    pub created: bool,
    /// Always false — register does not create git commits.
    pub committed: bool,
}

// ── create ────────────────────────────────────────────────────────────────────

/// Create a new wiki repository at `path`, register it, and optionally commit.
pub fn create(
    path: &Path,
    name: &str,
    description: Option<&str>,
    force: bool,
    set_default: bool,
    config_path: &Path,
    wiki_root: Option<&str>,
) -> Result<CreateReport> {
    let mut created = false;
    if !path.exists() {
        std::fs::create_dir_all(path)?;
        created = true;
    }
    let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let wiki_root = wiki_root.unwrap_or("wiki");
    let mut committed = false;

    // Check re-run conditions
    let global = load_global(config_path)?;
    if let Some(existing) = global
        .wikis
        .iter()
        .find(|w| w.path == path.to_string_lossy())
    {
        if existing.name == name {
            ensure_structure(&path, name, description, wiki_root)?;
            provision_admission(&path, name, config_path)?;
            return Ok(CreateReport {
                path: path.to_string_lossy().into(),
                name: name.into(),
                created: false,
                registered: false,
                committed: false,
            });
        } else if !force {
            bail!(
                "wiki already registered as \"{}\". Use --force to rename.",
                existing.name
            );
        }
    }

    ensure_structure(&path, name, description, wiki_root)?;

    // Git init if not already a repo
    if !path.join(".git").exists() {
        git::init_repo(&path)?;
    }

    // Initial commit
    let commit_result = git::commit(&path, &format!("create: {name}"));
    if let Ok(ref hash) = commit_result
        && !hash.is_empty()
    {
        committed = true;
    }

    // Register
    let entry = WikiEntry {
        name: name.into(),
        path: path.to_string_lossy().into(),
        description: description.map(|s| s.into()),
        remote: None,
    };
    register(entry, force, config_path)?;

    // Ensure global engine directories exist
    if let Some(wiki_dir) = config_path.parent() {
        let logs_dir = wiki_dir.join("logs");
        if !logs_dir.exists() {
            std::fs::create_dir_all(&logs_dir)?;
        }
    }

    provision_admission(&path, name, config_path)?;

    if set_default {
        set_default_wiki(name, config_path)?;
    }

    Ok(CreateReport {
        path: path.to_string_lossy().into(),
        name: name.into(),
        created,
        registered: true,
        committed,
    })
}

/// Register an existing wiki repository without creating files.
///
/// Reads `wiki.toml` from `path` (if it exists) to determine `wiki_root`.
/// If `wiki_root_override` is given and `wiki.toml` already declares a
/// different `wiki_root`, returns an error.
pub fn register_existing(
    path: &Path,
    name: &str,
    description: Option<&str>,
    wiki_root_override: Option<&str>,
    config_path: &Path,
) -> Result<RegisterReport> {
    if !path.exists() {
        bail!("path \"{}\" does not exist", path.display());
    }
    let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

    // Read existing wiki.toml wiki_root if present
    let existing_toml_root: Option<String> = {
        let toml_path = path.join("wiki.toml");
        if toml_path.exists() {
            let raw = std::fs::read_to_string(&toml_path)?;
            if raw.contains("wiki_root") {
                let cfg: crate::config::WikiConfig = toml::from_str(&raw).unwrap_or_default();
                Some(cfg.wiki_root)
            } else {
                None
            }
        } else {
            None
        }
    };

    // Conflict check
    let effective_root: String = match (&wiki_root_override, &existing_toml_root) {
        (Some(flag), Some(toml)) if *flag != toml => {
            bail!(
                "wiki.toml already declares wiki_root = \"{toml}\". \
                 Remove it manually before registering with a different value."
            );
        }
        (Some(flag), _) => flag.to_string(),
        (None, Some(toml)) => toml.clone(),
        (None, None) => "wiki".to_string(),
    };

    validate_wiki_root(&path, &effective_root)?;

    ensure_structure(&path, name, description, &effective_root)?;

    let entry = WikiEntry {
        name: name.into(),
        path: path.to_string_lossy().into(),
        description: description.map(|s| s.into()),
        remote: None,
    };

    let global = crate::config::load_global(config_path)?;
    let already_registered = global.wikis.iter().any(|w| w.name == name);
    if !already_registered {
        register(entry, false, config_path)?;
    }

    Ok(RegisterReport {
        path: path.to_string_lossy().into(),
        name: name.into(),
        registered: !already_registered,
        created: false,
        committed: false,
    })
}

fn ensure_structure(
    path: &Path,
    name: &str,
    description: Option<&str>,
    wiki_root: &str,
) -> Result<()> {
    for dir in &["inbox", "evidence", "schemas"] {
        let d = path.join(dir);
        if !d.exists() {
            std::fs::create_dir_all(&d)?;
        }
        let gitkeep = d.join(".gitkeep");
        if !gitkeep.exists() {
            std::fs::write(&gitkeep, "")?;
        }
    }

    // Create the (possibly custom) wiki content directory
    let wiki_dir = path.join(wiki_root);
    if !wiki_dir.exists() {
        std::fs::create_dir_all(&wiki_dir)?;
    }
    let gitkeep = wiki_dir.join(".gitkeep");
    if !gitkeep.exists() {
        std::fs::write(&gitkeep, "")?;
    }

    // Write embedded default schemas
    let schemas_dir = path.join("schemas");
    for (filename, content) in default_schemas() {
        let dest = schemas_dir.join(filename);
        if !dest.exists() {
            std::fs::write(&dest, content)?;
        }
    }

    // Write embedded default body templates
    for (filename, content) in crate::default_schemas::default_templates() {
        let dest = schemas_dir.join(filename);
        if !dest.exists() {
            std::fs::write(&dest, content)?;
        }
    }

    let readme = path.join("README.md");
    if !readme.exists() {
        let desc_line = description.map(|d| format!("\n{d}\n")).unwrap_or_default();
        let content = format!(
            "# {name}\n{desc_line}\nManaged by [llm-wiki](https://github.com/geronimo-iia/llm-wiki). Run `llm-wiki serve` to start the MCP server.\n"
        );
        std::fs::write(&readme, content)?;
    }

    let wiki_toml = path.join("wiki.toml");
    if !wiki_toml.exists() {
        std::fs::write(&wiki_toml, generate_wiki_toml(name, description, wiki_root))?;
    }

    Ok(())
}

fn generate_wiki_toml(name: &str, description: Option<&str>, wiki_root: &str) -> String {
    let mut s = format!("name = \"{name}\"\n");
    if let Some(desc) = description {
        s.push_str(&format!("description = \"{desc}\"\n"));
    }
    if wiki_root != "wiki" {
        s.push_str(&format!("wiki_root = \"{wiki_root}\"\n"));
    }

    // Como provisioning defaults (llm-wiki#14) — written per-space so the
    // admission contract travels with the data, not the machine.
    s.push_str(
        "\n\
         # Strict admission (kb-spec §3): every frontmatter violation fails ingest.\n\
         [validation]\n\
         type_strictness = \"strict\"\n\
         \n\
         # Search score multipliers for both status vocabularies (kb-spec §4):\n\
         # the decision lifecycle and the engine content vocabulary.\n\
         [search.status]\n\
         proposed = 0.7\n\
         accepted = 1.0\n\
         rejected = 0.9\n\
         deprecated = 0.3\n\
         superseded = 0.3\n\
         active = 1.0\n\
         draft = 0.8\n\
         stub = 0.5\n\
         generated = 0.8\n\
         archived = 0.3\n\
         unknown = 0.9\n",
    );
    s
}

// ── Como admission provisioning (llm-wiki#14) ─────────────────────────────────

/// Marker line identifying hooks this engine manages; a hook file without it
/// is user-owned and never overwritten.
const HOOK_MARKER: &str = "managed by `llm-wiki spaces create`";

/// Provision the admission model for a space: the two git hooks that make a
/// commit the unit of admission (kb-spec §7), plus catch-up-on-read
/// (`index.auto_rebuild`) in the global config. Idempotent on every path.
fn provision_admission(path: &Path, name: &str, config_path: &Path) -> Result<()> {
    install_admission_hooks(path, name, config_path)?;

    // Catch-up-on-read is the read-side half of the post-commit consumer
    // model (kb-spike findings/issue-09). Global-only key.
    let mut global = load_global(config_path)?;
    if !global.index.auto_rebuild {
        global.index.auto_rebuild = true;
        save_global(&global, config_path)?;
    }
    Ok(())
}

/// Write `pre-commit` (validate-only gate) and `post-commit` (index consumer)
/// into `.git/hooks`. The hooks embed this binary's path and the registry
/// config so a bare `git commit` in the space works from any shell. Hooks run
/// only for real `git` invocations — libgit2 commits (the engine's own
/// `ingest`/`content commit`) never execute them, so no recursion is possible.
fn install_admission_hooks(path: &Path, name: &str, config_path: &Path) -> Result<()> {
    if !path.join(".git").exists() {
        return Ok(());
    }
    let hooks_dir = path.join(".git").join("hooks");
    std::fs::create_dir_all(&hooks_dir)?;

    let exe = std::env::current_exe()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "llm-wiki".into());
    let cfg = std::path::absolute(config_path)
        .unwrap_or_else(|_| config_path.to_path_buf())
        .to_string_lossy()
        .into_owned();

    for (hook, dry_run) in [("pre-commit", " --dry-run"), ("post-commit", "")] {
        let dest = hooks_dir.join(hook);
        if dest.exists() {
            let existing = std::fs::read_to_string(&dest).unwrap_or_default();
            if !existing.contains(HOOK_MARKER) {
                continue; // user-owned hook — leave it alone
            }
        }
        let script = format!(
            "#!/bin/sh\n\
             # llm-wiki admission gate — {HOOK_MARKER}; delete to opt out.\n\
             exec \"{exe}\" --config \"{cfg}\" --wiki \"{name}\" ingest .{dry_run}\n"
        );
        std::fs::write(&dest, script)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755))?;
        }
    }
    Ok(())
}

/// Validate `wiki_root` before using it at registration time.
///
/// Checks: non-empty, relative, no `..`, not a reserved dir, directory exists,
/// and resolves to a path strictly inside `repo_path`.
pub fn validate_wiki_root(repo_path: &Path, wiki_root: &str) -> Result<()> {
    if wiki_root.is_empty() || wiki_root == "." {
        bail!("wiki_root must not be empty or \".\"");
    }
    if std::path::Path::new(wiki_root).is_absolute() {
        bail!("wiki_root must be a relative path (no leading \"/\")");
    }
    use std::path::Component;
    for component in std::path::Path::new(wiki_root).components() {
        if matches!(component, Component::ParentDir) {
            bail!("wiki_root must not contain \"..\" components");
        }
    }
    let top = std::path::Path::new(wiki_root)
        .components()
        .next()
        .and_then(|c| {
            if let Component::Normal(s) = c {
                Some(s.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .unwrap_or_default();
    // `raw` stays reserved for backward compatibility: existing spaces created
    // before the capture layer was renamed to `evidence/` still contain a
    // `raw/` directory.
    for reserved in &["inbox", "evidence", "raw", "schemas"] {
        if top == *reserved {
            bail!("wiki_root \"{wiki_root}\" uses reserved directory \"{reserved}\"");
        }
    }
    let candidate = repo_path.join(wiki_root);
    if !candidate.exists() {
        bail!(
            "wiki_root directory \"{}\" does not exist",
            candidate.display()
        );
    }
    let repo_abs = std::fs::canonicalize(repo_path)
        .with_context(|| format!("cannot canonicalize repo path {}", repo_path.display()))?;
    let root_abs = std::fs::canonicalize(&candidate)
        .with_context(|| format!("cannot canonicalize wiki_root {}", candidate.display()))?;
    if !root_abs.starts_with(&repo_abs) {
        bail!(
            "wiki_root must be inside the repository (resolved to {}, repo is {})",
            root_abs.display(),
            repo_abs.display()
        );
    }
    Ok(())
}

// ── Space management ──────────────────────────────────────────────────────────

/// Look up a registered wiki by name; error if not found.
pub fn resolve_name(name: &str, global: &GlobalConfig) -> Result<WikiEntry> {
    global
        .wikis
        .iter()
        .find(|w| w.name == name)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("wiki \"{name}\" is not registered"))
}

/// Add or update a wiki entry in the global config; errors if already registered and `force` is false.
pub fn register(entry: WikiEntry, force: bool, config_path: &Path) -> Result<()> {
    let mut config = load_global(config_path)?;

    if let Some(existing) = config.wikis.iter_mut().find(|w| w.name == entry.name) {
        if !force {
            bail!(
                "wiki already registered as \"{}\". Use --force to update.",
                entry.name
            );
        }
        *existing = entry;
    } else {
        config.wikis.push(entry);
    }

    save_global(&config, config_path)
}

/// Unregister a wiki from the global config; optionally delete its directory.
pub fn remove(name: &str, delete: bool, config_path: &Path) -> Result<()> {
    let mut config = load_global(config_path)?;

    if config.global.default_wiki == name {
        bail!("\"{name}\" is the default wiki \u{2014} set a new default first");
    }

    let idx = config
        .wikis
        .iter()
        .position(|w| w.name == name)
        .ok_or_else(|| anyhow::anyhow!("wiki \"{name}\" is not registered"))?;

    let entry = config.wikis.remove(idx);

    if delete {
        let path = Path::new(&entry.path);
        if path.exists() {
            std::fs::remove_dir_all(path)?;
        }
    }

    save_global(&config, config_path)
}

/// Return all registered wiki entries from the global config.
pub fn load_all(global: &GlobalConfig) -> Vec<WikiEntry> {
    global.wikis.clone()
}

/// Set the default wiki in the global config.
pub fn set_default_wiki(name: &str, config_path: &Path) -> Result<()> {
    let mut config = load_global(config_path)?;

    if !config.wikis.iter().any(|w| w.name == name) {
        bail!("wiki \"{name}\" is not registered");
    }

    config.global.default_wiki = name.to_string();
    save_global(&config, config_path)
}
