//! Works with the API

use crate::{
    error::{APIError, Result},
    API_VERSION,
};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use serde_json::{from_value, Map, Value};
use std::collections::HashMap;

#[cfg(feature = "trace_response")]
mod trace {
    use chrono::Local;
    use std::fs::write;
    use std::path::Path;

    pub fn try_trace_failed_response(response: &str, error_message: &str) {
        try_trace_response("/failed", response, error_message);
    }

    pub fn try_trace_succeeded_response(response: &str) {
        try_trace_response("/succeeded", response, "");
    }

    fn try_trace_response(subdir: &str, response: &str, error_message: &str) {
        let mut dir = std::env::var("RVK_TRACE_DIR").unwrap_or_else(|_| {
            let mut local = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            local.push_str("/.cache/rvk");
            local
        });
        dir.push_str(subdir);
        if std::fs::create_dir_all(Path::new(&dir)).is_err() {
            log::error!("failed to create directory to trace response {}", dir);
            return;
        }

        dir.push_str(format!("{}", Local::now().format("/%Y-%m-%d_%H-%M-%S")).as_str());
        let json = dir.clone() + ".json";
        if write(&Path::new(&json), response.as_bytes()).is_err() {
            log::error!("failed to write file {}", json);
        } else {
            log::debug!("write response into file {}", json);
        }
        if !error_message.is_empty() {
            let msg = dir + "_msg.txt";
            if write(&Path::new(&msg), error_message.as_bytes()).is_err() {
                log::error!("failed to write file {}", msg);
            } else {
                log::debug!("write problem message into file {}", msg);
            }
        }
    }
}

/// A HashMap which contains method parameters
pub type Params = HashMap<String, String>;

/// An API client used to call API methods.
#[derive(Debug)]
pub struct APIClient {
    client: Client,
    token: String,
}

impl APIClient {
    /// Creates a new `APIClient`, given an access token.
    ///
    /// # Panics
    /// This method panics if native TLS backend cannot be created or initialized by the `reqwest` crate.
    ///
    /// See [reqwest docs](https://docs.rs/reqwest/0.10/reqwest/struct.Client.html#panic) for more information.
    pub fn new(token: impl Into<String>) -> APIClient {
        APIClient {
            client: Client::new(),
            token: token.into(),
        }
    }

    /// Calls an API method, given its name and parameters.
    pub async fn call_method<T: DeserializeOwned>(
        &self,
        method_name: &str,
        mut params: Params,
    ) -> Result<T> {
        params.insert("v".into(), API_VERSION.into());
        params.insert("access_token".into(), self.token.clone());

        let response_result: Result<Response> = self
            .client
            .get(&("https://api.vk.com/method/".to_owned() + method_name))
            .query(&params)
            .send()
            .await
            .map_err(|e| e.into());
        let response = response_result?;

        let value_result: Result<Value> = response.json().await.map_err(|e| e.into());
        let mut value = value_result?;

        #[cfg(feature = "trace_response")]
        let response_copy = value.to_string();

        let api_response_result: Result<&mut Map<String, Value>> = value
            .as_object_mut()
            .ok_or_else(|| "API response is not an object!".into());
        let api_response = api_response_result?;

        match api_response.remove("response") {
            Some(ok) => {
                let res = from_value::<T>(ok);
                #[cfg(feature = "trace_response")]
                if let Err(e) = res.as_ref() {
                    trace::try_trace_failed_response(
                        response_copy.as_str(),
                        format!("{}", e).as_str(),
                    );
                } else {
                    if let Ok(var) = std::env::var("RVK_TRACE_ALL") {
                        if var == "1" {
                            trace::try_trace_succeeded_response(response_copy.as_str());
                        }
                    }
                }
                Ok(res?)
            }
            None => match api_response.remove("error") {
                Some(err) => Err(from_value::<APIError>(err)?.into()),
                None => Err("The API responded with neither a response nor an error!".into()),
            },
        }
    }
}
