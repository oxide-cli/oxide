use colored::Colorize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OxideError {
  #[error("You are not logged in.")]
  NotLoggedIn,
  #[error("Authentication failed. Your session may have expired.")]
  HttpUnauthorized,
  #[error("{0} was not found.")]
  HttpNotFound(String),
  #[error("The server returned an error while fetching {0}.")]
  HttpServerError(String),
  #[error("Could not connect to the server.")]
  NetworkConnect,
  #[error("The request timed out.")]
  NetworkTimeout,
}

/// Converts a `reqwest::Error` into an `anyhow::Error`, mapping well-known
/// error kinds to typed `OxideError` variants so `print_error` can add hints.
pub fn classify_reqwest_error(err: reqwest::Error, resource: &str) -> anyhow::Error {
  if err.is_connect() {
    return OxideError::NetworkConnect.into();
  }
  if err.is_timeout() {
    return OxideError::NetworkTimeout.into();
  }
  if let Some(status) = err.status() {
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
      return OxideError::HttpUnauthorized.into();
    }
    if status == reqwest::StatusCode::NOT_FOUND {
      return OxideError::HttpNotFound(resource.to_string()).into();
    }
    if status.is_server_error() {
      return OxideError::HttpServerError(resource.to_string()).into();
    }
  }
  anyhow::anyhow!("Network error while fetching {resource}")
}

/// Prints an error to stderr in a user-friendly format.
///
/// Set `OXIDE_DEBUG=1` to see the full error chain for debugging.
pub fn print_error(err: &anyhow::Error) {
  if std::env::var("OXIDE_DEBUG").is_ok() {
    eprintln!("{} {:?}", "error:".red().bold(), err);
    return;
  }

  // Check for OxideError anywhere in the chain — use outermost message + hint.
  for cause in err.chain() {
    if let Some(oxide_err) = cause.downcast_ref::<OxideError>() {
      eprintln!("{} {}", "error:".red().bold(), err);
      if let Some(hint) = hint_for_oxide_error(oxide_err) {
        eprintln!("  {} {}", "hint:".cyan().bold(), hint);
      }
      return;
    }
  }

  // Check for a raw reqwest error in the chain.
  for cause in err.chain() {
    if let Some(reqwest_err) = cause.downcast_ref::<reqwest::Error>() {
      // If reqwest is the outermost error, replace with a friendly message.
      // Otherwise the outermost with_context message is already human-readable.
      if err.downcast_ref::<reqwest::Error>().is_some() {
        eprintln!(
          "{} {}",
          "error:".red().bold(),
          friendly_reqwest_message(reqwest_err)
        );
      } else {
        eprintln!("{} {}", "error:".red().bold(), err);
      }
      if let Some(hint) = hint_for_reqwest_error(reqwest_err) {
        eprintln!("  {} {}", "hint:".cyan().bold(), hint);
      }
      return;
    }
  }

  // Default: print the outermost anyhow message (already readable via with_context).
  eprintln!("{} {}", "error:".red().bold(), err);
}

fn hint_for_oxide_error(err: &OxideError) -> Option<&'static str> {
  match err {
    OxideError::NotLoggedIn => Some("Run `oxide login` to authenticate."),
    OxideError::HttpUnauthorized => Some("Run `oxide login` to re-authenticate."),
    OxideError::HttpNotFound(_) => {
      Some("Check the name is correct and that you have access.")
    }
    OxideError::HttpServerError(_) => {
      Some("This is likely a temporary issue. Try again in a moment.")
    }
    OxideError::NetworkConnect | OxideError::NetworkTimeout => {
      Some("Check your internet connection and try again.")
    }
  }
}

fn friendly_reqwest_message(err: &reqwest::Error) -> String {
  if err.is_connect() {
    return "Could not connect to the server.".to_string();
  }
  if err.is_timeout() {
    return "The request timed out.".to_string();
  }
  if let Some(status) = err.status() {
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
      return "Authentication failed. Your session may have expired.".to_string();
    }
    if status == reqwest::StatusCode::NOT_FOUND {
      return "The requested resource was not found.".to_string();
    }
    if status.is_server_error() {
      return "The server returned an error. This is likely a temporary issue.".to_string();
    }
  }
  "An unexpected network error occurred.".to_string()
}

fn hint_for_reqwest_error(err: &reqwest::Error) -> Option<&'static str> {
  if err.is_connect() || err.is_timeout() {
    return Some("Check your internet connection and try again.");
  }
  if err.status().is_some_and(|s| {
    s == reqwest::StatusCode::UNAUTHORIZED || s == reqwest::StatusCode::FORBIDDEN
  }) {
    return Some("Run `oxide login` to re-authenticate.");
  }
  None
}
