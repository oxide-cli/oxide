use clap::{Subcommand, arg};

use crate::completions::CompletionShell;

#[derive(Subcommand)]
pub enum AddonCommands {
  #[command(alias = "i", about = "Install and cache an addon (oxide addon i)")]
  Install { addon_id: String },

  #[command(alias = "l", about = "List installed addons (oxide addon l)")]
  List,

  #[command(alias = "r", about = "Remove a cached addon (oxide addon r)")]
  Remove { addon_id: String },

  #[command(
    alias = "p",
    about = "Publish a GitHub repository as an Oxide addon (oxide addon p)"
  )]
  Publish {
    #[arg(help = "GitHub repository URL (e.g. https://github.com/owner/repo)")]
    addon_url: String,
  },

  #[command(
    alias = "u",
    about = "Update a GitHub repository as an Oxide addon (oxide addon u)"
  )]
  Update {
    #[arg(help = "GitHub repository URL (e.g. https://github.com/owner/repo)")]
    addon_url: String,
  },
}

#[derive(Subcommand)]
pub enum TemplateCommands {
  #[command(
    alias = "i",
    about = "Download and cache a template locally (oxide template i)"
  )]
  Install {
    #[arg(help = "Name of the template to install")]
    template_name: String,
  },

  #[command(
    alias = "l",
    about = "List all locally installed templates (oxide template l)"
  )]
  List,

  #[command(
    alias = "r",
    about = "Remove an installed template from the local cache (oxide template r)"
  )]
  Remove {
    #[arg(help = "Name of the template to remove")]
    template_name: String,
  },

  #[command(
    alias = "p",
    about = "Publish a GitHub repository as an Oxide template (oxide template p)"
  )]
  Publish {
    #[arg(help = "GitHub repository URL (e.g. https://github.com/owner/repo)")]
    template_url: String,
  },

  #[command(
    alias = "u",
    about = "Update a GitHub repository as an Oxide template (oxide template u)"
  )]
  Update {
    #[arg(help = "GitHub repository URL (e.g. https://github.com/owner/repo)")]
    template_url: String,
  },
}

#[derive(Subcommand)]
pub enum UseCommands {
  #[command(external_subcommand)]
  External(Vec<String>),
}

#[derive(Subcommand)]
pub enum Commands {
  #[command(alias = "n", about = "Create a new project from a template (oxide n)")]
  New {
    #[arg(help = "Name of the project directory to create")]
    name: String,

    #[arg(help = "Name of the template to use (e.g. react-vite-ts)")]
    template_name: String,
  },

  #[command(alias = "t", about = "Manage templates (oxide t)")]
  Template {
    #[command(subcommand)]
    command: TemplateCommands,
  },

  #[command(alias = "in", about = "Log in to your Oxide account (oxide in)")]
  Login,

  #[command(alias = "out", about = "Log out of your Oxide account (oxide out)")]
  Logout,

  #[command(about = "Show information about the currently logged-in account")]
  Account,

  #[command(alias = "a", about = "Manage addons (oxide a)")]
  Addon {
    #[command(subcommand)]
    command: AddonCommands,
  },

  #[command(
    about = "Run an installed addon command",
    override_usage = "oxide use <ADDON_ID> <COMMAND>",
    arg_required_else_help = true
  )]
  Use {
    #[command(subcommand)]
    command: UseCommands,
  },

  #[command(about = "Download and install the latest Oxide release")]
  Upgrade,

  #[command(about = "Install shell tab completion for oxide")]
  Completions {
    #[arg(value_enum, help = "Shell to install completions for")]
    shell: CompletionShell,
  },
}
