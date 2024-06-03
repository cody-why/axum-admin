use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response;

use crate::middleware::context::UserContext;
use crate::utils::jwt_util::JWTToken;

pub async fn auth(jwt_token: Result<JWTToken, String>, mut req: Request, next: Next) -> Result<response::Response, StatusCode> {
    log::info!("auth req {:?} {:?}", req.method(), req.uri());
    let path = req.uri().to_string();
    if path.starts_with("/api/login") {
        return Ok(next.run(req).await);
    }
    let jwt_token = match jwt_token {
        Ok(token) => token,
        Err(err) => {
            log::info!("auth failed: {}", err);
            return Err(StatusCode::UNAUTHORIZED)
        }
    };
    
    log::info!("permissions: {:?}",jwt_token.permissions);
    let flag = jwt_token.permissions.iter().any(|permission| permission == &path);
    if flag {
        let context = UserContext {
            id: jwt_token.id,
        };
        req.extensions_mut().insert(context);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }

}