use sqlx::SqlitePool;
use tokio::sync::OnceCell;

pub static DB: OnceCell<SqlitePool> = OnceCell::const_new();

pub async fn init_db() {
    // let pool = SqlitePool::connect("sqlite:assert/db/blog.db")
    //     .await
    //     .expect("Failed to connect to db");
    DB.get_or_init(|| async {
        SqlitePool::connect("sqlite:assert/db/blog.db")
            .await
            .expect("Failed to connect to db")
    })
    .await;
}
