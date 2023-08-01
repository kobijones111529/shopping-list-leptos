pub mod app;
pub mod pages;
use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: usize,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct ItemStore {
    next_id: usize,
    items: Vec<Item>,
}

impl ItemStore {
    pub fn new(start_id: usize) -> Self {
        Self {
            next_id: start_id,
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, text: String) {
        self.items.push(Item {
            id: self.next_id,
            text,
        });
        self.next_id += 1;
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }
}

cfg_if! {
if #[cfg(feature = "hydrate")] {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;

      console_error_panic_hook::set_once();

      leptos::mount_to_body(move |cx| {
          view! { cx, <App/> }
      });
    }
}
}
