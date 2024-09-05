use tera::Tera;
use tokio::sync::OnceCell;

pub static TERA: OnceCell<Tera> = OnceCell::const_new();

pub async fn init_tera() {
    let tera = Tera::new("assert/template/*").expect("Failed to compile templates");
    TERA.get_or_init(|| async { tera }).await;
}
