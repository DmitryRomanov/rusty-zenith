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
