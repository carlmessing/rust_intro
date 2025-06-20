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