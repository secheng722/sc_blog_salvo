use salvo::prelude::*;
use tera::Context;

use crate::{fs_helper, tera::TERA};
#[handler]
pub async fn handle_404(res: &mut Response, ctrl: &mut FlowCtrl) {
    if let Some(StatusCode::NOT_FOUND) = res.status_code {
        res.render(Text::Html(fs_helper::read_to_string(
            "assert/html/404.html",
        )));
        ctrl.skip_rest();
    }
}

#[handler]
async fn blog_list(res: &mut Response) {
    let tera = TERA
        .get()
        .ok_or(anyhow::anyhow!("Failed to get tera"))
        .unwrap();
    let md_info =
        fs_helper::md_helper::render_md_to_catalog().expect("Failed to render md to catalog");
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
    let tera = TERA
        .get()
        .ok_or(anyhow::anyhow!("Failed to get tera"))
        .unwrap();
    let mut ctx = Context::new();
    let md_content =
        fs_helper::md_helper::render_md_to_html(name).expect("Failed to render md to html");
    ctx.insert("post", &md_content);
    let rendered = tera
        .render("blog.html", &ctx)
        .expect("Failed to render template");
    res.render(Text::Html(rendered));
}

pub fn html_router() -> Router {
    Router::new()
        .get(blog_list)
        .push(Router::with_path("/post/<name>").get(blog_article))
}
