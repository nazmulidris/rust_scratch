use ansi_term::ANSIGenericString;
use ansi_term::Color::{Blue, Green, Red, White};
use ansi_term::Colour::Purple;

/// ANSI colorized text: /// https://github.com/ogham/rust-ansi-term
/// Equivalent for template string literal. One way to do this using format!
/// 1. https://doc.rust-lang.org/std/fmt/
/// 2. https://internals.rust-lang.org/t/string-interpolation-template-literals-like-js/9082/3
pub fn print_header(msg: &str) {
  let hamburger = "☰";
  let msg = format!("{0} {1} {0}", hamburger, msg);
  println!("{}", Purple.paint(&msg));
}

/// Equivalent for template string literal. Another way to do this using += and insert_str.
pub fn print_header2(arg: &str) {
  let hamburger = "☰";
  let mut msg = String::from(hamburger);
  msg += " "; // msg.insert_str(msg.len(), " ");
  msg += arg; // msg.insert_str(msg.len(), arg);
  msg.insert_str(msg.len(), " ");
  msg.insert_str(msg.len(), hamburger);
  println!("{}", Purple.paint(&msg))
}

pub fn style_primary(text: &str) -> ANSIGenericString<str> {
  return Green.bold().paint(text);
}

pub fn style_prompt(text: &str) -> ANSIGenericString<str> {
  return Blue.bold().paint(text);
}

pub fn style_error(text: &str) -> ANSIGenericString<str> {
  return Red.bold().paint(text);
}

pub fn style_dimmed(text: &str) -> ANSIGenericString<str> {
  return White.underline().paint(text);
}