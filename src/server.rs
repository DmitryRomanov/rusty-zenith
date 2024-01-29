use crate::client;
use crate::source;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct Server {
    pub sources: HashMap<String, Arc<RwLock<source::Source>>>,
    pub clients: HashMap<Uuid, client::Properties>,
    // TODO Find a better place to put these, for constant time fetching
    pub source_count: usize,
    pub relay_count: usize,
    pub properties: Properties,
    pub stats: Stats
}

impl Server {
    pub fn new(properties: Properties) -> Server {
        Server {
            sources: HashMap::new(),
            clients: HashMap::new(),
            source_count: 0,
            relay_count: 0,
            properties,
            stats: Stats::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Stats {
    pub start_time: u64,
    pub peak_listeners: usize,
    pub session_bytes_sent: usize,
    pub session_bytes_read: usize,
}

impl Stats {
    fn new() -> Stats {
        Stats {
            start_time: 0,
            peak_listeners: 0,
            session_bytes_sent: 0,
            session_bytes_read: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Properties {
    // Ideally, there would be multiple addresses and ports and TLS support
    #[serde(default = "default_property_address")]
    pub address: String,
    #[serde(default = "default_property_port")]
    pub port: u16,
    #[serde(default = "default_property_metaint")]
    pub metaint: usize,
    #[serde(default = "default_property_server_id")]
    pub server_id: String,
    #[serde(default = "default_property_admin")]
    pub admin: String,
    #[serde(default = "default_property_host")]
    pub host: String,
    #[serde(default = "default_property_location")]
    pub location: String,
    #[serde(default = "default_property_description")]
    pub description: String,
    #[serde(default = "default_property_limits")]
    pub limits: Limits,
    #[serde(default = "default_property_users")]
    pub users: Vec<Credential>,
    #[serde(default = "default_property_master_server")]
    pub master_server: Master,
}

impl Properties {
    pub fn new() -> Properties {
        Properties {
            address: default_property_address(),
            port: default_property_port(),
            metaint: default_property_metaint(),
            server_id: default_property_server_id(),
            admin: default_property_admin(),
            host: default_property_host(),
            location: default_property_location(),
            description: default_property_description(),
            limits: default_property_limits(),
            users: default_property_users(),
            master_server: default_property_master_server(),
        }
    }
}

// TODO Add permissions, specifically source, admin, bypass client count, etc
#[derive(Serialize, Deserialize, Clone)]
pub struct Credential {
    pub username: String,
    pub password: String,
}

// Add a total source limit
#[derive(Serialize, Deserialize, Clone)]
pub struct Limits {
    #[serde(default = "default_property_limits_clients")]
    pub clients: usize,
    #[serde(default = "default_property_limits_sources")]
    pub sources: usize,
    #[serde(default = "default_property_limits_total_sources")]
    pub total_sources: usize,
    #[serde(default = "default_property_limits_queue_size")]
    pub queue_size: usize,
    #[serde(default = "default_property_limits_burst_size")]
    pub burst_size: usize,
    #[serde(default = "default_property_limits_header_timeout")]
    pub header_timeout: u64,
    #[serde(default = "default_property_limits_source_timeout")]
    pub source_timeout: u64,
    #[serde(default = "default_property_limits_http_max_length")]
    pub http_max_length: usize,
    #[serde(default = "default_property_limits_http_max_redirects")]
    pub http_max_redirects: usize,
    #[serde(default = "default_property_limits_source_limits")]
    pub source_limits: HashMap<String, source::Limits>,
}

// TODO Add a list of "relay" structs
// Relay structs should have an auth of their own, if provided
// Having a master server is not very good
// It would be much better to add relays through an api or something
#[derive(Serialize, Deserialize, Clone)]
pub struct Master {
    #[serde(default = "default_property_master_server_enabled")]
    pub enabled: bool,
    #[serde(default = "default_property_master_server_url")]
    pub url: String,
    #[serde(default = "default_property_master_server_update_interval")]
    pub update_interval: u64,
    #[serde(default = "default_property_master_server_relay_limit")]
    pub relay_limit: usize,
}

// Default constants
// The default interval in bytes between icy metadata chunks
// The metaint cannot be changed per client once the response has been sent
// https://thecodeartist.blogspot.com/2013/02/shoutcast-internet-radio-protocol.html
const METAINT: usize = 16_000;
// Server that gets sent in the header
const SERVER_ID: &str = "Rusty Zenith 0.1.0";
// Contact information
const ADMIN: &str = "admin@localhost";
// Public facing domain/address
const HOST: &str = "localhost";
// Geographic location. Icecast included it in their settings, so why not
const LOCATION: &str = "1.048596";
// Description of the internet radio
const DESCRIPTION: &str = "Yet Another Internet Radio";

// How many regular sources, not including relays
const SOURCES: usize = 4;
// How many sources can be connected, in total
const MAX_SOURCES: usize = 4;
// How many clients can be connected, in total
const CLIENTS: usize = 400;
// How many bytes a client can have queued until they get disconnected
const QUEUE_SIZE: usize = 102400;
// How many bytes to send to the client all at once when they first connect
// Useful for filling up the client buffer quickly, but also introduces some delay
const BURST_SIZE: usize = 65536;
// How long in milliseconds a client has to send a complete request
const HEADER_TIMEOUT: u64 = 15_000;
// How long in milliseconds a source has to send something before being disconnected
const SOURCE_TIMEOUT: u64 = 10_000;
// The maximum size in bytes of an acceptable http message not including the body
const HTTP_MAX_LENGTH: usize = 8192;
// The maximum number of redirects allowed, when fetching relays from another server/stream
const HTTP_MAX_REDIRECTS: usize = 5;
const ADDRESS: &str = "0.0.0.0";
const PORT: u16 = 8000;

fn default_property_address() -> String {
    ADDRESS.to_string()
}
fn default_property_port() -> u16 {
    PORT
}
fn default_property_metaint() -> usize {
    METAINT
}
fn default_property_server_id() -> String {
    SERVER_ID.to_string()
}
fn default_property_admin() -> String {
    ADMIN.to_string()
}
fn default_property_host() -> String {
    HOST.to_string()
}
fn default_property_location() -> String {
    LOCATION.to_string()
}
fn default_property_description() -> String {
    DESCRIPTION.to_string()
}
fn default_property_users() -> Vec<Credential> {
    vec![
        Credential {
            username: "admin".to_string(),
            password: "hackme".to_string(),
        },
        Credential {
            username: "source".to_string(),
            password: "hackme".to_string(),
        },
    ]
}
fn default_property_limits() -> Limits {
    Limits {
        clients: default_property_limits_clients(),
        sources: default_property_limits_sources(),
        total_sources: default_property_limits_total_sources(),
        queue_size: default_property_limits_queue_size(),
        burst_size: default_property_limits_burst_size(),
        header_timeout: default_property_limits_header_timeout(),
        source_timeout: default_property_limits_source_timeout(),
        source_limits: default_property_limits_source_limits(),
        http_max_length: default_property_limits_http_max_length(),
        http_max_redirects: default_property_limits_http_max_redirects(),
    }
}

pub fn default_property_limits_clients() -> usize {
    CLIENTS
}
fn default_property_limits_sources() -> usize {
    SOURCES
}
fn default_property_limits_total_sources() -> usize {
    MAX_SOURCES
}
fn default_property_limits_queue_size() -> usize {
    QUEUE_SIZE
}
pub fn default_property_limits_burst_size() -> usize {
    BURST_SIZE
}
fn default_property_limits_header_timeout() -> u64 {
    HEADER_TIMEOUT
}
pub fn default_property_limits_source_timeout() -> u64 {
    SOURCE_TIMEOUT
}
fn default_property_limits_http_max_length() -> usize {
    HTTP_MAX_LENGTH
}
fn default_property_limits_http_max_redirects() -> usize {
    HTTP_MAX_REDIRECTS
}
fn default_property_limits_source_mountpoint() -> String {
    "/radio".to_string()
}
fn default_property_limits_source_limits() -> HashMap<String, source::Limits> {
    let mut map = HashMap::new();
    map.insert(
        default_property_limits_source_mountpoint(),
        source::Limits {
            clients: default_property_limits_clients(),
            burst_size: default_property_limits_burst_size(),
            source_timeout: default_property_limits_source_timeout(),
        },
    );
    map
}
fn default_property_master_server() -> Master {
    Master {
        enabled: default_property_master_server_enabled(),
        url: default_property_master_server_url(),
        update_interval: default_property_master_server_update_interval(),
        relay_limit: default_property_master_server_relay_limit(),
    }
}
fn default_property_master_server_enabled() -> bool {
    false
}
fn default_property_master_server_url() -> String {
    format!("http://localhost:{}", default_property_port() + 1)
}
fn default_property_master_server_update_interval() -> u64 {
    120
}
fn default_property_master_server_relay_limit() -> usize {
    SOURCES
}

// TODO Add some sort of permission system
pub fn validate_user(properties: &Properties, username: String, password: String) -> bool {
    for cred in &properties.users {
        if cred.username == username && cred.password == password {
            return true;
        }
    }
    false
}
