//! Query builder and macro
use std::marker::PhantomData;

use futures::stream::BoxStream;
use futures::{Stream, StreamExt};
use rorm_db::transaction::Transaction;
use rorm_db::{conditional::Condition, error::Error, row::Row, Database};
use rorm_declaration::hmr::db_type::DbType;

use crate::crud::builder::{ConditionMarker, TransactionMarker};
use crate::model::{AsDbType, Field, Model, Patch};

/// Builder for select queries
///
/// Is is recommended to start a builder using [query!].
///
/// [query!]: macro@crate::query
#[must_use]
pub struct QueryBuilder<
    'db,
    'a,
    M: Model,
    S: Selector<M>,
    C: ConditionMarker<'a>,
    T: TransactionMarker<'a, 'db>,
> {
    db: &'db Database,
    selector: &'a S,
    _phantom: PhantomData<M>,

    condition: C,
    transaction: T,
}

/// Specifies which columns to query and how to decode the rows into what.
pub trait Selector<M: Model> {
    /// Type as which rows should be decoded
    type Result;

    /// Decode a row
    fn decode(row: Row) -> Result<Self::Result, Error>;

    /// Columns to query
    fn columns(&self) -> &[&'static str];
}

impl<'db, 'rf, M: Model, S: Selector<M>> QueryBuilder<'db, 'rf, M, S, (), ()> {
    /// Start building a query using a generic [Selector]
    pub fn new(db: &'db Database, selector: &'rf S) -> QueryBuilder<'db, 'rf, M, S, (), ()> {
        QueryBuilder {
            db,
            selector,
            _phantom: PhantomData,

            condition: (),
            transaction: (),
        }
    }
}

impl<'db, 'a, M: Model, S: Selector<M>, T: TransactionMarker<'a, 'db>>
    QueryBuilder<'db, 'a, M, S, (), T>
{
    /// Add a condition to the query
    pub fn condition(
        self,
        condition: Condition<'a>,
    ) -> QueryBuilder<'db, 'a, M, S, Condition<'a>, T> {
        #[rustfmt::skip]
        let QueryBuilder { db, selector, _phantom, transaction, .. } = self;
        #[rustfmt::skip]
        return QueryBuilder { db, selector, _phantom, condition, transaction, };
    }
}

impl<'db, 'a, M: Model, S: Selector<M>, C: ConditionMarker<'a>> QueryBuilder<'db, 'a, M, S, C, ()> {
    /// Add a transaction to the query
    pub fn transaction(
        self,
        transaction: &'a mut Transaction<'db>,
    ) -> QueryBuilder<'db, 'a, M, S, C, &'a mut Transaction<'db>> {
        #[rustfmt::skip]
        let QueryBuilder { db, selector, _phantom, condition, .. } = self;
        #[rustfmt::skip]
        return QueryBuilder { db, selector, _phantom, condition, transaction, };
    }
}

impl<
        'rf,
        'db: 'rf,
        M: Model,
        S: Selector<M>,
        C: ConditionMarker<'rf>,
        T: TransactionMarker<'rf, 'db>,
    > QueryBuilder<'db, 'rf, M, S, C, T>
{
    /// Retrieve all matching rows
    pub async fn all_as_rows(self) -> Result<Vec<Row>, Error> {
        self.db
            .query_all(
                M::table_name(),
                self.selector.columns(),
                self.condition.as_option(),
                self.transaction.into_option(),
            )
            .await
    }

    /// Retrieve exactly one matching row
    ///
    /// An error is returned if no value could be retrieved.
    pub async fn one_as_row(self) -> Result<Row, Error> {
        self.db
            .query_one(
                M::table_name(),
                self.selector.columns(),
                self.condition.as_option(),
                self.transaction.into_option(),
            )
            .await
    }

    /// Retrieve the query as a stream of rows
    pub fn stream_as_row(self) -> BoxStream<'rf, Result<Row, Error>> {
        self.db.query_stream(
            M::table_name(),
            self.selector.columns(),
            self.condition.as_option(),
            self.transaction.into_option(),
        )
    }

    /// Try to retrieve the a matching row
    pub async fn optional_as_row(self) -> Result<Option<Row>, Error> {
        self.db
            .query_optional(
                M::table_name(),
                self.selector.columns(),
                self.condition.as_option(),
                self.transaction.into_option(),
            )
            .await
    }

    /// Retrieve and decode all matching rows
    pub async fn all(self) -> Result<Vec<S::Result>, Error> {
        self.all_as_rows()
            .await?
            .into_iter()
            .map(|x| {
                S::decode(x).map_err(|_| Error::DecodeError("Could not decode row".to_string()))
            })
            .collect::<Result<Vec<_>, _>>()
    }

    /// Retrieve and decode exactly one matching row
    ///
    /// An error is returned if no value could be retrieved.
    pub async fn one(self) -> Result<S::Result, Error> {
        S::decode(self.one_as_row().await?)
            .map_err(|_| Error::DecodeError("Could not decode row".to_string()))
    }

    /// Retrieve and decode the query as a stream
    pub fn stream(self) -> impl Stream<Item = Result<S::Result, Error>> + 'rf {
        self.stream_as_row().map(|row| {
            S::decode(row?).map_err(|_| Error::DecodeError("Could not decode row".to_string()))
        })
    }

    /// Try to retrieve and decode a matching row
    pub async fn optional(self) -> Result<Option<S::Result>, Error> {
        match self.optional_as_row().await? {
            None => Ok(None),
            Some(row) => {
                Ok(Some(S::decode(row).map_err(|_| {
                    Error::DecodeError("Could not decode row".to_string())
                })?))
            }
        }
    }
}

/// Create a SELECT query.
///
/// 1. Give a reference to your db and the patch to query.
///     If you just need a few fields and don't want to create a patch for it,
///     you can specify these fields directly as a tuple as well.
///
///     `query!(&db, MyModelType)`
///
///     `query!(&db, (MyModelType::F.some_field, MyModelType::F.another_field, ))`
///
/// 2. Set a condition which rows to query.
///
///     `.condition(MyModelType::F.some_field.equals("some_value"))`
///
/// 3. *Optionally* add this query to a transaction
///
///     `.transaction(&mut tr)`
///
/// 4. Finally specify how to get the queries results. This will also execute the query.
///     - Get [`all`](QueryBuilder::all) matching rows in a vector.
///
///         `.all().await`
///
///     - Get all matching rows in an async [`stream`](QueryBuilder::stream).
///
///         `.steam()`
///
///     - Just get exactly [`one`](QueryBuilder::one) row.
///
///         `.one().await`
///
///     - Get one row if any. ([`optional`](QueryBuilder::optional))
///
///         `.optional().await`
///
///     Each of these methods decodes the database's rows into the patch you specified in step 1.
///     If you want to work with raw rows, each of the methods in step 4 has a `*_as_row` twin.
///
/// Example:
/// ```no_run
/// # use rorm::{Model, Database, query};
/// #
/// # #[derive(Model)]
/// # struct User {
/// #     #[rorm(id)]
/// #     id: i64,
/// #     username: String,
/// #     password: String,
/// # }
/// #
/// #
/// # async fn shame_user(_user: &User) {}
/// #
/// pub async fn shame_users(db: &Database) {
///     for (id, password) in query!(db, (User::F.id, User::F.password)).all().await.unwrap() {
///         if password == "password" {
///             let user = query!(db, User)
///                 .condition(User::F.id.equals(id))
///                 .one()
///                 .await
///                 .unwrap();
///             shame_user(&user).await;
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! query {
    (replace $anything:tt with $result:tt) => { $result };
    ($db:expr, $patch:path) => {
        $crate::crud::query::QueryBuilder::new(
            $db,
            &$crate::crud::query::SelectPatch::<$patch>::new(),
        )
    };
    ($db:expr, ($($field:expr),+$(,)?)) => {
        $crate::crud::query::QueryBuilder::new(
            $db,
            &$crate::crud::query::SelectTuple::<_, { 0 $( + $crate::query!(replace {$field} with 1))+ }>::new(&($($field),+)),
        )
    };
}

/// The [Selector] for patches.
///
/// # Why implement `Selector` on `SelectPatch<P>` instead of `P` directly?
/// Since a selector is used by reference, it needs a runtime value.
/// But there wouldn't be any data to create a patch's instance with.
/// On top of that all that data would be ignored anyway,
/// because the columns to query are stored in the patch type.
///
/// => So create a struct without data to "wrap" the patch type.
pub struct SelectPatch<P: Patch>(PhantomData<P>);
impl<P: Patch> SelectPatch<P> {
    /// Create a SelectPatch
    pub const fn new() -> Self {
        SelectPatch(PhantomData)
    }
}
impl<M: Model, P: Patch<Model = M>> Selector<M> for SelectPatch<P> {
    type Result = P;

    fn decode(row: Row) -> Result<Self::Result, Error> {
        P::from_row(row)
    }

    fn columns(&self) -> &[&'static str] {
        P::COLUMNS
    }
}

/// The [Selector] for tuple
///
/// Implemented for tuple of size 8 or less.
///
/// # Why `SelectTuple<T, const C: usize>`?
/// Unlike patches (see [SelectPatch]) tuples are just normal datatypes. So why do they need a wrapper?
///
/// The [Selector] trait needs to produce a slice of columns to give to the database implementation.
/// For a patch this is easy since the trait stores it in the associated constant [Patch::COLUMNS].
/// For tuples the data is there, namely in `Field`'s `name` field, but stored in structs' fields in a tuple, not a simple slice.
/// So in order to have a slice, these `name` fields have to be copied into some storage.
/// But since `Selector::columns` just returns a slice, it can't do the copying, because the storage would be dropped immediately.
///
/// => So wrap the tuple and add an array of the correct size to copy the columns into.
pub struct SelectTuple<T, const C: usize> {
    tuple: PhantomData<T>,
    columns: [&'static str; C],
}
macro_rules! impl_select_tuple {
    ($C:literal, ($($index:tt: <$T:ident, $D:ident, $A:ident>,)+)) => {
        impl<M: Model, $($T, $D, $A,)+> SelectTuple<($(Field<$T, $D, M, $A>,)+), $C> {
            /// Create a SelectTuple
            pub const fn new(tuple: &($(Field<$T, $D, M, $A>,)+)) -> Self {
                Self {
                    tuple: PhantomData,
                    columns: [$(tuple.$index.name),+],
                }
            }
        }
        impl<M: Model, $($T: AsDbType, $D: DbType, $A,)+> Selector<M>
            for SelectTuple<($(Field<$T, $D, M, $A>,)+), $C>
        {
            type Result = ($($T,)+);

            fn decode(row: Row) -> Result<Self::Result, Error> {
                Ok(($($T::from_primitive(row.get::<$T::Primitive, usize>($index)?),)+))
            }

            fn columns(&self) -> &[&'static str] {
                &self.columns
            }
        }
    };
}
impl_select_tuple!(1, (0: <T0, D0, A0>,));
impl_select_tuple!(2, (0: <T0, D0, A0>, 1: <T1, D1, A1>,));
impl_select_tuple!(3, (0: <T0, D0, A0>, 1: <T1, D1, A1>, 2: <T2, D2, A2>,));
impl_select_tuple!(4, (0: <T0, D0, A0>, 1: <T1, D1, A1>, 2: <T2, D2, A2>, 3: <T3, D3, A3>,));
impl_select_tuple!(5, (0: <T0, D0, A0>, 1: <T1, D1, A1>, 2: <T2, D2, A2>, 3: <T3, D3, A3>, 4: <T4, D4, A4>,));
impl_select_tuple!(6, (0: <T0, D0, A0>, 1: <T1, D1, A1>, 2: <T2, D2, A2>, 3: <T3, D3, A3>, 4: <T4, D4, A4>, 5: <T5, D5, A5>,));
impl_select_tuple!(7, (0: <T0, D0, A0>, 1: <T1, D1, A1>, 2: <T2, D2, A2>, 3: <T3, D3, A3>, 4: <T4, D4, A4>, 5: <T5, D5, A5>, 6: <T6, D6, A6>,));
impl_select_tuple!(8, (0: <T0, D0, A0>, 1: <T1, D1, A1>, 2: <T2, D2, A2>, 3: <T3, D3, A3>, 4: <T4, D4, A4>, 5: <T5, D5, A5>, 6: <T6, D6, A6>, 7: <T7, D7, A7>,));
