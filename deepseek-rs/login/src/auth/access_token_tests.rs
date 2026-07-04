use super::*;

#[test]
fn classifies_personal_access_tokens_by_prefix() {
    assert!(matches!(
        classify_codex_access_token("at-example"),
        DeepSeekAccessToken::PersonalAccessToken("at-example")
    ));
    assert!(matches!(
        classify_codex_access_token("header.payload.signature"),
        DeepSeekAccessToken::AgentIdentityJwt("header.payload.signature")
    ));
}
