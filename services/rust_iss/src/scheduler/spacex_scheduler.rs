use std::time::Duration;

use tracing::error;

use crate::services::space_service::SpaceService;
use crate::utils::pg_lock::run_with_lock;
use crate::AppState;

pub fn run_spacex_scheduler(state: AppState) {
    tokio::spawn(async move {
        let pool = state.pool.clone();

        loop {
            let st = state.clone();

            if let Err(e) = run_with_lock(&pool, 1007, move || {
                let st = st.clone();
                async move {
                    let svc = SpaceService::new()?;
                    if let Err(err) = svc.refresh(&st, "spacex").await {
                        error!("SpaceX refresh error: {:?}", err);
                    }
                    Ok(())
                }
            })
            .await
            {
                error!("SpaceX scheduler lock error: {:?}", e);
            }

            tokio::time::sleep(Duration::from_secs(state.every_spacex)).await;
        }
    });
}
