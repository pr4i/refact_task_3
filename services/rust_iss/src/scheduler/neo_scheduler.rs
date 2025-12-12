use crate::{app_state::AppState, services::space_service::SpaceService};
use tokio::time::{sleep, Duration};
use tracing::{info, error};

pub async fn start(state: AppState) {
    let svc = match SpaceService::new(&state) {
        Ok(v) => v,
        Err(e) => {
            error!("NEO scheduler init failed: {e}");
            return;
        }
    };

    loop {
        if let Err(e) = svc.refresh(&state, "neo").await {
            info!("NEO scheduler error: {}", e);
        }

        sleep(Duration::from_secs(state.every_neo)).await;
    }
}
