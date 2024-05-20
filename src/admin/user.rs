use crate::{admin::auth::get_jwt_token, db::DB};
use anyhow::anyhow;

use anyhow::Result;
use salvo::{http::cookie::Cookie, prelude::*};
use uuid::Uuid;

#[warn(dead_code)]
async fn create_user_table() -> Result<()> {
    let db = DB.get().ok_or(anyhow!("数据库连接失败"))?;
    let _ = sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS user (
            id SERIAL PRIMARY KEY NOT NULL,
            username VARCHAR(255) NOT NULL,
            password VARCHAR(255) NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(db)
    .await?;
    Ok(())
}

#[warn(dead_code)]
async fn add_user() -> Result<()> {
    let db = DB.get().ok_or(anyhow!("数据库连接失败"))?;
    let username = "admin";
    let password = "admin";
    let uuid = Uuid::new_v4().to_string();

    let _ = sqlx::query!(
        r#"
        INSERT INTO user (id, username, password)
        VALUES ($1, $2, $3)
        "#,
        uuid,
        username,
        password
    )
    .execute(db)
    .await?;

    Ok(())
}

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: String,
    username: String,
    password: String,
    created_at: chrono::NaiveDateTime,
}

#[handler]
pub async fn login_handler(req: &mut Request, res: &mut Response, depot: &mut Depot) {
    let username = req.form::<String>("username").await;
    let password = req.form::<String>("password").await;
    //some none
    if username.is_none() || password.is_none() {
        res.render("参数不全")
    } else {
        let username = username.unwrap();
        let password = password.unwrap();
        if let Ok(user) = check_user(&username, &password).await {
            let token = get_jwt_token(&user.username, &user.id);
            //set cookie
            res.add_cookie(Cookie::new("token", token));
            //redirect
            res.render(Redirect::found("/article_upload"))
        } else {
            res.render("登录失败");
        }
    }
}

async fn check_user(username: &str, password: &str) -> Result<User> {
    let db = DB.get().ok_or(anyhow!("数据库连接失败"))?;
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM user WHERE username = ? AND password = ?
        "#,
    )
    .bind(username)
    .bind(password)
    .fetch_one(db)
    .await?;

    Ok(user)
}
