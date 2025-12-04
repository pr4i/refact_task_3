use std::time::Duration;
use tracing::error;

use crate::{AppState};
use crate::services::space_service::SpaceService;

pub async fn run_apod_scheduler(state: AppState) {
    tokio::spawn(async move {
        let service = SpaceService::new().expect("Failed to init SpaceService");

        loop {
            if let Err(e) = service.refresh(&state, "apod").await {
                error!("APOD scheduler error: {:?}", e);
            }

            tokio::time::sleep(Duration::from_secs(state.every_apod)).await;
        }
    });
}
