use anyhow::anyhow;
use oxide_cli::{
  addons::runner::{rerun_prompt_message_for_tests, should_fallback_to_cached_manifest_for_tests},
  utils::errors::OxideError,
};

#[test]
fn fallback_to_cached_manifest_when_user_is_not_logged_in() {
  let error = anyhow::Error::from(OxideError::NotLoggedIn);
  assert!(should_fallback_to_cached_manifest_for_tests(&error));
}

#[test]
fn do_not_fallback_to_cached_manifest_for_unrelated_errors() {
  let error = anyhow!("oxide.addon.json is malformed");
  assert!(!should_fallback_to_cached_manifest_for_tests(&error));
}

#[test]
fn rerun_prompt_message_is_none_when_versions_match() {
  let prompt = rerun_prompt_message_for_tests("install", Some("1.0.0"), "1.0.0");
  assert!(prompt.is_none());
}

#[test]
fn rerun_prompt_message_mentions_both_versions_when_version_changed() {
  let prompt = rerun_prompt_message_for_tests("install", Some("1.0.0"), "1.1.0");
  assert_eq!(
    prompt.as_deref(),
    Some(
      "Command 'install' was last run with v1.0.0 of this add-on. A new version (v1.1.0) is available. Re-run it now?"
    )
  );
}
