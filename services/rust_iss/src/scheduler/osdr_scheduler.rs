use crate::{app_state::AppState, services::osdr_service::OsdrService};
use tokio::time::{sleep, Duration};
use tracing::{info, error};

pub async fn start(state: AppState) {
    let svc = match OsdrService::new(&state) {
        Ok(v) => v,
        Err(e) => {
            error!("OSDR scheduler init failed: {e}");
            return;
        }
    };

    loop {
        if let Err(e) = svc.sync(&state).await {
            info!("OSDR scheduler error: {}", e);
        }

        sleep(Duration::from_secs(state.every_osdr)).await;
    }
}
