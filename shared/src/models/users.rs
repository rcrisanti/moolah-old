use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::schema::users;
use crate::MoolahSharedError;

lazy_static! {
    static ref PASSWORD_REGEX: Regex = Regex::new(r"[a-zA-Z\d@#$%^&-+=()!? ]{8,24}$").unwrap();
    static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9]{4,18}$").unwrap();
}

#[derive(Queryable, Debug, Identifiable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    password: String,
    password_salt: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    password: String,
    password_salt: String,
}

impl TryFrom<UserForm> for NewUser {
    type Error = MoolahSharedError;

    fn try_from(form: UserForm) -> Result<Self, Self::Error> {
        form.validate()?;

        let salt = SaltString::generate(&mut OsRng);
        let pass_hash = Argon2::default()
            .hash_password(form.password.as_bytes(), &salt)?
            .to_string();

        Ok(NewUser {
            username: form.username.to_lowercase(),
            email: form.email.to_lowercase(),
            password: pass_hash,
            password_salt: salt.to_string(),
        })
    }
}

#[derive(Debug, Deserialize, Validate, Serialize)]
pub struct UserForm {
    #[validate(
        length(min = 4, max = 18, message = "username should be 4-18 characters"),
        regex(
            path = "USERNAME_REGEX",
            message = "username should be only made up of letters, numbers, and digits"
        )
    )]
    username: String,
    #[validate(email(message = "please enter a valid email"))]
    email: String,
    #[validate(
        length(min = 8, max = 24, message = "password should be 8-24 characters"),
        regex(
            path = "PASSWORD_REGEX",
            message = "password should be made up of letters, numbers, digits, and the following special characters '@#$%^&-+=()!? '"
        )
    )]
    password: String,
    #[validate(must_match(
        other = "password",
        message = "confirm password should match password"
    ))]
    confirm_password: String,
}

impl UserForm {
    pub fn new(
        username: String,
        email: String,
        password: String,
        confirm_password: String,
    ) -> Self {
        UserForm {
            username: username.to_lowercase(),
            email: email.to_lowercase(),
            password,
            confirm_password,
        }
    }
}
