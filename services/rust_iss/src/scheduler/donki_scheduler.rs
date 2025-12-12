use crate::{app_state::AppState, services::space_service::SpaceService};
use tokio::time::{sleep, Duration};
use tracing::{info, error};

pub async fn start(state: AppState) {
    let svc = match SpaceService::new(&state) {
        Ok(v) => v,
        Err(e) => {
            error!("DONKI scheduler init failed: {e}");
            return;
        }
    };

    loop {
        if let Err(e) = svc.refresh(&state, "flr").await {
            info!("DONKI FLR error: {}", e);
        }

        if let Err(e) = svc.refresh(&state, "cme").await {
            info!("DONKI CME error: {}", e);
        }

        sleep(Duration::from_secs(state.every_donki)).await;
    }
}
