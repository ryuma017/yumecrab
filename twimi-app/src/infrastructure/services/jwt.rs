use std::sync::Arc;

use anyhow::Context as _;
use jsonwebtoken::{encode, EncodingKey};
use serde::{Deserialize, Serialize};
use shaku::Component;

use twimi_core::domain::services::JwtEncoder;

use crate::infrastructure::Secret;

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    name: String,
}

#[derive(Component)]
#[shaku(interface = JwtEncoder)]
pub struct JwtEncoderImpl {
    #[shaku(inject)]
    secret: Arc<dyn Secret>,
}

impl JwtEncoder for JwtEncoderImpl {
    fn encode(&self, username: &str) -> Result<String, anyhow::Error> {
        encode(
            &jsonwebtoken::Header::default(),
            &Claims {
                name: username.to_owned(),
            },
            &EncodingKey::from_secret(self.secret.get_secret().as_bytes()),
        )
        .context("Failed to encode JWT.")
    }
}