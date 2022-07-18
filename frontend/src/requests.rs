pub mod requester;
#[cfg(test)]
mod tests;

pub use requester::{Requester, ResponseAction};

use regex::Regex;

use super::MoolahFrontendError;

fn join_path(base_url: String, relative_url: String) -> Result<String, MoolahFrontendError> {
    match (base_url.chars().last(), relative_url.chars().next()) {
        (Some('/'), Some('/')) => {
            let mut rel_chars = relative_url.chars();
            rel_chars.next();
            Ok(format!("{}{}", base_url, rel_chars.collect::<String>()))
        }
        (Some('/'), Some(_)) => Ok(format!("{}{}", base_url, relative_url)),
        (Some(_), Some('/')) => Ok(format!("{}{}", base_url, relative_url)),
        (Some(_), Some(_)) => Ok(format!("{}/{}", base_url, relative_url)),
        _ => Err(MoolahFrontendError::JoinPathError),
    }
}

pub fn fully_qualified_path(relative_url: String) -> Result<String, MoolahFrontendError> {
    match web_sys::window() {
        Some(window) => join_path(window.origin(), relative_url),
        None => Err(MoolahFrontendError::WebSysError),
    }
}

pub fn replace_pattern(
    base: &str,
    re_pattern: &str,
    replace: &str,
) -> Result<String, MoolahFrontendError> {
    let re = Regex::new(re_pattern)?;

    Ok(re.replace(base, replace).to_string())
}
