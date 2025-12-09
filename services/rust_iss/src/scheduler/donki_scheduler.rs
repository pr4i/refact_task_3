use std::time::Duration;

use tracing::error;

use crate::services::space_service::SpaceService;
use crate::utils::pg_lock::run_with_lock;
use crate::AppState;

pub fn run_donki_scheduler(state: AppState) {
    tokio::spawn(async move {
        let pool = state.pool.clone();

        loop {
            let st = state.clone();

            // Солнечные вспышки (FLR)
            if let Err(e) = run_with_lock(&pool, 1005, {
                let st = st.clone();
                move || {
                    let st = st.clone();
                    async move {
                        let svc = SpaceService::new()?;
                        if let Err(err) = svc.refresh(&st, "flr").await {
                            error!("DONKI FLR refresh error: {:?}", err);
                        }
                        Ok(())
                    }
                }
            })
            .await
            {
                error!("DONKI FLR scheduler lock error: {:?}", e);
            }

            // CME
            if let Err(e) = run_with_lock(&pool, 1006, {
                let st = st.clone();
                move || {
                    let st = st.clone();
                    async move {
                        let svc = SpaceService::new()?;
                        if let Err(err) = svc.refresh(&st, "cme").await {
                            error!("DONKI CME refresh error: {:?}", err);
                        }
                        Ok(())
                    }
                }
            })
            .await
            {
                error!("DONKI CME scheduler lock error: {:?}", e);
            }

            tokio::time::sleep(Duration::from_secs(state.every_donki)).await;
        }
    });
}
