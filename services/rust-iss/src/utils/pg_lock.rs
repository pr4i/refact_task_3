use sqlx::PgPool;
use anyhow::Result;
use tracing::{error, info};

/// Получить advisory lock.
/// Возвращает true, если лок получен.
pub async fn try_lock(pool: &PgPool, id: i64) -> Result<bool> {
    let row: (bool,) = sqlx::query_as(
        "SELECT pg_try_advisory_lock($1)"
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

/// Освободить advisory lock.
pub async fn unlock(pool: &PgPool, id: i64) -> Result<()> {
    let _ = sqlx::query("SELECT pg_advisory_unlock($1)")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Выполнить задачу с advisory lock.
/// Если лок уже держится — возвращаем Ok(false).
pub async fn run_with_lock<F, Fut>(
    pool: &PgPool,
    lock_id: i64,
    task: F,
) -> Result<bool>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<()>>,
{
    if !try_lock(pool, lock_id).await? {
        info!("Task skipped (lock_id={}) — already running", lock_id);
        return Ok(false);
    }

    let result = task().await;

    if let Err(err) = unlock(pool, lock_id).await {
        error!("Failed to unlock advisory lock {}: {:?}", lock_id, err);
    }

    result?;

    Ok(true)
}
