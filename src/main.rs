use salvo::catcher::Catcher;
use salvo::prelude::*;
use sc_blog_salvo::{
    admin::admin_router,
    assert::assert_router,
    db::init_db,
    render_html::{handle_404, html_router},
    tera::init_tera,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    init_tera().await;
    init_db().await;
    let router = Router::new()
        .hoop(Logger::new())
        .push(html_router())
        .push(assert_router())
        .push(admin_router());
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    let service = Service::new(router).catcher(Catcher::default().hoop(handle_404));
    Server::new(acceptor).serve(service).await;
}
