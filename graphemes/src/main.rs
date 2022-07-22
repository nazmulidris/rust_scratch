/*
 *   Copyright (c) 2022 Nazmul
 *   All rights reserved.
 *
 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
 */

//! A grapheme cluster is a user-perceived character. Rust uses UTF-8 to represent text in String.
//! So each character takes up 8 bits or one byte. Grapheme clusters can take up many more bytes,
//! eg 4 bytes or 2 or 3, etc.
//!
//! Docs:
//! - format! arguments: https://doc.rust-lang.org/std/fmt/
//! - Grapheme clusters: https://medium.com/flutter-community/working-with-unicode-and-grapheme-clusters-in-dart-b054faab5705
//! - UTF-8 String: https://doc.rust-lang.org/book/ch08-02-strings.html

use seshat::unicode::{Segmentation, Ucd};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

fn main() {
  print_graphemes();
  print_cluster_breaks_using_seshat_and_unicode_width();
  print_graphemes_using_unicode_segmentation_and_unicode_width();
  print_grapheme_indices_using_unicode_segmentation_and_unicode_width();
}

pub fn print_graphemes() {
  println!("🦀 is {}!", '🦀'.na());
  println!("📦 is {}!", '📦'.na());
  println!("🦜 is {}!", '🦜'.na());
  println!("Multiple code points: 🙏🏽");
  println!("Multiple code points: 💇🏽‍♂️");
}

pub fn print_cluster_breaks_using_seshat_and_unicode_width() {
  let s = "Hi 📦 🙏🏽 👨🏾‍🤝‍👨🏿.";
  println!(
    "\n-- print_cluster_breaks_using_seshat_and_unicode_width -- unicode_width: {}\n",
    UnicodeWidthStr::width(s)
  );
  let bg = s.break_graphemes();
  for (g_c_idx, g_c) in bg.enumerate() {
    let g_c_display_width = UnicodeWidthStr::width(g_c);
    let _g_c_idx = format!("{:02}", g_c_idx);
    let _g_c_display_width = format!("{:02}", g_c_display_width);
    let _g_c = format!("{:◻<8}", g_c);
    println!(
      r#"g_c_idx = {} › g_c_display_width = {} › g_c = '{}' /* max 8 ch */"#,
      _g_c_idx, _g_c_display_width, _g_c
    );
  }
}

pub fn print_graphemes_using_unicode_segmentation_and_unicode_width() {
  println!("\n-- print_graphemes_using_unicode_segmentation --\n");
  let s = "Hi 📦 🙏🏽 👨🏾‍🤝‍👨🏿.";
  let g = s.graphemes(true);
  for (g_c_idx, g_c) in g.enumerate() {
    let _g_c_idx = format!("{:02}", g_c_idx);
    let _g_c = format!("{:◻<8}", g_c);
    let _g_c_width = format!("{:02}", UnicodeWidthStr::width(g_c));
    println!(
      r#"g_c_idx = {} › unicode_width = '{}' › g_c = {}"#,
      _g_c_idx, _g_c_width, _g_c,
    );
  }
}

pub fn print_grapheme_indices_using_unicode_segmentation_and_unicode_width() {
  println!("\n-- print_grapheme_indices_using_unicode_segmentation_and_unicode_width --\n");
  let s = "Hi 📦 🙏🏽 👨🏾‍🤝‍👨🏿.";
  let gi = s.grapheme_indices(true);
  let mut byte_len = 0;
  for (g_c_idx, (byte_offset, g_c)) in gi.enumerate() {
    let _g_c_idx = format!("{:02}", g_c_idx);
    let _byte_offset = format!("{:02}", byte_offset);
    let _g_c = format!("{:◻<8}", g_c);
    let _g_c_len = format!("{:02}", g_c.len());
    let _u_w = format!("{:02}", UnicodeWidthStr::width(g_c));
    println!(
      r#"g_c_idx = {} › (byte_offset = {} , len/byte-size = {} , u_w = {} , g_c = `{}`)"#,
      _g_c_idx, _byte_offset, _g_c_len, _u_w, _g_c
    );
    byte_len = byte_offset;
  }

  println!();
  println! {"❯ unicode_width:     {} ← display size / width", UnicodeWidthStr::width(s)};
  println! {"❯ byte_len:          {}", byte_len};
  println! {"❯ s.chars().count(): {} ← UTF-8 chars (not grapheme clusters)", s.chars().count()};
  println! {"❯ s.len():           {} ← byte size", s.len()};
}
