const MOVE_SPEED: f32 = 160.0;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HeldKeys {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl HeldKeys {
    pub fn set(&mut self, key: &str, is_down: bool) {
        match key {
            "W" | "Up" => self.up = is_down,
            "S" | "Down" => self.down = is_down,
            "A" | "Left" => self.left = is_down,
            "D" | "Right" => self.right = is_down,
            _ => {}
        }
    }

    pub fn velocity(&self) -> (f32, f32) {
        let x = (self.right as i32 - self.left as i32) as f32 * MOVE_SPEED;
        let y = (self.down as i32 - self.up as i32) as f32 * MOVE_SPEED;
        (x, y)
    }
}

#[cfg(test)]
mod tests;
