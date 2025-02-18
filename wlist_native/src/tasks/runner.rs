use tokio::sync::broadcast::Receiver;

use crate::tasks::tasks::TaskState;

pub fn get_change_receiver() -> Receiver<Option<(i64, TaskState)>> {
    unimplemented!()
}

pub fn get_refresh_listener() -> Receiver<()> {
    unimplemented!()
}

pub mod refresh {
    use anyhow::Result;
    use tokio::sync::watch::{Receiver, Sender};

    use crate::common::data::files::confirmations::RefreshConfirmation;
    use crate::common::data::files::progresses::RefreshProgress;
    use crate::tasks::tasks::RefreshTask;

    pub enum RefreshState {
        Confirming {
            file: u64,
            directory: u64,
        },
        Running(RefreshProgress),
    }

    pub async fn run_pending_refresh() -> Result<Option<()>> {
        unimplemented!()
    }

    pub fn get_state_receiver(id: i64) -> Option<Receiver<RefreshState>> {
        unimplemented!()
    }

    pub fn get_state_controller(id: i64) -> Option<Sender<bool>> {
        unimplemented!()
    }

    pub async fn insert_refresh_task(task: RefreshTask, confirmation: Option<RefreshConfirmation>) -> Result<RefreshTask> {
        unimplemented!()
    }
}
