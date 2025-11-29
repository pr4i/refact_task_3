use std::time::Duration;
use tracing::error;

use crate::AppState;
use crate::services::space_service::SpaceService;

pub async fn run_neo_scheduler(state: AppState) {
    tokio::spawn(async move {
        let service = SpaceService::new().expect("Failed to init SpaceService");

        loop {
            if let Err(e) = service.refresh(&state, "neo").await {
                error!("NEO scheduler error: {:?}", e);
            }

            tokio::time::sleep(Duration::from_secs(state.every_neo)).await;
        }
    });
}
