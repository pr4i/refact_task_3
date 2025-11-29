use std::time::Duration;
use tracing::error;

use crate::AppState;
use crate::services::space_service::SpaceService;

pub async fn run_spacex_scheduler(state: AppState) {
    tokio::spawn(async move {
        let service = SpaceService::new().expect("Failed to init SpaceService");

        loop {
            if let Err(e) = service.refresh(&state, "spacex").await {
                error!("SpaceX scheduler error: {:?}", e);
            }

            tokio::time::sleep(Duration::from_secs(state.every_spacex)).await;
        }
    });
}
