//! Typed Spectra ops errors for server and UI boundaries.
//!
//! Variants classify failures before they are mapped into Leptos `ServerFnError`
//! strings at the `#[server]` boundary. [`is_permission_denied_message`] lets
//! UI layers branch without substring guessing.

use crate::SpectraQueryNameError;

/// Stable prefix for permission-denied operator messages (see [`SpectraOpsError::PermissionDenied`]).
pub const PERMISSION_DENIED_PREFIX: &str = "Permission denied:";

/// Stable prefix for missing Spectra query backend.
pub const BACKEND_NOT_INSTALLED_MESSAGE: &str = "Spectra query backend not installed";

/// Stable prefix for unauthenticated session.
pub const AUTH_REQUIRED_MESSAGE: &str = "Authentication is required for this action";

/// Classified Spectra ops failure before Leptos serialization.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SpectraOpsError {
    /// Session is missing or anonymous.
    AuthRequired,
    /// Gauge denied `spectra.query.{table}` for the resolved actor.
    PermissionDenied {
        /// Permission name that was required (for example `spectra.query.ops.events`).
        permission: String,
    },
    /// Table or metric name failed validation.
    Validation(SpectraQueryNameError),
    /// Host did not install a global Spectra query router.
    BackendNotInstalled,
    /// Explore query execution failed after auth and validation.
    QueryFailed(String),
    /// Higgs or Valence context could not be resolved.
    ContextResolution(String),
}

impl std::fmt::Display for SpectraOpsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuthRequired => write!(f, "{AUTH_REQUIRED_MESSAGE}"),
            Self::PermissionDenied { permission } => write!(
                f,
                "{PERMISSION_DENIED_PREFIX} `{permission}` is required to query this table"
            ),
            Self::Validation(err) => write!(f, "{err}"),
            Self::BackendNotInstalled => write!(f, "{BACKEND_NOT_INSTALLED_MESSAGE}"),
            Self::QueryFailed(msg) | Self::ContextResolution(msg) => f.write_str(msg),
        }
    }
}

impl std::error::Error for SpectraOpsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(err) => Some(err),
            _ => None,
        }
    }
}

impl From<SpectraQueryNameError> for SpectraOpsError {
    fn from(value: SpectraQueryNameError) -> Self {
        Self::Validation(value)
    }
}

/// Returns true when `message` is a classified permission denial from [`SpectraOpsError::PermissionDenied`].
#[must_use]
pub fn is_permission_denied_message(message: &str) -> bool {
    message.contains(PERMISSION_DENIED_PREFIX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permission_denied_display_happy_path() {
        let err = SpectraOpsError::PermissionDenied {
            permission: "spectra.query.ops.events".into(),
        };
        let msg = err.to_string();
        assert!(is_permission_denied_message(&msg));
        assert!(msg.contains("spectra.query.ops.events"));
    }

    #[test]
    fn validation_maps_from_name_error_happy_path() {
        let err = SpectraOpsError::from(SpectraQueryNameError::EmptyTableName);
        assert_eq!(err.to_string(), "Spectra query table name is required");
        assert!(!is_permission_denied_message(&err.to_string()));
    }

    #[test]
    fn auth_required_is_not_permission_denied_sad() {
        assert!(!is_permission_denied_message(AUTH_REQUIRED_MESSAGE));
    }

    #[test]
    fn server_fn_wrapped_permission_denied_is_detected_happy_path() {
        let wrapped = format!(
            "error running server function: {PERMISSION_DENIED_PREFIX} `spectra.query.t` is required"
        );
        assert!(is_permission_denied_message(&wrapped));
    }
}
