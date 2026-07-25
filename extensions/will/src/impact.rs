use bones_messages::gfx::DrawRect;
use game_messages::HitConfirmed;

const IMPACT_SECONDS: f32 = 0.18;
const IMPACT_LAYER: u8 = 242;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ImpactFeedback {
    last_sequence: Option<u32>,
    marker: Option<Marker>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Marker {
    x: f32,
    y: f32,
    remaining: f32,
}

impl ImpactFeedback {
    pub fn confirm(&mut self, hit: HitConfirmed) {
        if self.last_sequence == Some(hit.sequence) || !hit.x.is_finite() || !hit.y.is_finite() {
            return;
        }
        self.last_sequence = Some(hit.sequence);
        self.marker = Some(Marker {
            x: hit.x,
            y: hit.y,
            remaining: IMPACT_SECONDS,
        });
    }

    pub fn tick(&mut self, dt: f32) {
        let Some(marker) = &mut self.marker else {
            return;
        };
        marker.remaining -= dt.max(0.0);
        if marker.remaining <= f32::EPSILON {
            self.marker = None;
        }
    }

    pub fn rectangles(&self) -> Option<[DrawRect; 2]> {
        let marker = self.marker?;
        let x = marker.x.round() as i32;
        let y = marker.y.round() as i32;
        Some([
            DrawRect {
                x: x - 15,
                y: y - 15,
                w: 30,
                h: 30,
                filled: false,
                color: (255, 211, 82, 255),
                layer: IMPACT_LAYER,
                screen_space: false,
            },
            DrawRect {
                x: x - 4,
                y: y - 4,
                w: 8,
                h: 8,
                filled: true,
                color: (255, 250, 218, 255),
                layer: IMPACT_LAYER + 1,
                screen_space: false,
            },
        ])
    }
}

#[cfg(test)]
mod tests;
