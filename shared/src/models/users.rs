use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Local, NaiveDateTime};
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::schema::users;
use crate::MoolahSharedError;

lazy_static! {
    static ref PASSWORD_REGEX: Regex = Regex::new(r"[a-zA-Z\d@#$%^&-+=()!? ]{8,24}$").unwrap();
    static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9]{4,18}$").unwrap();
}

#[derive(Queryable, Debug, Identifiable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created: NaiveDateTime,
    pub last_login: NaiveDateTime,
}

impl User {
    pub fn verify_user(
        &self,
        username: String,
        password: String,
    ) -> Result<bool, MoolahSharedError> {
        if username.to_lowercase() != self.username.to_lowercase() {
            return Ok(false);
        }

        let parsed_hash = PasswordHash::new(&self.password)?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    password: String,
    created: NaiveDateTime,
    last_login: NaiveDateTime,
}

impl TryFrom<UserRegisterForm> for NewUser {
    type Error = MoolahSharedError;

    fn try_from(form: UserRegisterForm) -> Result<Self, Self::Error> {
        form.validate()?;

        let salt = SaltString::generate(&mut OsRng);
        let pass_hash = Argon2::default()
            .hash_password(form.password.as_bytes(), &salt)?
            .to_string();

        let now = Local::now().naive_utc();

        Ok(NewUser {
            username: form.username.to_lowercase(),
            email: form.email.to_lowercase(),
            password: pass_hash,
            created: now,
            last_login: now,
        })
    }
}

#[derive(Debug, Deserialize, Validate, Serialize)]
pub struct UserRegisterForm {
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

impl UserRegisterForm {
    pub fn new(
        username: String,
        email: String,
        password: String,
        confirm_password: String,
    ) -> Self {
        UserRegisterForm {
            username: username.to_lowercase(),
            email: email.to_lowercase(),
            password,
            confirm_password,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserLoginRequestForm {
    pub username: String,
}

impl UserLoginRequestForm {
    pub fn new(username: String) -> Self {
        UserLoginRequestForm {
            username: username.to_lowercase(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserAccount {
    pub username: String,
    pub email: String,
    pub created: NaiveDateTime,
    pub last_login: NaiveDateTime,
}

impl From<User> for UserAccount {
    fn from(user: User) -> Self {
        UserAccount {
            username: user.username,
            email: user.email,
            created: user.created,
            last_login: user.last_login,
        }
    }
}
