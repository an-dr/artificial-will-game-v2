use crate::{Rect, SheetError};

struct Rgba {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

fn png_error(error: impl std::fmt::Display) -> SheetError {
    SheetError::Png(error.to_string())
}

fn decode(png_bytes: &[u8]) -> Result<Rgba, SheetError> {
    let mut decoder = png::Decoder::new(std::io::Cursor::new(png_bytes));
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().map_err(png_error)?;
    let size = reader
        .output_buffer_size()
        .ok_or_else(|| png_error("image too large"))?;
    let mut buffer = vec![0; size];
    let info = reader.next_frame(&mut buffer).map_err(png_error)?;
    let samples = &buffer[..info.buffer_size()];
    let pixels = match info.color_type {
        png::ColorType::Rgba => samples.to_vec(),
        png::ColorType::Rgb => samples
            .chunks_exact(3)
            .flat_map(|p| [p[0], p[1], p[2], 255])
            .collect(),
        png::ColorType::GrayscaleAlpha => samples
            .chunks_exact(2)
            .flat_map(|p| [p[0], p[0], p[0], p[1]])
            .collect(),
        png::ColorType::Grayscale => samples.iter().flat_map(|&g| [g, g, g, 255]).collect(),
        png::ColorType::Indexed => return Err(png_error("indexed colour was not expanded")),
    };
    Ok(Rgba {
        width: info.width,
        height: info.height,
        pixels,
    })
}

fn encode(image: &Rgba) -> Result<Vec<u8>, SheetError> {
    let mut out = Vec::new();
    let mut encoder = png::Encoder::new(&mut out, image.width, image.height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(png_error)?;
    writer.write_image_data(&image.pixels).map_err(png_error)?;
    writer.finish().map_err(png_error)?;
    Ok(out)
}

pub(crate) fn cut(
    png_bytes: &[u8],
    name: &str,
    rects: &[Rect],
    frame_size: [u32; 2],
    shift: (i32, i32),
) -> Result<Vec<u8>, SheetError> {
    let sheet = decode(png_bytes)?;
    let [frame_w, frame_h] = frame_size;
    let mut strip = Rgba {
        width: frame_w * rects.len() as u32,
        height: frame_h,
        pixels: vec![0; (frame_w * rects.len() as u32 * frame_h * 4) as usize],
    };
    for (index, &rect) in rects.iter().enumerate() {
        let [x, y, w, h] = rect;
        if x + w > sheet.width || y + h > sheet.height {
            return Err(SheetError::RectOutOfBounds {
                animation: name.to_string(),
                rect,
            });
        }
        for sy in 0..h.min(frame_h) {
            for sx in 0..w.min(frame_w) {
                let dx = sx as i32 + shift.0;
                let dy = sy as i32 + shift.1;
                if dx < 0 || dy < 0 || dx >= frame_w as i32 || dy >= frame_h as i32 {
                    continue;
                }
                let src = (((y + sy) * sheet.width + x + sx) * 4) as usize;
                let dst =
                    ((dy as u32 * strip.width + index as u32 * frame_w + dx as u32) * 4) as usize;
                strip.pixels[dst..dst + 4].copy_from_slice(&sheet.pixels[src..src + 4]);
            }
        }
    }
    encode(&strip)
}

/// Re-lays a grid tileset with every tile surrounded by `pad` copies of its
/// own edge pixels, `pad` px margin and `2 * pad` px spacing. A renderer
/// scaling tiles by a non-integer factor can sample a texel outside a
/// tile's source rect; with the extrusion that texel is the tile's own
/// edge instead of its atlas neighbour, so no seams show between tiles.
pub(crate) fn extrude_tiles(png_bytes: &[u8], tile: u32, pad: u32) -> Result<Vec<u8>, SheetError> {
    let source = decode(png_bytes)?;
    let (cols, rows) = (source.width / tile, source.height / tile);
    let cell = tile + 2 * pad;
    let (width, height) = (cols * cell, rows * cell);
    let mut pixels = vec![0u8; (width * height * 4) as usize];
    for row in 0..rows {
        for col in 0..cols {
            for dy in 0..cell {
                for dx in 0..cell {
                    let sx = col * tile + dx.saturating_sub(pad).min(tile - 1);
                    let sy = row * tile + dy.saturating_sub(pad).min(tile - 1);
                    let from = ((sy * source.width + sx) * 4) as usize;
                    let to = (((row * cell + dy) * width + col * cell + dx) * 4) as usize;
                    pixels[to..to + 4].copy_from_slice(&source.pixels[from..from + 4]);
                }
            }
        }
    }
    encode(&Rgba {
        width,
        height,
        pixels,
    })
}
