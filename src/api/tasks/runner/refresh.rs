use crate::api::common::data::files::confirmations::FRefreshConfirmation;
use crate::api::common::data::files::progresses::FRefreshProgress;
use crate::api::common::exceptions::UniverseError;
use crate::api::tasks::tasks::FRefreshTask;
use crate::frb_generated::StreamSink;
use crate::utils::channel::watch_to_stream;
use crate::utils::{convert_option_bool, o2o};

#[flutter_rust_bridge::frb(non_opaque)]
/// Represent the running refresh task progress.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::tasks::runner::refresh::RefreshState)]
pub enum FRefreshState {
    Confirming {
        file: u64,
        directory: u64,
    },
    Running(
        #[map(o2o::map(~))]
        FRefreshProgress,
    ),
}

/// Run next pending refresh task.
///
/// Return false means no more pending task.
pub async fn run_pending_refresh() -> Result<bool, UniverseError> {
    wlist_native::tasks::runner::refresh::run_pending_refresh().await.map(convert_option_bool).map_err(Into::into)
}

/// Register a refresh task progress listener.
pub async fn get_state_receiver(id: i64, stream: StreamSink<FRefreshState>) {
    if let Some(mut receiver) = wlist_native::tasks::runner::refresh::get_state_receiver(id) {
        loop {
            if watch_to_stream(&mut receiver, &stream, Into::into).await {
                break;
            }
        }
    }
}

/// Insert a new refresh task with confirmation.
pub async fn insert_refresh_task(task: FRefreshTask, confirmation: Option<FRefreshConfirmation>) -> Result<FRefreshTask, UniverseError> {
    wlist_native::tasks::runner::refresh::insert_refresh_task(task.into(), o2o::map_option(confirmation)).await.map(Into::into).map_err(Into::into)
}

/// Pause a refresh task.
///
/// Return false if the task is not a [Running](crate::api::tasks::tasks::FTaskState::Running)
/// or [Pending](crate::api::tasks::tasks::FTaskState::Pending) refresh task.
pub async fn pause_refresh_task(id: i64) -> Result<bool, UniverseError> {
    wlist_native::tasks::runner::refresh::pause_refresh_task(id).await.map(convert_option_bool).map_err(Into::into)
}

/// Resume a refresh task.
///
/// Return false if the task is not a [Running](crate::api::tasks::tasks::FTaskState::Running)
/// or [Pausing](crate::api::tasks::tasks::FTaskState::Pausing) refresh task.
pub async fn resume_refresh_task(id: i64) -> Result<bool, UniverseError> {
    wlist_native::tasks::runner::refresh::resume_refresh_task(id).await.map(convert_option_bool).map_err(Into::into)
}

/// Cancel a refresh task.
///
/// Return false if the task is not a [Running](crate::api::tasks::tasks::FTaskState::Running)
/// or [Pending](crate::api::tasks::tasks::FTaskState::Pending)
/// or [Pausing](crate::api::tasks::tasks::FTaskState::Pausing) refresh task.
pub async fn cancel_refresh_task(id: i64) -> Result<bool, UniverseError> {
    wlist_native::tasks::runner::refresh::cancel_refresh_task(id).await.map(convert_option_bool).map_err(Into::into)
}
