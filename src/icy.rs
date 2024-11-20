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

pub fn populate_properties(properties: &mut Properties, headers: &[httparse::Header<'_>]) {
    for header in headers {
        let name = header.name.to_lowercase();
        let name = name.as_str();
        let val = std::str::from_utf8(header.value).unwrap_or("");

        // There's a nice list here: https://github.com/ben221199/MediaCast
        // Although, these were taken directly from Icecast's source: https://github.com/xiph/Icecast-Server/blob/master/src/source.c
        match name {
            "user-agent" => properties.uagent = Some(val.to_string()),
            "ice-public" | "icy-pub" | "x-audiocast-public" | "icy-public" => {
                properties.public = val.parse::<usize>().unwrap_or(0) == 1
            }
            "ice-name" | "icy-name" | "x-audiocast-name" => properties.name = Some(val.to_string()),
            "ice-description" | "icy-description" | "x-audiocast-description" => {
                properties.description = Some(val.to_string())
            }
            "ice-url" | "icy-url" | "x-audiocast-url" => properties.url = Some(val.to_string()),
            "ice-genre" | "icy-genre" | "x-audiocast-genre" => {
                properties.genre = Some(val.to_string())
            }
            "ice-bitrate" | "icy-br" | "x-audiocast-bitrate" => {
                properties.bitrate = Some(val.to_string())
            }
            _ => (),
        }
    }
}
