use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::extract::FromRequestParts;
use axum::http::header;
use axum::http::request::Parts;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTToken {
    pub id: u64,
    pub username: String,
    pub permissions: Vec<String>,
    exp: usize,
    iat: usize,
    aud: String,

}

impl JWTToken {
    pub fn new(id: u64, username: &str, permissions: Vec<String>) -> JWTToken {
        let now = SystemTime::now();
        //过期时间
        let m30 = Duration::from_secs(1800000);
        let now = now.duration_since(UNIX_EPOCH).expect("获取系统时间失败");

        JWTToken {
            id,
            username: String::from(username),
            permissions,
            aud: String::from("rust_admin"), // (audience)：受众
            exp: (now + m30).as_secs() as usize,
            iat: now.as_secs() as usize,  // (Issued At)：签发时间
            // iss: String::from("code"),     // (issuer)：签发人
            // nbf: now.as_secs() as usize,  // (Not Before)：生效时间
            // sub: String::from("rust_admin"), // (subject)：主题
            // jti: String::from("ignore"),  // (JWT ID)：编号
        }
    }

    /// create token
    /// secret: your secret string
    pub fn create_token(&self, secret: &str) -> Result<String, Error> {
        return match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(t) => Ok(t),
            Err(e) => Err(Error::Jwt(e)),
        };
    }
    /// verify token invalid
    /// secret: your secret string
    pub fn verify(secret: &str, token: &str) -> Result<JWTToken, Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        // validation.sub = Some("rust_admin".to_string());
        validation.set_audience(&["rust_admin"]);
        validation.set_required_spec_claims(&["exp", "aud"]);
        
        decode::<JWTToken>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        ).map(|c| c.claims)
            .map_err(Error::Jwt)
        
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for JWTToken
where
    S: Send + Sync,
{
    type Rejection = String;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // log::info!("JWTToken from_request_parts");
        let auth_header = parts.headers
            .get(header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok());

        let token = if let Some(auth_header) = auth_header {
            auth_header.to_string().replace("Bearer ", "")
        } else {
            // log::info!("Authorization header not found");
            return Err("Authorization header not found".into());
        };

        // log::info!("token:{}",token);
        let jwt_token_e = JWTToken::verify("123", &token);
        let jwt_token = match jwt_token_e {
            Ok(data) => { data }
            Err(err) => {
                // log::error!("Token verify error:{}",err);
                return Err(format!("Token verify error:{}",err));
            }
        };
        Ok(jwt_token)
        
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::jwt_util::JWTToken;

    #[test]
    fn test_jwt() {
        let jwt = JWTToken::new(1, "code", vec![]);
        let res = jwt.create_token("123") ;
        println!("{:?}",res);
        let token = JWTToken::verify("123", &res.unwrap());
        println!("{:?}",token)

    }
}