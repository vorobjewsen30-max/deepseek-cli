use super::*;

pub(super) fn environment_selection_error(err: DeepSeekErr) -> JSONRPCErrorError {
    match err {
        DeepSeekErr::InvalidRequest(message) => invalid_request(message),
        err => internal_error(format!("failed to validate environment selections: {err}")),
    }
}
