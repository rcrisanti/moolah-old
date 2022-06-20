use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Queryable, Debug, Identifiable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    password: String,
}

impl From<UserForm> for NewUser {
    fn from(form: UserForm) -> Self {
        NewUser {
            username: form.username,
            email: form.email,
            password: form.password,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserForm {
    pub username: String,
    pub email: String,
    pub password: String,
    // pub confirm_password: String,
}
