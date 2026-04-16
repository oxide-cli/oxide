use std::{fs, path::Path, path::PathBuf};

use oxide_cli::completions::{
  addon_candidates, command_for_paths, powershell_profile_paths_in, template_candidates,
  upsert_managed_block, upsert_zsh_config, zsh_fpath_snippet,
};

// ── upsert_managed_block ─────────────────────────────────────────────────────

#[test]
fn upsert_managed_block_inserts_into_empty() {
  let result = upsert_managed_block("", "# start\ncode\n# end", "# start", "# end");
  assert_eq!(result, "# start\ncode\n# end\n");
}

#[test]
fn upsert_managed_block_replaces_existing() {
  let original = "before\n\n# start\nold code\n# end\nafter\n";
  let result = upsert_managed_block(original, "# start\nnew code\n# end", "# start", "# end");
  assert_eq!(result, "before\n\n# start\nnew code\n# end\nafter\n");
}

#[test]
fn upsert_managed_block_appends_when_absent() {
  let original = "existing content\n";
  let result = upsert_managed_block(original, "# start\ncode\n# end", "# start", "# end");
  assert_eq!(result, "existing content\n\n# start\ncode\n# end\n");
}

// ── powershell_profile_paths_in ───────────────────────────────────────────────

#[test]
fn powershell_profile_paths_in_returns_both_profiles() {
  let docs = PathBuf::from("/home/user/Documents");
  let paths = powershell_profile_paths_in(&docs);
  assert_eq!(paths.len(), 2);
  assert!(paths[0].ends_with("PowerShell/Microsoft.PowerShell_profile.ps1"));
  assert!(paths[1].ends_with("WindowsPowerShell/Microsoft.PowerShell_profile.ps1"));
}

// ── template_candidates ───────────────────────────────────────────────────────

#[test]
fn template_candidates_empty_when_no_cache() {
  let candidates = template_candidates(None);
  assert!(candidates.is_empty());
}

// ── addon_candidates ──────────────────────────────────────────────────────────

#[test]
fn addon_candidates_empty_when_no_dir() {
  let candidates = addon_candidates(None);
  assert!(candidates.is_empty());
}

// ── command_for_paths ─────────────────────────────────────────────────────────

#[test]
fn command_for_paths_builds_without_panic() {
  let _ = command_for_paths(None, None);
}

// ── zsh_fpath_snippet ─────────────────────────────────────────────────────────

#[test]
fn zsh_fpath_snippet_contains_dir_and_compinit() {
  let snippet = zsh_fpath_snippet(Path::new("/home/user/.zfunc"));
  assert!(snippet.contains("fpath=(/home/user/.zfunc $fpath)"));
  assert!(snippet.contains("autoload -Uz compinit && compinit"));
  assert!(snippet.starts_with("# oxide completions start"));
  assert!(snippet.ends_with("# oxide completions end"));
}

// ── upsert_zsh_config ─────────────────────────────────────────────────────────

#[test]
fn upsert_zsh_config_writes_new_file() {
  let dir = assert_fs::TempDir::new().unwrap();
  let config = dir.path().join(".zshrc");
  upsert_zsh_config(&config, Path::new("/home/user/.zfunc")).unwrap();
  let content = fs::read_to_string(&config).unwrap();
  assert!(content.contains("fpath=(/home/user/.zfunc $fpath)"));
}

#[test]
fn upsert_zsh_config_is_idempotent() {
  let dir = assert_fs::TempDir::new().unwrap();
  let config = dir.path().join(".zshrc");
  upsert_zsh_config(&config, Path::new("/home/user/.zfunc")).unwrap();
  upsert_zsh_config(&config, Path::new("/home/user/.zfunc")).unwrap();
  let content = fs::read_to_string(&config).unwrap();
  assert_eq!(
    content.matches("# oxide completions start").count(),
    1,
    "managed block should appear exactly once"
  );
}

#[test]
fn upsert_zsh_config_replaces_old_block() {
  let dir = assert_fs::TempDir::new().unwrap();
  let config = dir.path().join(".zshrc");
  upsert_zsh_config(&config, Path::new("/old/.zfunc")).unwrap();
  upsert_zsh_config(&config, Path::new("/new/.zfunc")).unwrap();
  let content = fs::read_to_string(&config).unwrap();
  assert!(content.contains("/new/.zfunc"), "should contain new path");
  assert!(!content.contains("/old/.zfunc"), "should not contain old path");
}

#[test]
fn upsert_zsh_config_preserves_existing_content() {
  let dir = assert_fs::TempDir::new().unwrap();
  let config = dir.path().join(".zshrc");
  fs::write(&config, "export PATH=$PATH:/usr/local/bin\n").unwrap();
  upsert_zsh_config(&config, Path::new("/home/user/.zfunc")).unwrap();
  let content = fs::read_to_string(&config).unwrap();
  assert!(content.contains("export PATH=$PATH:/usr/local/bin"));
  assert!(content.contains("fpath=(/home/user/.zfunc $fpath)"));
}
