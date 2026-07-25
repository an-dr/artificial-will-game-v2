#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PlayerMode {
    #[default]
    Idle,
    Moving,
    Attacking,
}
