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
