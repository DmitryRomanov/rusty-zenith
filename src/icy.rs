use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Properties {
    pub uagent: Option<String>,
    pub public: bool,
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub genre: Option<String>,
    pub bitrate: Option<String>,
    pub content_type: String,
}

#[derive(Serialize, Clone)]
pub struct Metadata {
    pub title: Option<String>,
    pub url: Option<String>,
}

impl Properties {
    pub fn new(content_type: String) -> Properties {
        Properties {
            uagent: None,
            public: false,
            name: None,
            description: None,
            url: None,
            genre: None,
            bitrate: None,
            content_type,
        }
    }
}
