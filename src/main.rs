use image::Rgb;

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

/// Columns to wrap text at
const MAX_COLS: usize = 6;
/// Size of tile
const TILE_SIZE: u32 = 200;
/// Half size of tile
const TILE_HALF: u32 = TILE_SIZE / 2;
/// Padding size for tile
const PADDING_Y: u32 = 5;
/// More padding for sides
const PADDING_X: u32 = PADDING_Y + 1;
/// Stroke width
const STROKE: u32 = 10;

/// Square root of 2
const SQRT2: f32 = 1.4142135623730951;

fn main() {
  // Text to draw
  // let text = encrypt_text("có, ďȏz twálv běj húks a ďǒnd íf ǎ páč ø yãň, gǔ mȃť");
  let text = encrypt_text("pbtdkg fvszcj mnrlwy ťďčǰňh øiueoa ãíúéóá .,ǔěǒǎ ẽĩũȇȏȃ");

  // Size of tile map (Scaled down image)
  let cols = text.len().min(MAX_COLS) as u32;
  let rows = (text.len() as f32 / cols as f32).ceil() as u32;

  //  Size of image
  let img_x = cols * (TILE_SIZE);
  let img_y = rows * (TILE_SIZE);

  println!("Size: {img_x}x{img_y}");

  // Create a new ImgBuf with size
  let mut img = image::ImageBuffer::new(img_x, img_y);

  // Iterate over the coordinates and pixels of the image
  for (x, y, pixel) in img.enumerate_pixels_mut() {
    // If character exists
    if let Some(ch) = text.get(((x / TILE_SIZE) + (y / TILE_SIZE) * cols) as usize) {
      // Stroke - black
      if check_stroke(*ch, x % TILE_SIZE, y % TILE_SIZE) {
        // let color = *COLORS.get(ch.1 as usize).unwrap_or(&[255, 0, 0]);
        let color = RGB_BLACK;
        *pixel = Rgb::from(color);
        continue;
      }
    }

    // Background - white / gray at edge
    *pixel = if std::cmp::min(TILE_SIZE - (x % TILE_SIZE), TILE_HALF - (y % TILE_HALF)) == 1 {
      RGB_WHITE
      // RGB_GRAY
    } else {
      RGB_WHITE
    }
  }

  img.save("./image.png").expect("Could not save image");
}

/// Check if stroke is at x and y
pub fn check_stroke(ch: Char, x: u32, y: u32) -> bool {
  // Decide character part, adjust y if is bottom
  let (ch_part, y) = if y <= TILE_HALF {
    (ch.0, y)
  } else {
    (ch.1, y - TILE_HALF)
  };

  let top = PADDING_Y;
  let bottom = TILE_HALF - 1 - PADDING_Y;
  let left = PADDING_Y + PADDING_X;
  let right = TILE_SIZE - 1 - PADDING_Y - PADDING_X;

  // Padding - skip
  if x < left || y < top || x > right || y > bottom {
    return false;
  }

  // Top hr
  if [1, 3, 8].contains(&(ch_part as i32)) {
    if diff(y, top) < STROKE {
      return true;
    }
  }

  // Bottom hr
  if [0, 2, 9].contains(&(ch_part as i32)) {
    if diff(y, bottom) < STROKE {
      return true;
    }
  }

  // Left and right vr
  if [4, 5, 6, 7].contains(&(ch_part as i32)) {
    if std::cmp::min(diff(x, right), diff(x, left)) < (STROKE as f32 * SQRT2) as u32 {
      return true;
    }
  }

  // Left up diagonal
  if [0, 6].contains(&(ch_part as i32)) {
    if x > (TILE_HALF - 1) && diff(((x + PADDING_X) as i32 - TILE_HALF as i32) as u32, y) < STROKE {
      return true;
    }
  }

  // Left down diagonal
  if [1, 7].contains(&(ch_part as i32)) {
    if x > (TILE_HALF - 1) && diff((TILE_SIZE as i32 - ((x + PADDING_X + 1) as i32)) as u32, y) < STROKE
    {
      return true;
    }
  }

  // Right up diagonal
  if [2, 6].contains(&(ch_part as i32)) {
    if x < (TILE_HALF + 1)
      && diff(((TILE_HALF + PADDING_X) as i32 - 1 - (x as i32)) as u32, y) < STROKE
    {
      return true;
    }
  }

  // Right down diagonal
  if [3, 7].contains(&(ch_part as i32)) {
    if x < (TILE_HALF + 1) && diff(((x as i32) - PADDING_X as i32) as u32, y) < STROKE {
      return true;
    }
  }

  // Full right down diagonal
  if [4].contains(&(ch_part as i32)) {
    if (diff(x, y * 2) as f32) < STROKE as f32 * SQRT2 {
      return true;
    }
  }

  // Full right up diagonal
  if [5].contains(&(ch_part as i32)) {
    if (diff(x + 2, (TILE_HALF as i32 - y as i32) as u32 * 2) as f32) < STROKE as f32 * SQRT2 {
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
