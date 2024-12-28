///rorm's representation of [`Generic`]'s `id` field
#[allow(non_camel_case_types)]
pub struct __Generic_id<X: rorm::fields::traits::FieldType>(
    ::std::marker::PhantomData<(X,)>,
);
impl<X: rorm::fields::traits::FieldType> ::std::clone::Clone for __Generic_id<X> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<X: rorm::fields::traits::FieldType> ::std::marker::Copy for __Generic_id<X> {}
impl<X: rorm::fields::traits::FieldType> ::rorm::internal::field::Field
for __Generic_id<X> {
    type Type = i64;
    type Model = Generic<X>;
    const INDEX: usize = 0usize;
    const NAME: &'static str = "id";
    const EXPLICIT_ANNOTATIONS: ::rorm::internal::hmr::annotations::Annotations = ::rorm::internal::hmr::annotations::Annotations {
        auto_create_time: None,
        auto_update_time: None,
        auto_increment: Some(::rorm::internal::hmr::annotations::AutoIncrement),
        choices: None,
        default: None,
        index: None,
        max_length: None,
        on_delete: None,
        on_update: None,
        primary_key: Some(::rorm::internal::hmr::annotations::PrimaryKey),
        unique: None,
        nullable: false,
        foreign: None,
    };
    const SOURCE: ::rorm::internal::hmr::Source = ::rorm::internal::hmr::Source {
        file: ::std::file!(),
        line: ::std::line!() as usize,
        column: ::std::column!() as usize,
    };
    fn new() -> Self {
        Self(::std::marker::PhantomData)
    }
}
///rorm's representation of [`Generic`]'s `x` field
#[allow(non_camel_case_types)]
pub struct __Generic_x<X: rorm::fields::traits::FieldType>(
    ::std::marker::PhantomData<(X,)>,
);
impl<X: rorm::fields::traits::FieldType> ::std::clone::Clone for __Generic_x<X> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<X: rorm::fields::traits::FieldType> ::std::marker::Copy for __Generic_x<X> {}
impl<X: rorm::fields::traits::FieldType> ::rorm::internal::field::Field
for __Generic_x<X> {
    type Type = X;
    type Model = Generic<X>;
    const INDEX: usize = 1usize;
    const NAME: &'static str = "x";
    const EXPLICIT_ANNOTATIONS: ::rorm::internal::hmr::annotations::Annotations = ::rorm::internal::hmr::annotations::Annotations {
        auto_create_time: None,
        auto_update_time: None,
        auto_increment: None,
        choices: None,
        default: None,
        index: None,
        max_length: None,
        on_delete: None,
        on_update: None,
        primary_key: None,
        unique: None,
        nullable: false,
        foreign: None,
    };
    const SOURCE: ::rorm::internal::hmr::Source = ::rorm::internal::hmr::Source {
        file: ::std::file!(),
        line: ::std::line!() as usize,
        column: ::std::column!() as usize,
    };
    fn new() -> Self {
        Self(::std::marker::PhantomData)
    }
}
///[`Generic`]'s [`Fields`](::rorm::model::Model::Fields) struct.
#[allow(non_camel_case_types)]
pub struct __Generic_Fields_Struct<X: rorm::fields::traits::FieldType, Path: 'static> {
    ///[`Generic`]'s `id` field
    pub id: ::rorm::internal::field::FieldProxy<__Generic_id<X>, Path>,
    ///[`Generic`]'s `x` field
    pub x: ::rorm::internal::field::FieldProxy<__Generic_x<X>, Path>,
}
impl<X: rorm::fields::traits::FieldType, Path: 'static> ::rorm::model::ConstNew
for __Generic_Fields_Struct<X, Path> {
    const NEW: Self = Self {
        id: ::rorm::internal::field::FieldProxy::new(),
        x: ::rorm::internal::field::FieldProxy::new(),
    };
    const REF: &'static Self = &Self::NEW;
}
const _: () = {
    impl<X: rorm::fields::traits::FieldType> ::rorm::model::Model for Generic<X> {
        type Primary = __Generic_id<X>;
        type Fields<P: ::rorm::internal::relation_path::Path> = __Generic_Fields_Struct<
            X,
            P,
        >;
        const F: __Generic_Fields_Struct<X, Self> = ::rorm::model::ConstNew::NEW;
        const FIELDS: __Generic_Fields_Struct<X, Self> = ::rorm::model::ConstNew::NEW;
        const TABLE: &'static str = "generic";
        const SOURCE: ::rorm::internal::hmr::Source = ::rorm::internal::hmr::Source {
            file: ::std::file!(),
            line: ::std::line!() as usize,
            column: ::std::column!() as usize,
        };
        fn push_fields_imr(fields: &mut Vec<::rorm::imr::Field>) {
            ::rorm::internal::field::push_imr::<__Generic_id<X>>(&mut *fields);
            ::rorm::internal::field::push_imr::<__Generic_x<X>>(&mut *fields);
        }
    }
    use ::rorm::internal::field::decoder::FieldDecoder;
    use ::rorm::fields::traits::FieldType;
    pub struct __Generic_Decoder<X: rorm::fields::traits::FieldType> {
        id: <i64 as ::rorm::fields::traits::FieldType>::Decoder,
        x: <X as ::rorm::fields::traits::FieldType>::Decoder,
    }
    impl<X: rorm::fields::traits::FieldType> ::rorm::crud::decoder::Decoder
    for __Generic_Decoder<X> {
        type Result = Generic<X>;
        fn by_name<'index>(
            &'index self,
            row: &'_ ::rorm::db::Row,
        ) -> Result<Self::Result, ::rorm::db::row::RowError<'index>> {
            Ok(Generic {
                id: self.id.by_name(row)?,
                x: self.x.by_name(row)?,
            })
        }
        fn by_index<'index>(
            &'index self,
            row: &'_ ::rorm::db::Row,
        ) -> Result<Self::Result, ::rorm::db::row::RowError<'index>> {
            Ok(Generic {
                id: self.id.by_index(row)?,
                x: self.x.by_index(row)?,
            })
        }
    }
    impl<X: rorm::fields::traits::FieldType> ::rorm::model::Patch for Generic<X> {
        type Model = Generic<X>;
        type Decoder = __Generic_Decoder<X>;
        fn select<P: ::rorm::internal::relation_path::Path>(
            ctx: &mut ::rorm::internal::query_context::QueryContext,
        ) -> Self::Decoder {
            __Generic_Decoder {
                id: FieldDecoder::new(
                    ctx,
                    <<Self as ::rorm::model::Patch>::Model as ::rorm::model::Model>::FIELDS
                        .id
                        .through::<P>(),
                ),
                x: FieldDecoder::new(
                    ctx,
                    <<Self as ::rorm::model::Patch>::Model as ::rorm::model::Model>::FIELDS
                        .x
                        .through::<P>(),
                ),
            }
        }
        fn push_columns(columns: &mut Vec<&'static str>) {
            columns
                .extend(
                    ::rorm::internal::field::FieldProxy::columns(
                        <<Self as ::rorm::model::Patch>::Model as ::rorm::model::Model>::FIELDS
                            .id,
                    ),
                );
            columns
                .extend(
                    ::rorm::internal::field::FieldProxy::columns(
                        <<Self as ::rorm::model::Patch>::Model as ::rorm::model::Model>::FIELDS
                            .x,
                    ),
                );
        }
        fn push_references<'a>(
            &'a self,
            values: &mut Vec<::rorm::conditions::Value<'a>>,
        ) {
            values.extend(self.id.as_values());
            values.extend(self.x.as_values());
        }
        fn push_values(self, values: &mut Vec<::rorm::conditions::Value>) {
            values.extend(self.id.into_values());
            values.extend(self.x.into_values());
        }
    }
    impl<
        'a,
        X: rorm::fields::traits::FieldType,
    > ::rorm::internal::patch::IntoPatchCow<'a> for Generic<X> {
        type Patch = Generic<X>;
        fn into_patch_cow(self) -> ::rorm::internal::patch::PatchCow<'a, Generic<X>> {
            ::rorm::internal::patch::PatchCow::Owned(self)
        }
    }
    impl<
        'a,
        X: rorm::fields::traits::FieldType,
    > ::rorm::internal::patch::IntoPatchCow<'a> for &'a Generic<X> {
        type Patch = Generic<X>;
        fn into_patch_cow(self) -> ::rorm::internal::patch::PatchCow<'a, Generic<X>> {
            ::rorm::internal::patch::PatchCow::Borrowed(self)
        }
    }
};
impl<X: rorm::fields::traits::FieldType> ::rorm::model::FieldByIndex<{ 0usize }>
for Generic<X> {
    type Field = __Generic_id<X>;
}
impl<X: rorm::fields::traits::FieldType> ::rorm::model::GetField<__Generic_id<X>>
for Generic<X> {
    fn get_field(self) -> i64 {
        self.id
    }
    fn borrow_field(&self) -> &i64 {
        &self.id
    }
    fn borrow_field_mut(&mut self) -> &mut i64 {
        &mut self.id
    }
}
impl<X: rorm::fields::traits::FieldType> ::rorm::model::FieldByIndex<{ 1usize }>
for Generic<X> {
    type Field = __Generic_x<X>;
}
impl<X: rorm::fields::traits::FieldType> ::rorm::model::GetField<__Generic_x<X>>
for Generic<X> {
    fn get_field(self) -> X {
        self.x
    }
    fn borrow_field(&self) -> &X {
        &self.x
    }
    fn borrow_field_mut(&mut self) -> &mut X {
        &mut self.x
    }
}
impl<X: rorm::fields::traits::FieldType> ::rorm::model::UpdateField<__Generic_x<X>>
for Generic<X> {
    fn update_field<'m, T>(
        &'m mut self,
        update: impl FnOnce(&'m i64, &'m mut X) -> T,
    ) -> T {
        update(&self.id, &mut self.x)
    }
}
