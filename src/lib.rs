use image::Rgb;

/// Square root of 2
const SQRT2: f32 = 1.4142135623730951;

// Colors
const RGB_BLACK: Rgb<u8> = image::Rgb([0u8, 0u8, 0u8]);
const RGB_WHITE: Rgb<u8> = image::Rgb([255u8, 255u8, 255u8]);
const _RGB_GRAY: Rgb<u8> = image::Rgb([150u8, 150u8, 150u8]);

// Debug colors
#[allow(dead_code)]
const COLORS: &'static [[u8; 3]] = &[
  [0, 255, 0],
  [0, 0, 255],
  [0, 255, 255],
  [255, 0, 255],
  [255, 255, 0],
  [255, 150, 0],
];

pub struct Config {
  /// Columns to wrap text at
  pub max_columns: u32,
  /// Size of tile
  pub tile_size: u32,
  /// Padding size for tile
  pub padding: u32,
  /// More padding for sides
  pub padding_side: u32,
  /// Stroke width
  pub stroke: u32,
}

pub fn make_image(text: &Vec<Char>, config: &Config) {
  // Size of tile map (Scaled down image)
  let cols = std::cmp::min(text.len() as u32, config.max_columns);
  let rows = (text.len() as f32 / cols as f32).ceil() as u32;

  //  Size of image
  let img_x = cols * (config.tile_size);
  let img_y = rows * (config.tile_size);

  println!("Size: {img_x}x{img_y}");

  // Create a new ImgBuf with size
  let mut img = image::ImageBuffer::new(img_x, img_y);

  // Iterate over the coordinates and pixels of the image
  for (x, y, pixel) in img.enumerate_pixels_mut() {
    // If character exists
    if let Some(ch) = text.get(((x / config.tile_size) + (y / config.tile_size) * cols) as usize) {
      // Stroke - black
      if check_stroke(*ch, x % config.tile_size, y % config.tile_size, config) {
        // let color = *COLORS.get(ch.1 as usize).unwrap_or(&[255, 0, 0]);
        let color = RGB_BLACK;
        *pixel = Rgb::from(color);
        continue;
      }
    }

    // Background - white / gray at edge
    *pixel = if std::cmp::min(
      config.tile_size - (x % config.tile_size),
      config.tile_size / 2 - (y % (config.tile_size / 2)),
    ) == 1
    {
      RGB_WHITE
      // RGB_GRAY
    } else {
      RGB_WHITE
    }
  }

  img.save("./image.png").expect("Could not save image");
}

/// Check if stroke is at x and y
pub fn check_stroke(ch: Char, x: u32, y: u32, config: &Config) -> bool {
  // Decide character part, adjust y if is bottom
  let (ch_part, y) = if y <= config.tile_size / 2 {
    (ch.0, y)
  } else {
    (ch.1, y - config.tile_size / 2)
  };

  let top = config.padding;
  let bottom = config.tile_size / 2 - 1 - config.padding;
  let left = config.padding + config.padding_side;
  let right = config.tile_size - 1 - config.padding - config.padding_side;

  // Padding - skip
  if x < left || y < top || x > right || y > bottom {
    return false;
  }

  // Top hr
  if [1, 3, 8].contains(&(ch_part as i32)) {
    if diff(y, top) < config.stroke {
      return true;
    }
  }

  // Bottom hr
  if [0, 2, 9].contains(&(ch_part as i32)) {
    if diff(y, bottom) < config.stroke {
      return true;
    }
  }

  // Left and right vr
  if [4, 5, 6, 7].contains(&(ch_part as i32)) {
    if std::cmp::min(diff(x, right), diff(x, left)) < (config.stroke as f32 * SQRT2) as u32 {
      return true;
    }
  }

  // Left up diagonal
  if [0, 6].contains(&(ch_part as i32)) {
    if x > (config.tile_size / 2 - 1)
      && diff(
        ((x + config.padding_side) as i32 - (config.tile_size / 2) as i32) as u32,
        y,
      ) < config.stroke
    {
      return true;
    }
  }

  // Left down diagonal
  if [1, 7].contains(&(ch_part as i32)) {
    if x > (config.tile_size / 2 - 1)
      && diff(
        (config.tile_size as i32 - ((x + config.padding_side + 1) as i32)) as u32,
        y,
      ) < config.stroke
    {
      return true;
    }
  }

  // Right up diagonal
  if [2, 6].contains(&(ch_part as i32)) {
    if x < (config.tile_size + 1)
      && diff(
        ((config.tile_size / 2 + config.padding_side) as i32 - 1 - (x as i32)) as u32,
        y,
      ) < config.stroke
    {
      return true;
    }
  }

  // Right down diagonal
  if [3, 7].contains(&(ch_part as i32)) {
    if x < (config.tile_size / 2 + 1)
      && diff(((x as i32) - config.padding_side as i32) as u32, y) < config.stroke
    {
      return true;
    }
  }

  // Full right down diagonal
  if [4].contains(&(ch_part as i32)) {
    if (diff(x, y * 2) as f32) < config.stroke as f32 * SQRT2 {
      return true;
    }
  }

  // Full right up diagonal
  if [5].contains(&(ch_part as i32)) {
    if (diff(x + 2, ((config.tile_size / 2) as i32 - y as i32) as u32 * 2) as f32)
      < config.stroke as f32 * SQRT2
    {
      return true;
    }
  }

  false
}

fn diff(a: u32, b: u32) -> u32 {
  ((a as i32) - (b as i32)).abs() as u32
}

/// Encrypted character
type Char = (u8, u8);

/// Map of characters
const MAP: &'static [&'static str] = &[
  "pbtdkg",
  "fvszcj",
  "mnrlwy",
  "ťďčǰňh",
  "øiueoa",
  "ãíúéóá",
  ".,ǔěǒǎ",
  "ẽĩũȇȏȃ",
  //TODO Implement these!
  // "012345",
  // "6789!?",
];

/// Encrypt text to cypher
pub fn encrypt_text(text: &str) -> Vec<Char> {
  let mut vec: Vec<Char> = vec![];

  'Ch: for ch in text.chars() {
    if ch == ' ' {
      //TODO Add space!
      continue;
    }
    for (y, line) in MAP.iter().enumerate() {
      for (x, map_ch) in line.chars().enumerate() {
        if ch == map_ch {
          vec.push((y as u8, x as u8));
          continue 'Ch;
        }
      }
    }
    panic!("Unknown character: '{}'", ch);
  }

  vec
}
