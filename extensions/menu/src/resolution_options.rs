const MAX_RESOLUTIONS: usize = 6;

pub fn normalize_resolutions(mut modes: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    modes.retain(|&(width, height)| width > 0 && height > 0);
    modes.sort_unstable();
    modes.dedup();
    if modes.is_empty() {
        return vec![(800, 600)];
    }
    if modes.len() <= MAX_RESOLUTIONS {
        return modes;
    }
    let last = modes.len() - 1;
    let stride = last as f32 / (MAX_RESOLUTIONS - 1) as f32;
    (0..MAX_RESOLUTIONS)
        .map(|index| modes[((index as f32 * stride).round() as usize).min(last)])
        .collect()
}

#[cfg(test)]
mod tests;
