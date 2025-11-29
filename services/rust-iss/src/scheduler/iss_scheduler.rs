use std::time::Duration;
use tracing::error;

use crate::{AppState};
use crate::services::iss_service::IssService;

pub async fn run_iss_scheduler(state: AppState) {
    tokio::spawn(async move {
        let service = IssService::new(&state).expect("Failed to init IssService");

        loop {
            if let Err(e) = service.fetch_and_store(&state).await {
                error!("ISS scheduler error: {:?}", e);
            }

            tokio::time::sleep(Duration::from_secs(state.every_iss)).await;
        }
    });
}
