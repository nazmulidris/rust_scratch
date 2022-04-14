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

use crate::json_rpc::{
  fake_contact_data_api::make_request as fake_contact_data_api, FakeContactData,
};
use crate::{
  address_book::{Action, State},
  tui::{DELAY_ENABLED, MAX_DELAY, MIN_DELAY},
};
use r3bl_rs_utils::{print_header, redux::StoreStateMachine};
use rand::Rng;
use std::sync::Arc;
use tokio::{spawn, sync::RwLock};

pub fn logger_mw(
  action: Action,
  _: Arc<RwLock<StoreStateMachine<State, Action>>>,
) -> Option<Action> {
  if DELAY_ENABLED {
    // Artificial delay before calling the function.
    let delay_ms = rand::thread_rng().gen_range(MIN_DELAY..MAX_DELAY) as u64;
    std::thread::sleep(tokio::time::Duration::from_millis(
      delay_ms,
    ));
  }
  spawn(async move {
    // Log the action.
    println!("");
    print_header("middleware");
    println!("action: {:?}", action);
  });
  None
}

pub fn add_async_cmd_mw(
  action: Action,
  store_ref: Arc<RwLock<StoreStateMachine<State, Action>>>,
) -> Option<Action> {
  if let Action::AsyncAddContact = action {
    tokio::spawn(async { add_async_cmd_impl(store_ref).await });
  }
  None
}

/// Spawns a task. Fire and forget.
async fn add_async_cmd_impl(store_ref: Arc<RwLock<StoreStateMachine<State, Action>>>) {
  spawn(async move {
    let fake_data = fake_contact_data_api()
      .await
      .unwrap_or_else(|_| FakeContactData {
        name: "Foo Bar".to_string(),
        phone_h: "123-456-7890".to_string(),
        email_u: "foo".to_string(),
        email_d: "bar.com".to_string(),
        ..FakeContactData::default()
      });

    let action = Action::AddContact(
      format!("{}", fake_data.name),
      format!(
        "{}@{}",
        fake_data.email_u, fake_data.email_d
      ),
      format!("{}", fake_data.phone_h),
    );

    let mut my_store = store_ref.write().await;

    my_store
      .dispatch_action(&action, store_ref.clone())
      .await;
  }); /* Don't await this. Fire and forget. */
}
