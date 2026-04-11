use std::path::Path;

use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

use crate::{
  AppContext,
  auth::token::get_auth_user,
  cache::{CachedTemplate, get_cached_template, update_templates_cache},
  utils::archive::download_and_extract,
};

#[derive(Deserialize)]
struct TemplateInfoRes {
  archive_url: String,
  commit_sha: String,
  subdir: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallResult {
  Installed,
  Updated { version: String },
  UpToDate,
}

impl InstallResult {
  pub fn message(&self, template_name: &str) -> Option<String> {
    match self {
      Self::Installed => Some(format!(
        "Template '{template_name}' downloaded successfully"
      )),
      Self::Updated { version } => {
        Some(format!("Template '{template_name}' updated to v{version}"))
      }
      Self::UpToDate => None,
    }
  }

  pub fn up_to_date_message(template_name: &str) -> String {
    format!("Template '{template_name}' is already up to date")
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InstallState {
  Install,
  Update,
  UpToDate,
}

fn classify_install_state(
  cached_template: Option<&CachedTemplate>,
  template_dir_exists: bool,
  latest_commit_sha: &str,
) -> InstallState {
  let Some(cached_template) = cached_template else {
    return InstallState::Install;
  };

  if !template_dir_exists {
    return InstallState::Install;
  }

  if cached_template.commit_sha == latest_commit_sha {
    InstallState::UpToDate
  } else {
    InstallState::Update
  }
}

#[doc(hidden)]
pub fn classify_install_state_for_tests(
  cached_template: Option<&CachedTemplate>,
  template_dir_exists: bool,
  latest_commit_sha: &str,
) -> &'static str {
  match classify_install_state(cached_template, template_dir_exists, latest_commit_sha) {
    InstallState::Install => "install",
    InstallState::Update => "update",
    InstallState::UpToDate => "up_to_date",
  }
}

async fn get_template_info(
  template_name: &str,
  client: &Client,
  auth_path: &Path,
  backend_url: &str,
) -> Result<TemplateInfoRes> {
  let user = get_auth_user(auth_path)?;

  let res: TemplateInfoRes = client
    .get(format!("{backend_url}/template/{template_name}/url"))
    .bearer_auth(user.token)
    .header("Content-Type", "application/json")
    .send()
    .await?
    .error_for_status()?
    .json()
    .await?;

  Ok(res)
}

pub async fn install_template(ctx: &AppContext, template_name: &str) -> Result<InstallResult> {
  let info = get_template_info(
    template_name,
    &ctx.client,
    &ctx.paths.auth,
    &ctx.backend_url,
  )
  .await?;

  let dest = ctx.paths.templates.join(template_name);
  let cached_template = get_cached_template(ctx, template_name)?;
  let install_state =
    classify_install_state(cached_template.as_ref(), dest.exists(), &info.commit_sha);

  if install_state == InstallState::UpToDate {
    return Ok(InstallResult::UpToDate);
  }

  {
    let mut guard = ctx.cleanup_state.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(dest.clone());
  }

  let download_result = download_and_extract(
    &ctx.client,
    &info.archive_url,
    &dest,
    info.subdir.as_deref(),
  )
  .await;

  {
    let mut guard = ctx.cleanup_state.lock().unwrap_or_else(|e| e.into_inner());
    *guard = None;
  }

  download_result?;

  let cached_template = update_templates_cache(
    &ctx.paths.templates,
    Path::new(template_name),
    &info.commit_sha,
  )?;

  Ok(match install_state {
    InstallState::Install => InstallResult::Installed,
    InstallState::Update => InstallResult::Updated {
      version: cached_template.version,
    },
    InstallState::UpToDate => unreachable!("up-to-date templates should return early"),
  })
}
