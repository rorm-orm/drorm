//! Update builder and macro

use std::marker::PhantomData;

use rorm_db::database;
use rorm_db::error::Error;
use rorm_db::executor::Executor;

use crate::conditions::{Condition, DynamicCollection, Value};
use crate::internal::field::{FieldProxy, SingleColumnField};
use crate::internal::query_context::QueryContext;
use crate::model::Identifiable;
use crate::{Model, Patch};

/// Wrapper around `Vec` to indicate on type level, that possible no column has been set yet.
pub struct OptionalColumns<'a>(Vec<(&'static str, Value<'a>)>);

/// Builder for update queries
///
/// It is recommended to start a builder using [`update!`](macro@crate::update).
///
/// ## Generics
/// - `'rf`
///
///     Lifetime of external values (eg: condition values).
///
/// - `E`: [`Executor`]
///
///     The executor to query with.
///
/// - `M`: [`Model`](Model)
///
///     The model from whose table to update rows.
///
/// - `L`
///
///     List of columns and values to set.
///     This is a generic instead of just being a `Vec` in order to prevent the list from being empty.
#[must_use]
pub struct UpdateBuilder<'rf, E, M, L> {
    executor: E,
    columns: L,

    _phantom: PhantomData<(&'rf (), M)>,
}

impl<'rf, 'e, E, M> UpdateBuilder<'rf, E, M, ()>
where
    E: Executor<'e>,
    M: Model,
{
    /// Start building a delete query
    pub fn new(executor: E, _: M::UpdatePermission) -> Self {
        Self {
            executor,
            columns: (),

            _phantom: PhantomData,
        }
    }
}

impl<'rf, E, M> UpdateBuilder<'rf, E, M, ()> {
    /// Prepare the builder to accept a dynamic (possibly zero) amount of set calls.
    ///
    /// Call [`finish_dyn_set`](UpdateBuilder::finish_dyn_set) to go back to normal operation.
    ///
    /// Normally `set` would use the type system to ensure that it has been called at least once
    /// before executing the query.
    /// This can be troublesome, when you want to call it dynamically
    /// and can't ensure that at least one such call will happen.
    pub fn begin_dyn_set(self) -> UpdateBuilder<'rf, E, M, OptionalColumns<'rf>> {
        #[rustfmt::skip]
        let UpdateBuilder { executor, _phantom, .. } = self;
        #[rustfmt::skip]
        return UpdateBuilder { executor, columns: OptionalColumns(Vec::new()), _phantom, };
    }
}

impl<'rf, E, M> UpdateBuilder<'rf, E, M, OptionalColumns<'rf>> {
    /// Add a column to update.
    ///
    /// Can be called multiple times.
    pub fn set<F: SingleColumnField>(self, _field: FieldProxy<F, M>, value: F::Type) -> Self {
        let mut builder = self;
        builder.columns.0.push((F::NAME, F::type_into_value(value)));
        builder
    }

    /// Add a column to update if `value` is `Some`
    ///
    /// Can be called multiple times.
    pub fn set_if<F: SingleColumnField>(
        self,
        field: FieldProxy<F, M>,
        value: Option<F::Type>,
    ) -> Self {
        if let Some(value) = value {
            self.set(field, value)
        } else {
            self
        }
    }

    /// Go back to a "normal" builder after calling [`begin_dyn_set`](UpdateBuilder::begin_dyn_set).
    ///
    /// This will check if `set` has been called at least once.
    /// If it hasn't, the "unset" builder will be returned as `Err`.
    pub fn finish_dyn_set(
        self,
    ) -> Result<UpdateBuilderWithSet<'rf, E, M>, UpdateBuilderWithoutSet<'rf, E, M>> {
        #[rustfmt::skip]
        let UpdateBuilder { executor, _phantom, columns } = self;
        #[rustfmt::skip]
        return if columns.0.is_empty() {
            Err(UpdateBuilder { executor, columns: (), _phantom, })
        } else {
            Ok(UpdateBuilder { executor, columns: columns.0, _phantom, })
        };
    }
}
type UpdateBuilderWithoutSet<'rf, E, M> = UpdateBuilder<'rf, E, M, ()>;
type UpdateBuilderWithSet<'rf, E, M> = UpdateBuilder<'rf, E, M, Vec<(&'static str, Value<'rf>)>>;

impl<'rf, E, M> UpdateBuilder<'rf, E, M, ()>
where
    M: Model,
{
    /// Add a column to update.
    ///
    /// Can be called multiple times.
    pub fn set<F: SingleColumnField>(
        self,
        _field: FieldProxy<F, M>,
        value: F::Type,
    ) -> UpdateBuilder<'rf, E, M, Vec<(&'static str, Value<'rf>)>> {
        #[rustfmt::skip]
        let UpdateBuilder { executor, _phantom, .. } = self;
        #[rustfmt::skip]
        return UpdateBuilder { executor, columns: vec![(F::NAME, F::type_into_value(value))], _phantom, };
    }
}

impl<'rf, E, M> UpdateBuilder<'rf, E, M, Vec<(&'static str, Value<'rf>)>>
where
    M: Model,
{
    /// Add a column to update.
    ///
    /// Can be called multiple times.
    pub fn set<F: SingleColumnField>(self, _field: FieldProxy<F, M>, value: F::Type) -> Self {
        let mut builder = self;
        builder.columns.push((F::NAME, F::type_into_value(value)));
        builder
    }
}

impl<'ex, 'rf, E, M> UpdateBuilder<'rf, E, M, Vec<(&'static str, Value<'rf>)>>
where
    E: Executor<'ex>,
    M: Model,
{
    /// Update a single row identified by a patch instance
    ///
    /// Note: The patch only provides the primary key, its other values will be ignored.
    pub async fn single<P>(self, patch: &P) -> Result<u64, Error>
    where
        P: Patch<Model = M> + Identifiable,
    {
        self.condition(patch.as_condition()).await
    }

    /// Update a bulk of rows identified by patch instances
    ///
    /// Note: The patches only provide the primary key, their other values will be ignored.
    pub async fn bulk<P>(self, patches: impl IntoIterator<Item = &P>) -> Result<u64, Error>
    where
        P: Patch<Model = M> + Identifiable,
    {
        self.condition(DynamicCollection::or(
            patches
                .into_iter()
                .map(|patch| patch.as_condition())
                .collect(),
        ))
        .await
    }

    /// Update all rows matching a condition
    pub async fn condition<C: Condition<'rf>>(self, condition: C) -> Result<u64, Error> {
        let mut context = QueryContext::new();
        let columns: Vec<_> = self
            .columns
            .iter()
            .map(|(name, value)| (*name, value.as_sql()))
            .collect();
        let condition_index = context.add_condition(&condition);
        let condition = context.get_condition(condition_index);
        database::update(self.executor, M::TABLE, &columns, Some(&condition)).await
    }

    /// Update all rows
    pub async fn all(self) -> Result<u64, Error> {
        let columns: Vec<_> = self
            .columns
            .iter()
            .map(|(name, value)| (*name, value.as_sql()))
            .collect();
        database::update(self.executor, M::TABLE, &columns, None).await
    }
}

/// Create a UPDATE query.
///
/// # Basic usage
/// ```no_run
/// # use rorm::{Model, Database, update, FieldAccess};
/// # #[derive(Model)] struct User { #[rorm(id)] id: i64, #[rorm(max_length = 255)] password: String, }
/// pub async fn set_good_password(db: &Database) {
///     update!(db, User)
///         .set(User::F.password, "I am way more secure™".to_string())
///         .condition(User::F.password.equals("password"))
///         .await
///         .unwrap();
/// }
/// ```
///
/// Like every crud macro `update!` starts a [builder](UpdateBuilder) which is consumed to execute the query.
///
/// `update!`'s first argument is a reference to the [`Database`](crate::Database).
/// Its second is the [`Model`] type you want to update rows of.
///
/// # Dynamic number of [`set`](UpdateBuilder::set)
/// ```no_run
/// # use std::collections::HashMap;
/// # use rorm::{Model, Database, update, FieldAccess};
/// # #[derive(Model)] struct User { #[rorm(id)] id: i64, #[rorm(max_length = 255)] nickname: String, #[rorm(max_length = 255)] password: String, }
/// /// POST endpoint allowing a user to change its nickname or password
/// pub async fn update_user(db: &Database, id: i64, post_params: HashMap<String, String>) {
///     let mut builder = update!(db, User).begin_dyn_set();
///
///     if let Some(password) = post_params.get("password") {
///         builder = builder.set(User::F.password, password.clone());
///     }
///     if let Some(nickname) = post_params.get("nickname") {
///         builder = builder.set(User::F.nickname, nickname.clone())
///     }
///
///     if let Ok(builder) = builder.finish_dyn_set() {
///         builder.condition(User::F.id.equals(id)).await.unwrap();
///     } else {
///         panic!("Invalid POST request: missing fields to update")
///     }
/// }
/// ```
///
/// Before executing the query [`set`](UpdateBuilder::set) has to be called at least once
/// to set a value to set for a column (The first call changes the builders type).
/// Otherwise the query wouldn't do anything.
///
/// This can be limiting when your calls are made conditionally.
///
/// To support this, the builder can be put into a "dynamic" mode by calling [begin_dyn_set](UpdateBuilder::begin_dyn_set).
/// Then calls to [`set`](UpdateBuilder::set) won't change the type.
/// When you're done use [finish_dyn_set](UpdateBuilder::finish_dyn_set) to go back to "normal" mode.
/// It will check the number of "sets" and return `Result` which is `Ok` for at least one and an
/// `Err` for zero.
/// Both variants contain the builder in "normal" mode to continue.
#[macro_export]
macro_rules! update {
    ($db:expr, $model:path) => {
        $crate::update!(
            $db,
            $model,
            perm = <<$model as $crate::model::Patch>::Model as $crate::model::Model>::permissions()
                .update_permission()
        )
    };
    ($db:expr, $model:path, perm = $perm:expr) => {
        $crate::crud::update::UpdateBuilder::<_, $model, _, _>::new($db, $perm)
    };
}
