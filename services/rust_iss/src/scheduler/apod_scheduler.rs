use std::time::Duration;

use tracing::error;

use crate::services::space_service::SpaceService;
use crate::utils::pg_lock::run_with_lock;
use crate::AppState;

pub fn run_apod_scheduler(state: AppState) {
    tokio::spawn(async move {
        let pool = state.pool.clone();

        loop {
            let st = state.clone();

            if let Err(e) = run_with_lock(&pool, 1003, move || {
                let st = st.clone();
                async move {
                    let svc = SpaceService::new()?;
                    if let Err(err) = svc.refresh(&st, "apod").await {
                        error!("APOD refresh error: {:?}", err);
                    }
                    Ok(())
                }
            })
            .await
            {
                error!("APOD scheduler lock error: {:?}", e);
            }

            tokio::time::sleep(Duration::from_secs(state.every_apod)).await;
        }
    });
}
