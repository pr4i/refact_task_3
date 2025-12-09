use std::time::Duration;

use tracing::error;

use crate::services::iss_service::IssService;
use crate::utils::pg_lock::run_with_lock;
use crate::AppState;

pub fn run_iss_scheduler(state: AppState) {
    // Запускаем отдельную таску
    tokio::spawn(async move {
        let pool = state.pool.clone();

        loop {
            let st = state.clone();

            // Берём advisory lock и выполняем работу
            if let Err(e) = run_with_lock(&pool, 1001, move || {
                let st = st.clone();
                async move {
                    let svc = IssService::new(&st)?;
                    if let Err(err) = svc.fetch_and_store(&st).await {
                        error!("ISS scheduler error: {:?}", err);
                    }
                    // ВАЖНО: вернуть Result<()>, иначе тип не сойдётся
                    Ok(())
                }
            })
            .await
            {
                error!("ISS scheduler lock error: {:?}", e);
            }

            tokio::time::sleep(Duration::from_secs(state.every_iss)).await;
        }
    });
}
