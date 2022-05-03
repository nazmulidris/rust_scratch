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

use tui_layout_crossterm::layout::*;

// TODO: write test for box_canvas.rs
#[test]
fn test_simple_2_col_layout() {
  let mut canvas = Canvas::default();
  {
    // start
    canvas
      .start((0, 0), (500, 500))
      .unwrap();

    // start layout (main container)
    {
      canvas
        .start_layout(Direction::Vert, (100, 100))
        .unwrap();

      {
        // start layout (left column)
        canvas
          .start_layout(Direction::Vert, (50, 100))
          .unwrap();
        {
          canvas
            .print("col 1 - Hello")
            .unwrap();
          canvas
            .print("col 1 - World")
            .unwrap();
        }
        canvas.end_layout().unwrap();

        // start layout (right column)
        canvas
          .start_layout(Direction::Vert, (50, 100))
          .unwrap();
        {
          canvas
            .print("col 2 - Hello")
            .unwrap();
          canvas
            .print("col 2 - World")
            .unwrap();
        }
        canvas.end_layout().unwrap();
      }

      canvas.end_layout().unwrap();
    }

    // end
    canvas.end().unwrap();
  }
}