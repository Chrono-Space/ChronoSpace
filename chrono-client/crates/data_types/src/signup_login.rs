use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignupReq {
    pub email: String,
    pub code: String,
    pub password: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LoginReq {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LoginRes {
    pub token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResetPasswordReq {
    pub email: String,
    pub code: String,
    pub password: String,
}
