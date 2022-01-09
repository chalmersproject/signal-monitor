mod mutation;
mod query;
mod subscription;

pub use mutation::*;
pub use query::*;
pub use subscription::*;

mod date;
mod date_time;
mod test;

use date::*;
use date_time::*;
use test::*;

use super::*;

use graphql::scalar;
use graphql::Context;
use graphql::Value;
use graphql::{Enum, EnumType};
use graphql::{FieldError, FieldResult};
use graphql::{InputObject, InputObjectType};
use graphql::{InputValueError, InputValueResult};
use graphql::{Interface, InterfaceType};
use graphql::{MergedObject, Object, ObjectType, SimpleObject};
use graphql::{MergedSubscription, Subscription, SubscriptionType};
use graphql::{Scalar, ScalarType};
use graphql::{Union, UnionType};

trait IntoFieldResult<T> {
    fn into_field_result(self) -> FieldResult<T>;
}

impl<T> IntoFieldResult<T> for Result<T> {
    fn into_field_result(self) -> FieldResult<T> {
        self.map_err(into_field_error)
    }
}

fn into_field_error(error: Error) -> FieldError {
    FieldError::new(format!("{:#}", error))
}
