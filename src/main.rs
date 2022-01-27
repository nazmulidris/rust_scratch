/*
 * Copyright (c) 2022 Nazmul Idris. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

mod lib;
mod guessing_game;
mod hello_world;
mod strings;
mod variables;
mod control_flow;
mod ownership;
mod structs;
mod enum_variants;

fn main() {
  hello_world::run();
  strings::run();
  guessing_game::run();
  variables::run();
  control_flow::run();
  ownership::run();
  structs::run();
  enum_variants::run();
}
