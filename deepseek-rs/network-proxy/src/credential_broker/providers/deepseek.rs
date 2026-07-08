use super::CredentialHostBinding;
use super::CredentialProvider;
use super::CredentialSource;
use super::shaped_dummy_value;
use crate::policy::normalize_host;
use rama_http::HeaderMap;
use rama_http::HeaderValue;
use rama_http::header::AUTHORIZATION;
use std::collections::HashMap;

const DEEPSEEK_API_KEY_ENV_VARS: &[&str] = &["DEEPSEEK_API_KEY"];
const DEEPSEEK_HOSTS: &[&str] = &["api.deepseek.com", "api.deepseek.com"];

pub(super) static PROVIDER: CredentialProvider = CredentialProvider {
    context_env_vars: &[],
    sources: &[CredentialSource {
        env_vars: DEEPSEEK_API_KEY_ENV_VARS,
        host_binding: deepseek_binding,
    }],
    dummy_value,
    request_header,
    request_header_value,
    insert_request_header,
};

fn dummy_value(real_value: &str) -> String {
    shaped_dummy_value(real_value, "sk-", 32)
}

fn request_header(headers: &HeaderMap) -> Option<&HeaderValue> {
    headers.get(AUTHORIZATION)
}

fn request_header_value(value: &str) -> Option<HeaderValue> {
    HeaderValue::from_str(&format!("Bearer {value}")).ok()
}

fn insert_request_header(headers: &mut HeaderMap, value: HeaderValue) {
    headers.insert(AUTHORIZATION, value);
}

fn deepseek_binding(_: &HashMap<String, String>) -> Option<CredentialHostBinding> {
    Some(CredentialHostBinding::HostPattern {
        exact_hosts: DEEPSEEK_HOSTS,
        suffixes: &[],
    })
}
