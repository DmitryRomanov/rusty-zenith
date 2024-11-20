use regex::Regex;

#[derive(Clone)]
pub struct Query {
    field: String,
    value: String,
}

pub fn extract_queries(url: &str) -> (&str, Option<Vec<Query>>) {
    if let Some((path, last)) = url.split_once("?") {
        let mut queries: Vec<Query> = Vec::new();
        for field in last.split('&') {
            // decode doesn't treat + as a space
            if let Some((name, value)) = field.replace("+", " ").split_once('=') {
                let name = urlencoding::decode(name);
                let value = urlencoding::decode(value);

                if let Ok(field) = name {
                    if let Ok(value) = value {
                        queries.push(Query { field, value });
                    }
                }
            }
        }

        (path, Some(queries))
    } else {
        (url, None)
    }
}

pub fn get_queries_for(keys: Vec<&str>, queries: &[Query]) -> Vec<Option<String>> {
    let mut results = vec![None; keys.len()];

    for query in queries {
        let field = query.field.as_str();
        for (i, key) in keys.iter().enumerate() {
            if &field == key {
                results[i] = Some(query.value.to_string());
            }
        }
    }

    results
}

pub fn get_header<'a>(key: &str, headers: &[httparse::Header<'a>]) -> Option<&'a [u8]> {
    let key = key.to_lowercase();
    for header in headers {
        if header.name.to_lowercase() == key {
            return Some(header.value);
        }
    }
    None
}

pub fn get_basic_auth(headers: &[httparse::Header]) -> Option<(String, String)> {
    if let Some(auth) = get_header("Authorization", headers) {
        let reg =
            Regex::new(r"^Basic ((?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?)$")
                .unwrap();
        if let Some(capture) = reg.captures(std::str::from_utf8(&auth).unwrap()) {
            if let Some((name, pass)) = std::str::from_utf8(&base64::decode(&capture[1]).unwrap())
                .unwrap()
                .split_once(":")
            {
                return Some((String::from(name), String::from(pass)));
            }
        }
    }
    None
}
