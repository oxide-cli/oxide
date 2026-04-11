use std::path::Path;

use oxide_cli::completions::{
  powershell_profile_paths_in_for_tests, powershell_profile_snippet_for_tests,
  powershell_script_for_tests, upsert_managed_block_for_tests,
};

const POWERSHELL_PROFILE_START_MARKER: &str = "# oxide completions start";
const POWERSHELL_PROFILE_END_MARKER: &str = "# oxide completions end";

#[test]
fn powershell_profile_paths_cover_both_shells() {
  let profiles = powershell_profile_paths_in_for_tests(Path::new("/tmp/Documents"));

  assert_eq!(profiles.len(), 2);
  assert!(profiles[0].ends_with("PowerShell/Microsoft.PowerShell_profile.ps1"));
  assert!(profiles[1].ends_with("WindowsPowerShell/Microsoft.PowerShell_profile.ps1"));
}

#[test]
fn powershell_profile_snippet_embeds_script_path() {
  let snippet = powershell_profile_snippet_for_tests(Path::new(
    "C:\\Users\\maksym\\.oxide\\completions\\oxide.ps1",
  ));

  assert!(snippet.contains(POWERSHELL_PROFILE_START_MARKER));
  assert!(snippet.contains(POWERSHELL_PROFILE_END_MARKER));
  assert!(
    snippet
      .contains("$oxideCompletionScript = 'C:\\Users\\maksym\\.oxide\\completions\\oxide.ps1'")
  );
}

#[test]
fn upsert_managed_block_appends_once() {
  let block =
    format!("{POWERSHELL_PROFILE_START_MARKER}\nmanaged\n{POWERSHELL_PROFILE_END_MARKER}");

  let updated = upsert_managed_block_for_tests(
    "Set-PSReadLineOption -EditMode Windows\n",
    &block,
    POWERSHELL_PROFILE_START_MARKER,
    POWERSHELL_PROFILE_END_MARKER,
  );
  let updated_again = upsert_managed_block_for_tests(
    &updated,
    &block,
    POWERSHELL_PROFILE_START_MARKER,
    POWERSHELL_PROFILE_END_MARKER,
  );

  assert_eq!(updated, updated_again);
  assert_eq!(updated.matches(POWERSHELL_PROFILE_START_MARKER).count(), 1);
}

#[test]
fn powershell_script_mentions_native_registration() {
  let powershell_script = powershell_script_for_tests();
  assert!(powershell_script.contains("Register-ArgumentCompleter"));
  assert!(powershell_script.contains("Native"));
  assert!(powershell_script.contains("_complete"));
}
