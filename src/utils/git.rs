use std::{
  fs,
  path::{Path, PathBuf},
};

use anyhow::Result;
use reqwest::{Client, header::USER_AGENT};
use serde::Deserialize;

use crate::cache::is_template_installed;

#[derive(Deserialize)]
struct GithubEntry {
  name: String,

  #[serde(rename = "type")]
  entry_type: String,

  download_url: Option<String>,
  url: String,
}

#[derive(Deserialize)]
struct OxideTemplateConfig {
  name: String,
}

pub async fn download_dir(
  client: &Client,
  api_url: &str,
  path: &Path,
  is_root: bool,
  tempate_path: &Path,
) -> Result<()> {
  fs::create_dir_all(path)?;

  let entries: Vec<GithubEntry> = client
    .get(api_url)
    .header(USER_AGENT, "oxide")
    .send()
    .await?
    .error_for_status()?
    .json()
    .await?;

  let is_indstalled = if is_root {
    let template_name = fetch_template_name(client, &entries).await?;
    is_template_installed(&template_name, tempate_path)?
  } else {
    false
  };

  if !is_indstalled {
    for entry in entries {
      let local_path: PathBuf = path.join(&entry.name);

      match entry.entry_type.as_str() {
        "file" => {
          if let Some(download_url) = entry.download_url {
            let bytes = client
              .get(download_url)
              .header(USER_AGENT, "oxide")
              .send()
              .await?
              .bytes()
              .await?;

            fs::write(&local_path, bytes)?;
            println!("✓ {}", local_path.display());
          }
        }
        "dir" => {
          Box::pin(download_dir(
            client,
            &entry.url,
            &local_path,
            false,
            tempate_path,
          ))
          .await?;
        }
        _ => {}
      }
    }

    Ok(())
  } else {
    println!("This template is installed in");
    std::process::exit(0)
  }
}

async fn fetch_template_name(client: &Client, entries: &[GithubEntry]) -> Result<String> {
  let entry = entries
    .iter()
    .find(|e| e.name == "oxide.template.json")
    .ok_or_else(|| anyhow::anyhow!("oxide.template.json not found"))?;

  let download_url = entry
    .download_url
    .as_ref()
    .ok_or_else(|| anyhow::anyhow!("No download URL for oxide.template.json"))?;

  let text = client
    .get(download_url)
    .header(USER_AGENT, "oxide")
    .send()
    .await?
    .text()
    .await?;

  let config: OxideTemplateConfig = serde_json::from_str(&text)?;
  Ok(config.name)
}
