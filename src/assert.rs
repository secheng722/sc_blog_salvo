use crate::fs_helper;
use salvo::{
    http::{HeaderValue, ResBody},
    prelude::*,
    writing::Text,
};

#[handler]
async fn handle_css(req: &mut Request, res: &mut Response) {
    let name = req.param("name").unwrap_or("index");
    res.render(Text::Css(fs_helper::read_to_string(&format!(
        "assert/css/{}.css",
        name
    ))));
}

#[handler]
async fn handle_js(req: &mut Request, res: &mut Response) {
    let name = req.param("name").unwrap_or("index");
    res.render(Text::Js(fs_helper::read_to_string(&format!(
        "assert/js/{}.js",
        name
    ))));
}

#[handler]
async fn handle_img(req: &mut Request, res: &mut Response) {
    let name = req.param("name").unwrap_or("index");
    let res_body = ResBody::from(fs_helper::read_to_bytes(&format!("assert/images/{}", name)));
    res.headers_mut()
        .insert("Content-Type", HeaderValue::from_static("image/jpeg"));
    res.body(res_body);
}

pub fn assert_router() -> Router {
    Router::new()
        .push(Router::with_path("/assert/css/<name>.css").get(handle_css))
        .push(Router::with_path("/assert/js/<name>.js").get(handle_js))
        .push(Router::with_path("/assert/images/<name>").get(handle_img))
}
