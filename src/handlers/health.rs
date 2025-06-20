/// This function handles kubernetes (health check) readyz requests on GET /healthcheck/readyz,
/// returns true if application is ready, false otherwise
pub(crate) fn readyz() -> bool {
    let ready_status = true;
    
    ready_status
}

/// This function handles kubernetes (health check) livez requests on GET /healthcheck/livez,
/// returns true if application is alive, false otherwise
pub(crate) fn livez() -> bool {
    let live_status = true;

    live_status
}

/// This function handles requests for information about the application.
/// It should return the information as `String` for the requested key or otherwise, `None`.
pub(crate) fn infoz(key: &String) -> Option<String> {
    match key.as_str() { 
        "version" => Some("1.0.0".to_string()),
        "openapi-version" => Some("3.0.3".to_string()),
        _ => None
    }
}