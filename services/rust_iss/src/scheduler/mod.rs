pub mod iss_scheduler;
pub mod osdr_scheduler;
pub mod apod_scheduler;
pub mod neo_scheduler;
pub mod donki_scheduler;
pub mod spacex_scheduler;

use crate::AppState;
use tokio::task;

pub async fn start_schedulers(state: AppState) {
    task::spawn(iss_scheduler::start(state.clone()));
    task::spawn(osdr_scheduler::start(state.clone()));
    task::spawn(apod_scheduler::start(state.clone()));
    task::spawn(neo_scheduler::start(state.clone()));
    task::spawn(donki_scheduler::start(state.clone()));
    task::spawn(spacex_scheduler::start(state.clone()));
}
