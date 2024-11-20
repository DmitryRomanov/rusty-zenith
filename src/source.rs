use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::client;
use crate::icy;

// TODO Add something determining if a source is a relay, or any other kind of source, for that matter
// TODO Implement hidden sources
pub struct Source {
    // Is setting the mountpoint in the source really useful, since it's not like the source has any use for it
    pub mountpoint: String,
    pub properties: icy::Properties,
    pub metadata: Option<icy::Metadata>,
    pub metadata_vec: Vec<u8>,
    pub clients: HashMap<Uuid, Arc<RwLock<client::Client>>>,
    pub burst_buffer: Vec<u8>,
    pub stats: RwLock<Stats>,
    pub fallback: Option<String>,
    // Not really sure how else to signal when to disconnect the source
    pub disconnect_flag: bool,
}

impl Source {
    pub fn new(mountpoint: String, properties: icy::Properties) -> Source {
        Source {
            mountpoint,
            properties,
            metadata: None,
            metadata_vec: vec![0],
            clients: HashMap::new(),
            burst_buffer: Vec::new(),
            stats: RwLock::new(Stats {
                start_time: {
                    if let Ok(time) = SystemTime::now().duration_since(UNIX_EPOCH) {
                        time.as_secs()
                    } else {
                        0
                    }
                },
                bytes_read: 0,
                peak_listeners: 0,
            }),
            fallback: None,
            disconnect_flag: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Stats {
    pub start_time: u64,
    pub bytes_read: usize,
    pub peak_listeners: usize,
}
