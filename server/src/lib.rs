#![allow(unused_imports)]

pub mod api;
pub mod app;
pub mod config;
pub mod handlers;

use std::collections::HashMap as Map;
use std::collections::HashSet as Set;
use std::convert::{Infallible, TryFrom, TryInto};
use std::error::Error as StdError;
use std::fmt::{Debug, Display};
use std::future::ready;
use std::future::Future;
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::Deref;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration as StdDuration;

use futures::future::{join, join_all, try_join, try_join_all};
use futures::future::{select, try_select};
use futures::future::{FutureExt, TryFuture, TryFutureExt};
use futures::stream::{StreamExt, TryStream, TryStreamExt};
use futures::Stream;

use derives::Display;
use derives::{AsMut, AsRef, Deref, DerefMut};
use derives::{Constructor, IsVariant};
use derives::{From, FromStr, Into, TryInto};

use serde::de::Error as DeserializeError;
use serde::ser::Error as SerializeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::from_str as from_json_str;
use serde_json::from_value as from_json;
use serde_json::json;
use serde_json::to_string as to_json_string;
use serde_json::to_value as to_json;
use serde_json::Value as Json;
use serde_plain::derive_display_from_serialize;
use serde_plain::derive_fromstr_from_deserialize;

use chrono::NaiveDate as Date;
use chrono::NaiveTime as Time;
use chrono::{Duration, FixedOffset, TimeZone, Utc};

use anyhow::Context as AnyhowContext;
use anyhow::{bail, ensure};
use anyhow::{Error, Result};

use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::io::{AsyncWrite, AsyncWriteExt};
use tokio::spawn;

use derivative::Derivative;
use request::Client;
use tracing::{debug, error, info, trace, warn};
use typed_builder::TypedBuilder as Builder;
use url::Url;

pub fn default<T: Default>() -> T {
    T::default()
}

type DateTime<Tz = Utc> = chrono::DateTime<Tz>;

pub fn format_json<T: Serialize>(value: &T) -> String {
    match to_json_string(value) {
        Ok(s) => s,
        Err(error) => {
            format!("(invalid JSON: {:#})", &error)
        }
    }
}
