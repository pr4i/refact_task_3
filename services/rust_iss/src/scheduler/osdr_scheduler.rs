use std::time::Duration;

use tracing::error;

use crate::services::osdr_service::OsdrService;
use crate::utils::pg_lock::run_with_lock;
use crate::AppState;

pub fn run_osdr_scheduler(state: AppState) {
    tokio::spawn(async move {
        let pool = state.pool.clone();

        loop {
            let st = state.clone();

            if let Err(e) = run_with_lock(&pool, 1002, move || {
                let st = st.clone();
                async move {
                    let svc = OsdrService::new(&st)?;
                    if let Err(err) = svc.sync(&st).await {
                        error!("OSDR sync error: {:?}", err);
                    }
                    Ok(())
                }
            })
            .await
            {
                error!("OSDR scheduler lock error: {:?}", e);
            }

            tokio::time::sleep(Duration::from_secs(state.every_osdr)).await;
        }
    });
}
