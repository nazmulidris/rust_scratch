/*
 *   Copyright (c) 2022 Nazmul Idris
 *   All rights reserved.

 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at

 *   http://www.apache.org/licenses/LICENSE-2.0

 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
*/

use crossterm::terminal;

const DEBUG: bool = false;

/// This will automatically disable [raw
/// mode](https://docs.rs/crossterm/0.23.2/crossterm/terminal/index.html#raw-mode) when
/// the enclosed block ends.
///
/// Example 1:
/// ```ignore
/// raw_mode!(repl().await?);
/// ```
///
/// Example 2:
/// ```ignore
/// return raw_mode!({
///   repl().await?;
///   Ok(())
/// });
/// ```
///
/// Example 3:
/// ```ignore
/// raw_mode!({
///   println!("crossterm: Entering raw mode...");
///   repl().await?;
///   println!("crossterm: Exiting raw mode...");
///   return Ok(());
/// });
/// ```
#[macro_export]
macro_rules! raw_mode {
  ($code_block: stmt) => {{
    use crate::crossterm_mod::raw_mode::RawMode;
    let _raw_mode = RawMode::start();
    $code_block
  }};
}

#[macro_export]
macro_rules! println_raw {
  ($arg:tt) => {
    println!("{}\r", $arg)
  };
}

/// To use this, you need to make sure to create an instance using `default()` (which
/// enables raw mode) and then when this instance is dropped (when code_block falls out of
/// scope) raw mode will be disabled.
pub struct RawMode;

impl RawMode {
  pub fn start() -> Self {
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    if DEBUG {
      println_raw!("Raw mode enabled.");
    }
    RawMode
  }
}

impl Drop for RawMode {
  fn drop(&mut self) {
    terminal::disable_raw_mode().expect("Failed to disable raw mode");
    if DEBUG {
      println_raw!("Raw mode disabled.");
    }
  }
}