//! Leptos server-fn error mapping for Spectra ops.

use leptos::prelude::ServerFnError;
use spectra_backend::{is_permission_denied_message, SpectraOpsError};

/// Map a classified Spectra ops error into Leptos `ServerFnError`.
#[must_use]
pub fn to_server_fn_error(err: SpectraOpsError) -> ServerFnError {
    ServerFnError::new(err.to_string())
}

/// Returns true when `err` is a permission denial from [`SpectraOpsError::PermissionDenied`].
#[must_use]
pub fn server_fn_is_permission_denied(err: &ServerFnError) -> bool {
    is_permission_denied_message(&err.to_string())
}
