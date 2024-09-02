use std::sync::Arc;

use chrono::Utc;
use dashmap::DashMap;

use crate::common::data::files::confirmations::DownloadConfirmation;
use crate::common::data::files::information::{FileDetailsInformation, FileInformation};

pub(crate) mod lanzou;

#[derive(Clone)]
pub(crate) struct Node {
    pub storages: Arc<DashMap<i64, Node>>,
    pub info: FileInformation,
    pub md5: Option<Arc<String>>,
    pub children: Option<Vec<Node>>,
}

impl Node {
    pub(crate) fn root(storages: &Arc<DashMap<i64, Node>>, id: i64) -> Self {
        Self {
            storages: storages.clone(),
            info: FileInformation {
                id,
                parent_id: id,
                name: Arc::new("".to_string()),
                is_directory: true,
                size: Some(0),
                create_time: Some(Utc::now()),
                update_time: Some(Utc::now()),
            },
            md5: None,
            children: Some(vec![]),
        }
    }

    pub(crate) fn children(&self) -> &Vec<Node> {
        assert!(self.info.is_directory, "getting children in a file");
        self.children.as_ref().unwrap()
    }

    pub(crate) fn children_mut(&mut self) -> &mut Vec<Node> {
        assert!(self.info.is_directory, "getting children in a file");
        self.children.as_mut().unwrap()
    }

    pub(crate) fn detail(&self, thumbnail: Option<DownloadConfirmation>) -> FileDetailsInformation {
        let mut path = vec![];
        let mut node = self.info.parent_id;
        loop {
            let parent = self.storages.get(&node).unwrap();
            if parent.info.id == parent.info.parent_id { break; }
            node = parent.info.parent_id;
            path.push(parent.info.name.to_string());
        }
        path.reverse();
        FileDetailsInformation {
            basic: self.info.clone(),
            md5: self.md5.clone(),
            path, thumbnail,
        }
    }
}
