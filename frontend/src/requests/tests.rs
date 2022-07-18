use super::*;

#[test]
fn test_both() {
    let joined = join_path("localhost:8000/", "/login");
    assert_eq!(joined.unwrap(), "localhost:8000/login");
}

#[test]
fn test_left() {
    let joined = join_path("localhost:8000/", "login");
    assert_eq!(joined.unwrap(), "localhost:8000/login");
}

#[test]
fn test_right() {
    let joined = join_path("localhost:8000", "/login");
    assert_eq!(joined.unwrap(), "localhost:8000/login");
}

#[test]
fn test_neither() {
    let joined = join_path("localhost:8000", "login");
    assert_eq!(joined.unwrap(), "localhost:8000/login");
}

#[test]
fn test_fails() {
    let joined = join_path("", "login");
    assert!(joined.is_err());

    let joined = join_path("localhost:8000", "");
    assert!(joined.is_err());

    let joined = join_path("", "");
    assert!(joined.is_err());
}

#[test]
fn test_replace_pattern() {
    assert_eq!(
        replace_pattern("api/account/{username}", r"\{username\}", "my_username").unwrap(),
        "api/account/my_username"
    )
}
