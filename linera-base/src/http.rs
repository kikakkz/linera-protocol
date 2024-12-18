// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Types used when performing HTTP requests.

use custom_debug_derive::Debug;
use linera_witty::{WitLoad, WitStore, WitType};
use serde::{Deserialize, Serialize};

use crate::hex_debug;

/// The method used in an HTTP request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, WitLoad, WitStore, WitType)]
pub enum Method {
    /// A GET request.
    Get,

    /// A POST request.
    Post,

    /// A PUT request.
    Put,

    /// A DELETE request.
    Delete,

    /// A HEAD request.
    Head,

    /// A OPTIONS request.
    Options,

    /// A CONNECT request.
    Connect,

    /// A PATCH request.
    Patch,

    /// A TRACE request.
    Trace,
}

#[cfg(with_reqwest)]
impl From<Method> for reqwest::Method {
    fn from(method: Method) -> Self {
        match method {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Delete => reqwest::Method::DELETE,
            Method::Head => reqwest::Method::HEAD,
            Method::Options => reqwest::Method::OPTIONS,
            Method::Connect => reqwest::Method::CONNECT,
            Method::Patch => reqwest::Method::PATCH,
            Method::Trace => reqwest::Method::TRACE,
        }
    }
}

/// A response for an HTTP request.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, WitLoad, WitStore, WitType)]
pub struct Response {
    /// The status code of the HTTP response.
    pub status: u16,

    /// The headers included in the response.
    pub headers: Vec<(String, Vec<u8>)>,

    /// The body of the response.
    #[debug(with = "hex_debug")]
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

#[cfg(with_reqwest)]
impl Response {
    /// Creates a [`Response`] from a [`reqwest::Response`], waiting for it to be fully
    /// received.
    pub async fn from_reqwest(response: reqwest::Response) -> reqwest::Result<Self> {
        let headers = response
            .headers()
            .into_iter()
            .map(|(name, value)| (name.to_string(), value.as_bytes().to_owned()))
            .collect();

        Ok(Response {
            status: response.status().as_u16(),
            headers,
            body: response.bytes().await?.to_vec(),
        })
    }
}

/// A header for a HTTP request or response.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, WitLoad, WitStore, WitType)]
pub struct Header {
    /// The header name.
    pub name: String,

    /// The value of the header.
    #[debug(with = "hex_debug")]
    #[serde(with = "serde_bytes")]
    pub value: Vec<u8>,
}

impl Header {
    /// Creates a new [`Header`] with the provided `name` and `value`.
    pub fn new(name: impl Into<String>, value: impl Into<Vec<u8>>) -> Self {
        Header {
            name: name.into(),
            value: value.into(),
        }
    }
}