use salvo::prelude::*;
use tera::Context;

use crate::{admin::auth::jwt_middleware, tera::TERA};

pub mod article;
pub mod auth;
pub mod user;

#[handler]
async fn login(req: &mut Request, res: &mut Response) {
    let tera = TERA
        .get()
        .ok_or(anyhow::anyhow!("Failed to get tera"))
        .unwrap();
    if let Some(token) = req.cookie("token") {
        if auth::decode_token(token.value()) {
            res.render(Redirect::found("/article_upload"));
            return;
        }
    }
    let rendered = tera
        .render("login.html", &Context::new())
        .expect("Failed to render template");
    res.render(Text::Html(rendered));
}

#[handler]
async fn article_upload(res: &mut Response) {
    let tera = TERA
        .get()
        .ok_or(anyhow::anyhow!("Failed to get tera"))
        .unwrap();
    let rendered = tera
        .render("article_upload.html", &Context::new())
        .expect("Failed to render template");
    res.render(Text::Html(rendered));
}

pub fn admin_router() -> Router {
    let mut no_auth_router = vec![Router::with_path("/login")
        .get(login)
        .post(user::login_handler)];
    let mut auth_router = vec![Router::with_path("/article_upload")
        .get(article_upload)
        .post(article::upload)];
    Router::new().append(&mut no_auth_router).push(
        Router::new()
            .append(&mut auth_router)
            .hoop(jwt_middleware()),
    )
}
