use lazy_static::lazy_static;
use salvo::http::ResBody;
use salvo::prelude::*;
use salvo::{catcher::Catcher, http::HeaderValue};
use sc_blog_salvo::fs_helper::{
    self, md_helper::render_md_to_catalog, md_helper::render_md_to_html,
};
use std::sync::Arc;
use tera::{Context, Tera};

#[handler]
async fn handle_404(res: &mut Response, ctrl: &mut FlowCtrl) {
    if let Some(StatusCode::NOT_FOUND) = res.status_code {
        res.render(Text::Html(fs_helper::read_to_string(
            "assert/html/404.html",
        )));
        ctrl.skip_rest();
    }
}

#[handler]
async fn handle_css(req: &mut Request, res: &mut Response) {
    let name = req.param("name").unwrap_or("index");
    tracing::info!("css_name: {}", name);
    res.render(Text::Css(fs_helper::read_to_string(&format!(
        "assert/css/{}.css",
        name
    ))));
}

#[handler]
async fn handle_js(req: &mut Request, res: &mut Response) {
    let name = req.param("name").unwrap_or("index");
    tracing::info!("js_name: {}", name);
    res.render(Text::Js(fs_helper::read_to_string(&format!(
        "assert/js/{}.js",
        name
    ))));
}

#[handler]
async fn handle_img(req: &mut Request, res: &mut Response) {
    let name = req.param("name").unwrap_or("index");
    tracing::info!("img_name: {}", name);
    let res_body = ResBody::from(fs_helper::read_to_bytes(&format!("assert/images/{}", name)));
    res.headers_mut()
        .insert("Content-Type", HeaderValue::from_static("image/jpeg"));
    res.body(res_body);
}

#[handler]
async fn blog_list(res: &mut Response) {
    //使用tera模版引擎
    let tera = TERA.clone();
    let md_info = render_md_to_catalog().expect("Failed to render md to catalog");
    let mut ctx = Context::new();
    ctx.insert("posts", &md_info);
    let rendered = tera
        .render("index.html", &ctx)
        .expect("Failed to render template");
    res.render(Text::Html(rendered));
}

#[handler]
async fn blog_article(req: &mut Request, res: &mut Response) {
    let name = req.param("name").unwrap_or("index");
    let tera = TERA.clone();
    let mut ctx = Context::new();
    let md_content = render_md_to_html(name).expect("Failed to render md to html");
    ctx.insert("post", &md_content);
    let rendered = tera
        .render("blog.html", &ctx)
        .expect("Failed to render template");
    res.render(Text::Html(rendered));
}

lazy_static! {
    static ref TERA: Arc<Tera> = {
        let tera = Tera::new("assert/template/*").expect("Failed to compile templates");
        Arc::new(tera)
    };
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let router = Router::new()
        .get(blog_list)
        .push(Router::with_path("/list").get(blog_list))
        .push(Router::with_path("/post/<name>").get(blog_article))
        .push(Router::with_path("/assert/css/<name>.css").get(handle_css))
        .push(Router::with_path("/assert/js/<name>.js").get(handle_js))
        .push(Router::with_path("/assert/images/<name>").get(handle_img));
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    let service = Service::new(router).catcher(Catcher::default().hoop(handle_404));
    Server::new(acceptor).serve(service).await;
}
