use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize, Eq)]
pub struct AuthDetails {
    pub token: String,
    pub user: User,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default, Eq)]
pub struct User {
    pub id: Option<String>,
    pub user_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub full_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<Gender>,
    pub dob: Option<String>,
    pub email: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub status: Option<AccountStatus>,
    pub oauth_client: Option<OAuthClientName>,
    pub profile_picture: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub address: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum OAuthClientName {
    Google,
    Github,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum AccountStatus {
    Active,
    Inactive,
    Suspended,
    Deleted,
}
