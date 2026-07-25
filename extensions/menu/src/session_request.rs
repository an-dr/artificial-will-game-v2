use crate::level::Level;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionRequest {
    Replace(Level),
    Stop,
}
