use std::time::Duration;
use tracing::error;

use crate::{AppState};
use crate::services::osdr_service::OsdrService;

pub async fn run_osdr_scheduler(state: AppState) {
    tokio::spawn(async move {
        let service = OsdrService::new(&state).expect("Failed to init OsdrService");

        loop {
            if let Err(e) = service.sync(&state).await {
                error!("OSDR scheduler error: {:?}", e);
            }

            tokio::time::sleep(Duration::from_secs(state.every_osdr)).await;
        }
    });
}
