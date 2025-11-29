use std::time::Duration;
use tracing::error;

use crate::AppState;
use crate::services::space_service::SpaceService;

pub async fn run_donki_scheduler(state: AppState) {
    tokio::spawn(async move {
        let service = SpaceService::new().expect("Failed to init SpaceService");

        loop {
            // FLR
            let _ = service.refresh(&state, "flr").await;
            // CME
            let _ = service.refresh(&state, "cme").await;

            tokio::time::sleep(Duration::from_secs(state.every_donki)).await;
        }
    });
}
