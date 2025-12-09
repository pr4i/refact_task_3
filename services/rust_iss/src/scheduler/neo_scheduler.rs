use std::time::Duration;

use tracing::error;

use crate::services::space_service::SpaceService;
use crate::utils::pg_lock::run_with_lock;
use crate::AppState;

pub fn run_neo_scheduler(state: AppState) {
    tokio::spawn(async move {
        let pool = state.pool.clone();

        loop {
            let st = state.clone();

            if let Err(e) = run_with_lock(&pool, 1004, move || {
                let st = st.clone();
                async move {
                    let svc = SpaceService::new()?;
                    if let Err(err) = svc.refresh(&st, "neo").await {
                        error!("NEO refresh error: {:?}", err);
                    }
                    Ok(())
                }
            })
            .await
            {
                error!("NEO scheduler lock error: {:?}", e);
            }

            tokio::time::sleep(Duration::from_secs(state.every_neo)).await;
        }
    });
}
