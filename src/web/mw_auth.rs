use crate::{web, Error, Result};
use axum::{http::Request, middleware::Next, response::Response};
use tower_cookies::{Cookie, Cookies};

pub async fn mw_requier_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    // cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
    let auth_token = cookies.get(web::AUTH_TOKEN).map(|c| c.value().to_string());
    println!("{:?}", auth_token);
    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;

    Ok(next.run(req).await)
}
