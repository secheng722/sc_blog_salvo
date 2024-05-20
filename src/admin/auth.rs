use jsonwebtoken::EncodingKey;
use salvo::{
    http::cookie::time::OffsetDateTime,
    jwt_auth::{ConstDecoder, CookieFinder, HeaderFinder, JwtAuth, QueryFinder},
};
use serde::{Deserialize, Serialize};

pub const JWT_SECRET: &str = "7dgfj8cfpic6yc81";

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub username: String,
    pub user_id: String,
    pub exp: i64,
}

pub fn get_jwt_token(username: &str, user_id: &str) -> String {
    let exp = OffsetDateTime::now_utc().unix_timestamp() + 60 * 60 * 24 * 7;
    let claims = JwtClaims {
        username: username.to_string(),
        user_id: user_id.to_string(),
        exp,
    };
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
    .unwrap()
}

pub fn jwt_middleware() -> JwtAuth<JwtClaims, ConstDecoder> {
    let auth_handler: JwtAuth<JwtClaims, _> =
        JwtAuth::new(ConstDecoder::from_secret(JWT_SECRET.as_bytes()))
            .finders(vec![
                Box::new(HeaderFinder::new()),
                Box::new(QueryFinder::new("token")),
                Box::new(CookieFinder::new("token")),
            ])
            .force_passed(false);
    auth_handler
}
