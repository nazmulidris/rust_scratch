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

use crate::{BoxPosition, BoxSize};

/// Direction of the layout of the box.
#[derive(Copy, Clone, Debug)]
pub enum BoxDirection {
  Horizontal,
  Vertical,
}

impl Default for BoxDirection {
  fn default() -> BoxDirection {
    BoxDirection::Horizontal
  }
}

/// A box is a rectangle with a position and size. The direction of the box determines how
/// it's contained elements are positioned.
#[derive(Copy, Clone, Debug, Default)]
pub struct BoxLayout {
  pub position: BoxPosition,
  pub size: BoxSize,
  pub direction: BoxDirection,
}

/// Represents a rectangular area of the terminal screen, and not necessarily the full
/// terminal screen.
#[derive(Clone, Debug, Default)]
pub struct BoxCanvas {
  pub origin: BoxPosition,
  pub size: BoxSize,
  pub layout_stack: Vec<BoxLayout>,
}