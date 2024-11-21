use anyhow::Result;

use crate::tasks::data::{TaskListInformation, TaskStateFilter, TasksFilter};
use crate::tasks::tasks::{TaskBase, TaskState};
use crate::tasks::Task;

pub async fn tasks_select(id: i64) -> Result<Option<Task>> {
    unimplemented!()
}

pub async fn tasks_select_parents(id: i64) -> Result<Vec<Task>> {
    unimplemented!()
}

pub async fn tasks_select_children(id: i64, filter: TasksFilter, state_filter: TaskStateFilter) -> Result<Vec<Task>> {
    unimplemented!()
}

pub async fn tasks_select_list(filter: TasksFilter, state_filter: TaskStateFilter, offset: u64, limit: usize) -> Result<TaskListInformation> {
    unimplemented!()
}

pub async fn tasks_insert(tasks: Vec<Task>) -> Result<Vec<Task>> {
    unimplemented!()
}

pub async fn tasks_update(tasks: Vec<(i64, TaskState)>) -> Result<()> {
    unimplemented!()
}

pub async fn tasks_delete(tasks: Vec<i64>) -> Result<()> {
    unimplemented!()
}

pub async fn tasks_delete_all(filter: TasksFilter, state_filter: TaskStateFilter) -> Result<()> {
    unimplemented!()
}


pub async fn tasks_select_refresh(storage: i64, directory: i64) -> Result<Vec<TaskBase>> {
    unimplemented!()
}
