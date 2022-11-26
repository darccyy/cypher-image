use cypher::{encrypt_text, make_image, Config};

fn main() {
  // let text = "có, ďȏz twálv běj húks a ďǒnd íf ǎ páč ø yãň, gǔ mȃť";
  let text = "pbtdkg fvszcj mnrlwy ťďčǰňh øiueoa ãíúéóá .,ǔěǒǎ ẽĩũȇȏȃ";
  let encrypted = encrypt_text(text);

  let config = Config {
    max_columns: 6,
    tile_size: 50,
    padding: 1,
    padding_side: 1,
    stroke: 2,
  };

  make_image(&encrypted, &config);
}
