use std::time::Duration;
use tracing::error;

use crate::{AppState};
use crate::services::iss_service::IssService;
use crate::utils::pg_lock::run_with_lock;

pub fn run_iss_scheduler(state: AppState) {
    tokio::spawn(async move {
        loop {
            let pool = state.pool.clone();
            let st = state.clone();

            let _ = run_with_lock(&pool, 1001, || async move {
                st.iss_service().fetch_and_store(&st).await?;
                Ok(())
            })
            .await;

            tokio::time::sleep(Duration::from_secs(state.every_iss)).await;
        }
    });
}
