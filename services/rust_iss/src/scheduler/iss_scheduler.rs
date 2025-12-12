use crate::{app_state::AppState, services::iss_service::IssService};
use tokio::time::{sleep, Duration};
use tracing::{info, error};

pub async fn start(state: AppState) {
    let svc = match IssService::new(&state) {
        Ok(v) => v,
        Err(e) => {
            error!("ISS scheduler init failed: {e}");
            return;
        }
    };

    loop {
        if let Err(e) = svc.fetch_and_store(&state).await {
            info!("ISS scheduler error: {}", e);
        }

        sleep(Duration::from_secs(state.every_iss)).await;
    }
}
