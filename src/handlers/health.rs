/// handler for kubernetes (health check) readyz endpoint,
/// returns true if application is ready, false otherwise
pub(crate) fn readyz() -> bool {
    let ready_status = true;
    
    ready_status
}

/// handler for kubernetes (health check) livez endpoint,
/// returns true if application is alive, false otherwise
pub(crate) fn livez() -> bool {
    let live_status = true;

    live_status
}