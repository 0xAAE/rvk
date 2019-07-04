//! Contains all of the API methods in the respective submodules.
//!
//! ## Note about naming
//! Rust prefers `snake_case` in the function names instead of `camelCase` used by the VK API,
//! which means all of the API method's corresponding functions are named using `snake_case`.
//!
//! **Example:** To call the `appWidgets.getAppImageUploadServer` API method, use the `rvk::methods::app_widgets::get_app_image_upload_server` function.
//!
//! ## Note: `execute`
//! The `execute` method has no category, so its path is `rvk::methods::execute`.
//!
//! ## Note: `photos.move`
//! Since `move` is a Rust keyword, the function for calling `photos.move` API method is `rvk::methods::photos::move_` (**with the underscore!**)

macro_rules! api_category {
    ($category:expr; methods { $($name:ident),* }) => {
        use heck::MixedCase;
        use std::collections::HashMap;
        use lazy_static::lazy_static;
        const CATEGORY: &str = $category;

        lazy_static! {
            static ref METHOD_NAMES: HashMap<&'static str, String> = {
                let mut m = HashMap::new();

                $(
                    m.insert(stringify!($name), CATEGORY.to_owned() + "." + &stringify!($name).to_mixed_case());
                )*

                m
            };
        }

        $(
            api_method!(
                $name,
                METHOD_NAMES
                    .get(stringify!($name))
                    .expect(&format!("No method with name {} found in METHOD_NAMES.
This is a bug.
Please report it at <https://github.com/u32i64/rvk>", stringify!($name)))
            );
        )*
    };
}

macro_rules! api_method {
    ($func_name:ident, $method_name:expr) => {
        /// [generated] Calls the corresponding VK API method
        ///
        /// Generated by the `api_method!` macro.
        pub fn $func_name(
            api: &crate::api::APIClient,
            params: crate::Params,
        ) -> crate::error::Result<serde_json::Value> {
            api.call_method($method_name, params)
        }
    };
}

api_method!(execute, "execute");

pub mod account;
pub mod ads;
pub mod app_widgets;
pub mod apps;
pub mod auth;
pub mod board;
pub mod database;
pub mod docs;
pub mod fave;
pub mod friends;
pub mod gifts;
pub mod groups;
pub mod leads;
pub mod likes;
pub mod market;
pub mod messages;
pub mod newsfeed;
pub mod notes;
pub mod notifications;
pub mod orders;
pub mod pages;
pub mod photos;
pub mod places;
pub mod polls;
pub mod search;
pub mod secure;
pub mod stats;
pub mod status;
pub mod storage;
pub mod stories;
pub mod streaming;
pub mod users;
pub mod utils;
pub mod video;
pub mod wall;
pub mod widgets;