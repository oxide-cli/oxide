use chrono::{TimeZone, Utc};
use oxide_cli::upgrade::{
  asset_filename_for_tests, is_cache_fresh_for_tests, is_newer_version_for_tests,
  normalize_version_tag_for_tests, parse_version_for_tests, release_asset_url_for_tests,
  render_upgrade_notice,
};

#[test]
fn normalize_version_tag_strips_leading_v() {
  let version = normalize_version_tag_for_tests("v1.2.3").unwrap();
  assert_eq!(version, "1.2.3");
}

#[test]
fn parse_version_rejects_invalid_formats() {
  let error = parse_version_for_tests("1.2").unwrap_err();
  assert!(error.to_string().contains("patch"));
}

#[test]
fn is_newer_version_compares_numeric_components() {
  assert!(is_newer_version_for_tests("0.7.4", "0.8.0").unwrap());
  assert!(!is_newer_version_for_tests("0.8.0", "0.7.4").unwrap());
}

#[test]
fn asset_filename_appends_exe_for_windows_builds() {
  assert_eq!(
    asset_filename_for_tests("windows-x86_64"),
    "oxide-windows-x86_64.exe"
  );
  assert_eq!(
    asset_filename_for_tests("linux-x86_64"),
    "oxide-linux-x86_64"
  );
}

#[test]
fn release_asset_url_uses_expected_github_pattern() {
  let asset_url = release_asset_url_for_tests("1.2.3", "linux-x86_64");
  assert_eq!(
    asset_url,
    "https://github.com/oxide-cli/oxide/releases/download/v1.2.3/oxide-linux-x86_64"
  );
}

#[test]
fn cache_is_fresh_for_recent_version_checks() {
  let now = Utc.with_ymd_and_hms(2026, 4, 11, 10, 30, 0).unwrap();
  assert!(is_cache_fresh_for_tests(
    "2026-04-11T10:00:00Z",
    "0.8.0",
    now
  ));
}

#[test]
fn cache_is_stale_after_one_hour() {
  let now = Utc.with_ymd_and_hms(2026, 4, 11, 11, 0, 1).unwrap();
  assert!(!is_cache_fresh_for_tests(
    "2026-04-11T10:00:00Z",
    "0.8.0",
    now
  ));
}

#[test]
fn render_upgrade_notice_mentions_upgrade_command() {
  let mut parts = env!("CARGO_PKG_VERSION").split('.');
  let major = parts.next().unwrap();
  let minor = parts.next().unwrap();
  let patch: u64 = parts.next().unwrap().parse().unwrap();
  let latest = format!("{major}.{minor}.{}", patch + 1);
  let notice = render_upgrade_notice(&latest);
  assert!(notice.contains("Run `oxide upgrade` to update."));
  assert!(notice.contains(&format!("v{} → v{}", env!("CARGO_PKG_VERSION"), latest)));
}
