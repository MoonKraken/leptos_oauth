use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AuthContext {
    sub: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub email: String,
    pub aud: String,
    pub iss: String,
    pub exp: u64,
}
