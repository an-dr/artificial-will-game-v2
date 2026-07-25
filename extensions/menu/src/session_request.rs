use crate::level::Level;

/// Concrete lifecycle effect requested by a menu-state transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionRequest {
    /// Replace the optional current level with `next`.
    Replace {
        previous: Option<Level>,
        next: Level,
    },
    /// Stop the current level and return to a menu-only state.
    Stop(Level),
}
