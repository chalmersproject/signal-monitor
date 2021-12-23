#![allow(unused_imports)]

pub mod app;
pub mod config;
pub mod handlers;

use std::fmt::{Debug, Display};
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use derives::Display;
use derives::{AsMut, AsRef, Deref, DerefMut};
use derives::{Constructor, IsVariant};
use derives::{From, FromStr, Into, TryInto};

use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_plain::derive_display_from_serialize;
use serde_plain::derive_fromstr_from_deserialize;

use anyhow::Context as AnyhowContext;
use anyhow::{bail, ensure};
use anyhow::{Error, Result};

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::spawn;

use request::Client;
use tracing::{debug, error, info, warn};
use typed_builder::TypedBuilder as Builder;
use url::Url;

pub fn default<T: Default>() -> T {
    T::default()
}
