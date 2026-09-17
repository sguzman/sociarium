use serde_json::Value;

pub(crate) fn safe_remote_category(status: u16, body: &[u8]) -> &'static str {
    if let Ok(value) = serde_json::from_slice::<Value>(body) {
        if let Some(category) = structured_category(&value) {
            return category;
        }
    }
    status_category(status)
}

fn structured_category(value: &Value) -> Option<&'static str> {
    if let Some(error) = value.get("error").and_then(Value::as_str) {
        if let Some(category) = oauth_error_category(error) {
            return Some(category);
        }
    }

    if let Some(title) = value.get("title").and_then(Value::as_str) {
        if let Some(category) = x_title_category(title) {
            return Some(category);
        }
    }

    value
        .get("errors")
        .and_then(Value::as_array)
        .and_then(|errors| {
            errors.iter().find_map(|error| {
                error
                    .get("error")
                    .and_then(Value::as_str)
                    .and_then(oauth_error_category)
                    .or_else(|| {
                        error
                            .get("title")
                            .and_then(Value::as_str)
                            .and_then(x_title_category)
                    })
            })
        })
}

fn oauth_error_category(value: &str) -> Option<&'static str> {
    match value {
        "invalid_request" => Some("invalid_request"),
        "invalid_client" => Some("invalid_client"),
        "invalid_grant" => Some("invalid_grant"),
        "unauthorized_client" => Some("unauthorized_client"),
        "unsupported_grant_type" => Some("unsupported_grant_type"),
        "invalid_scope" => Some("invalid_scope"),
        "access_denied" => Some("access_denied"),
        "server_error" => Some("server_error"),
        "temporarily_unavailable" => Some("temporarily_unavailable"),
        _ => None,
    }
}

fn x_title_category(value: &str) -> Option<&'static str> {
    match value {
        "Too Many Requests" => Some("rate_limit"),
        "Unauthorized" => Some("unauthorized"),
        "Forbidden" => Some("forbidden"),
        "Client Forbidden" => Some("client_forbidden"),
        "Not Found" | "Not Found Error" => Some("not_found"),
        _ => None,
    }
}

fn status_category(status: u16) -> &'static str {
    match status {
        400 => "bad_request",
        401 => "unauthorized",
        402 => "payment_required",
        403 => "forbidden",
        404 => "not_found",
        409 => "conflict",
        429 => "rate_limit",
        500..=599 => "server_error",
        _ => "remote_error",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_known_oauth_category_without_free_text() {
        let body = br#"{
            "error":"invalid_grant",
            "error_description":"refresh-secret authorization-code code_verifier-secret"
        }"#;
        assert_eq!(safe_remote_category(400, body), "invalid_grant");
    }

    #[test]
    fn ignores_unrecognized_secret_shaped_remote_values() {
        let body = br#"{
            "error":"access-secret-value",
            "detail":"private-post-text refresh-secret code_verifier-secret"
        }"#;
        assert_eq!(safe_remote_category(400, body), "bad_request");
    }

    #[test]
    fn recognizes_safe_x_title_categories() {
        let body = br#"{
            "title":"Too Many Requests",
            "detail":"private payload that must never be emitted"
        }"#;
        assert_eq!(safe_remote_category(429, body), "rate_limit");
    }

    #[test]
    fn falls_back_to_status_without_echoing_long_remote_text() {
        let body = format!(
            r#"{{"detail":"{}-access-secret-private-profile"}}"#,
            "x".repeat(10_000)
        );
        assert_eq!(
            safe_remote_category(402, body.as_bytes()),
            "payment_required"
        );
    }
}
