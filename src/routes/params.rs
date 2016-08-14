use route_recognizer::Params;
use url::form_urlencoded;
use webapp::UriValue;

#[derive(Debug)]
pub struct UriParams {
    internal: Params
}

impl UriParams {
    pub fn get(&self, key: &str) -> Option<String> {
        let raw = try_opt!(self.internal.find(key));
        let val = UriValue::bless(raw);
        Some(val.unescape())
    }
}

pub fn url_params_from_route_recognizer(params: Params) -> UriParams {
    UriParams {
        internal: params,
    }
}

pub struct BodyParams {
    data: Vec<u8>,
}

impl BodyParams {
    pub fn as_form(&self) -> FormData {
        FormData {
            internal: form_urlencoded::parse(&self.data).into_owned().collect()
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        ::std::str::from_utf8(&self.data).ok()
    }
}

pub fn body_params_from_data(data: Vec<u8>) -> BodyParams {
    BodyParams {
        data: data,
    }
}

pub struct FormData {
    internal: Vec<(String, String)>
}

impl FormData {
    pub fn get(&self, key: &str) -> Option<String> {
        self.internal.iter()
            .find(|v| v.0 == key)
            .map(|v| v.1.clone())
    }
}
