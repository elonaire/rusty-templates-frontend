use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize, Eq)]
pub struct AuthDetails {
    pub token: String,
    pub user: User,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default, Eq)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AccountStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_client: Option<OAuthClientName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct SignUpForm {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Logins {
    #[serde(skip_serializing_if = "Option::is_none", rename = "userName")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "oauthClient")]
    pub oauth_client: Option<OAuthClientName>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginPayload {
    #[serde(rename = "rawUserDetails")]
    pub raw_user_details: Logins,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignUpPayload {
    pub user: SignUpForm
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct SignUpResponse {
    #[serde(rename = "signUp")]
    pub sign_up: User,
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "signIn")]
    pub sign_in: LoginTokens,
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct LoginTokens {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct AuthStatus {
    #[serde(rename = "isAuth")]
    pub is_auth: bool,
    pub sub: String,
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct CheckAuthResponse {
    #[serde(rename = "checkAuth")]
    pub check_auth: AuthStatus,
}
