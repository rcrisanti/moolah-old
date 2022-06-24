use crate::MoolahFrontendError;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_both() {
        let joined = join_path("localhost:8000/".to_string(), "/login".to_string());
        assert_eq!(joined, Ok("localhost:8000/login".to_string()));
    }

    #[test]
    fn test_left() {
        let joined = join_path("localhost:8000/".to_string(), "login".to_string());
        assert_eq!(joined, Ok("localhost:8000/login".to_string()));
    }

    #[test]
    fn test_right() {
        let joined = join_path("localhost:8000".to_string(), "/login".to_string());
        assert_eq!(joined, Ok("localhost:8000/login".to_string()));
    }

    #[test]
    fn test_neither() {
        let joined = join_path("localhost:8000".to_string(), "login".to_string());
        assert_eq!(joined, Ok("localhost:8000/login".to_string()));
    }

    #[test]
    fn test_fails() {
        let joined = join_path("".to_string(), "login".to_string());
        assert_eq!(joined, Err(MoolahFrontendError::JoinPathError));

        let joined = join_path("localhost:8000".to_string(), "".to_string());
        assert_eq!(joined, Err(MoolahFrontendError::JoinPathError));

        let joined = join_path("".to_string(), "".to_string());
        assert_eq!(joined, Err(MoolahFrontendError::JoinPathError));
    }
}
