use cypher_image::{encrypt_text, make_image, Config};

fn main() {
  let text = std::env::args().nth(1).expect("No text given");
  if text.len() < 1 {
    panic!("Text must contain characters");
  }

  let encrypted = encrypt_text(&text);

  let config = Config::new(50, 2, 6, 2, false);

  make_image(&encrypted, &config)
    .save("./image.png")
    .expect("Could not save image");
}

// let text = "có, ďȏz twálv běj húks a ďǒnd íf ǎ páč ø yãň, gǔ mȃť";
// let text = "pbtdkg fvszcj mnrlwy ťďčǰňh øiueoa ãíúéóá .,ǔěǒǎ ẽĩũȇȏȃ";
