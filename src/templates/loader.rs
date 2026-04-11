use std::path::Path;

use anyhow::Result;

use crate::{
  AppContext,
  templates::{TemplateFile, install::install_template},
  utils::fs::read_dir_to_files,
};

pub async fn get_files(ctx: &AppContext, template_name: &str) -> Result<Vec<TemplateFile>> {
  let path = Path::new(template_name);
  let install_result = install_template(ctx, template_name).await?;
  if let Some(message) = install_result.message(template_name) {
    println!("{message}");
  }

  let files = read_dir_to_files(&ctx.paths.templates.join(path))?;

  Ok(files)
}
