use bevy_renet::renet::transport::NETCODE_USER_DATA_BYTES;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const MAX_USERNAME_LENGTH: usize = 32;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
    username: String,
    password_hash: String,
}

impl LoginData {
    pub fn new(username: String, password: String) -> Result<Self, LoginDataError> {
        if username.is_empty() {
            return Err(LoginDataError::UsernameEmpty);
        }

        if username.len() > 32 {
            return Err(LoginDataError::UsernameTooLong {
                length: username.len(),
            });
        }

        if password.is_empty() {
            return Err(LoginDataError::PasswordEmpty);
        }

        Ok(Self {
            username,
            password_hash: sha256::digest(password),
        })
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_salted_password(&self, salt: &str) -> String {
        let mut data = self.password_hash.clone();
        data.push_str(salt);
        sha256::digest(&data)
    }

    pub fn as_bytes(&self) -> [u8; NETCODE_USER_DATA_BYTES] {
        let mut buffer = [0; NETCODE_USER_DATA_BYTES];

        let bytes = bincode::serialize(self).unwrap();
        buffer[0] = bytes.len() as u8;
        buffer[1 .. bytes.len()].copy_from_slice(&bytes);

        buffer
    }

    pub fn from_bytes(bytes: &[u8; NETCODE_USER_DATA_BYTES]) -> Result<Self, bincode::Error> {
        let len = bytes[0] as usize;
        let bytes = &bytes[1 .. len + 1];

        bincode::deserialize(bytes)
    }
}

#[derive(Debug, Error)]
pub enum LoginDataError {
    #[error("Username is empty")]
    UsernameEmpty,
    #[error("Username is too long (max {MAX_USERNAME_LENGTH} characters, provided {length})")]
    UsernameTooLong { length: usize },
    #[error("Password is empty")]
    PasswordEmpty,
}
