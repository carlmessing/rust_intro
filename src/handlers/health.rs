use crate::schemas::component_types::InfoZ;

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
/// It should return the information as `String` per key.
pub(crate) fn infoz() -> InfoZ {
    InfoZ {
        title: "Calculator API".to_string(),
        version: "1.0.0".to_string(),
        description: "A simple calculator with basic arithmetic operations.".to_string()
    }
}