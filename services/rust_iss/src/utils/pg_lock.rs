use anyhow::Result;
use sqlx::{PgPool, Transaction};

pub async fn run_with_lock<F, Fut, T>(
    pool: &PgPool,
    key: i64,
    f: F,
) -> Result<T>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<T>> + Send,
    T: Send + 'static,
{
    let mut tx = pool.begin().await?;

    let locked: bool = sqlx::query_scalar("SELECT pg_try_advisory_lock($1)")
        .bind(key)
        .fetch_one(&mut *tx)
        .await?;

    if !locked {
        tx.rollback().await?;
        return Err(anyhow::anyhow!("Could not acquire advisory lock"));
    }

    let result = f().await;

    sqlx::query("SELECT pg_advisory_unlock($1)")
        .bind(key)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    result
}
