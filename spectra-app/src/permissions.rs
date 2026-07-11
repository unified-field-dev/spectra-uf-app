//! Permission manifest deferred until upstream Orbital exports manifest types.

/// Placeholder permission enum for the exported Spectra app shell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpectraPermission {
    QueryTable,
}

impl SpectraPermission {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::QueryTable => "spectra.query_table",
        }
    }
}
