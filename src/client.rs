use serde::Serialize;
use std::sync::Arc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct Client {
    pub source: RwLock<String>,
    pub sender: RwLock<UnboundedSender<Arc<Vec<u8>>>>,
    pub receiver: RwLock<UnboundedReceiver<Arc<Vec<u8>>>>,
    pub buffer_size: RwLock<usize>,
    pub properties: Properties,
    pub stats: RwLock<Stats>,
}

#[derive(Serialize, Clone)]
pub struct Stats {
    pub start_time: u64,
    pub bytes_sent: usize,
}

#[derive(Serialize, Clone)]
pub struct Properties {
    pub id: Uuid,
    pub uagent: Option<String>,
    pub metadata: bool,
}
