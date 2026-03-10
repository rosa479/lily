/// Unique identifier generation for temporary variables.
///
/// Produces names like "tmp.0", "tmp.1", etc.
/// The "." ensures no conflict with real C identifiers.

use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Generate a fresh temporary variable name.
pub fn make_temporary() -> String {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("tmp.{}", n)
}

/// Reset the counter (useful for tests).
#[cfg(test)]
pub fn reset() {
    COUNTER.store(0, Ordering::SeqCst);
}
