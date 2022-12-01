use std::path::PathBuf;
use axum::{Extension, Router};
use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use crate::ServerContext;

async fn restore(
    Path(key): Path<String>,
    Extension(mut ctx): Extension<ServerContext>
) -> impl IntoResponse {
    use handlebars::Handlebars;
    use serde_json::json;
    use crypto::util::fixed_time_eq;

    let reg = Handlebars::new();
    let restore_failure = include_str!("../../templates/restore_failure.html");
    let restore_success = include_str!("../../templates/restore_success.html");

    tracing::info!("PROVIDED RESTORE KEY: {key}");

    if !fixed_time_eq(key.as_bytes(), ctx.restore_key.as_bytes()) {
        return Html(reg.render_template(restore_failure, &json!({
            "error_text": "Invalid restore key"
        })).unwrap());
    }

    async fn execute_restore(ctx: &mut ServerContext) -> anyhow::Result<()> {
        let entries = std::fs::read_to_string("./restore/order")?;

        let entries = entries
            .split('\n')
            .map(|s| s.trim())
            .filter(|s| *s != "");

        for entry in entries {
            let path = PathBuf::from("./restore").join(entry);

            tracing::info!("EXECUTING RESTORE FILE: {}", path.display());

            let contents = std::fs::read_to_string(&path)?;

            if contents.contains("$$") {
              sqlx::query(&contents).execute(&ctx.pool).await?;
            } else {
              for query in std::fs::read_to_string(&path)?.split(';') {
                sqlx::query(&query).execute(&ctx.pool).await?;
              }
            }
        }

        Ok(())
    }

    match execute_restore(&mut ctx).await {
        Ok(_) => {},
        Err(err) => return Html(reg.render_template(restore_failure, &json!({
            "error_text": format!("{:?}", err)
        })).unwrap())
    }

    Html(restore_success.to_owned())
}

pub fn route(router: Router) -> Router {
    router.route("/restore/:id", get(restore).post(restore))
}
