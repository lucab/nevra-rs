//! NEVRA parsing (pest-based).

/// Nevra parser with `pest` rules.
#[derive(Parser)]
#[grammar = "nevra.pest"]
pub(crate) struct NevraParser {}
