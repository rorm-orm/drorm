///rorm's representation of [`Foo`]'s `id` field
#[allow(non_camel_case_types)]
pub struct __Foo_id<X: 'static + FieldType>(::std::marker::PhantomData<(X,)>);
impl<X: 'static + FieldType> ::std::clone::Clone for __Foo_id<X> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<X: 'static + FieldType> ::std::marker::Copy for __Foo_id<X> {}
impl<X: 'static + FieldType> ::rorm::internal::field::Field for __Foo_id<X> {
    type Type = i64;
    type Model = Foo<X>;
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
    const SOURCE: Option<::rorm::internal::hmr::Source> = None;
    fn new() -> Self {
        Self(::std::marker::PhantomData)
    }
}
///rorm's representation of [`Foo`]'s `x` field
#[allow(non_camel_case_types)]
pub struct __Foo_x<X: 'static + FieldType>(::std::marker::PhantomData<(X,)>);
impl<X: 'static + FieldType> ::std::clone::Clone for __Foo_x<X> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<X: 'static + FieldType> ::std::marker::Copy for __Foo_x<X> {}
impl<X: 'static + FieldType> ::rorm::internal::field::Field for __Foo_x<X> {
    type Type = X;
    type Model = Foo<X>;
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
    const SOURCE: Option<::rorm::internal::hmr::Source> = None;
    fn new() -> Self {
        Self(::std::marker::PhantomData)
    }
}
///[`Foo`]'s [`Fields`](::rorm::model::Model::Fields) struct.
#[allow(non_camel_case_types)]
pub struct __Foo_Fields_Struct<X: 'static + FieldType, Path: 'static> {
    ///[`Foo`]'s `id` field
    pub id: ::rorm::internal::field::FieldProxy<__Foo_id<X>, Path>,
    ///[`Foo`]'s `x` field
    pub x: ::rorm::internal::field::FieldProxy<__Foo_x<X>, Path>,
}
impl<X: 'static + FieldType, Path: 'static> ::rorm::model::ConstNew
for __Foo_Fields_Struct<X, Path> {
    const NEW: Self = Self {
        id: ::rorm::internal::field::FieldProxy::new(),
        x: ::rorm::internal::field::FieldProxy::new(),
    };
    const REF: &'static Self = &Self::NEW;
}
const _: () = {
    impl<X: 'static + FieldType> ::rorm::model::Model for Foo<X> {
        type Primary = __Foo_id<X>;
        type Fields<P: ::rorm::internal::relation_path::Path> = __Foo_Fields_Struct<
            X,
            P,
        >;
        const F: __Foo_Fields_Struct<X, Self> = ::rorm::model::ConstNew::NEW;
        const FIELDS: __Foo_Fields_Struct<X, Self> = ::rorm::model::ConstNew::NEW;
        const TABLE: &'static str = "foo";
        fn get_imr() -> ::rorm::imr::Model {
            use ::rorm::internal::field::Field;
            let mut fields = Vec::new();
            ::rorm::internal::field::push_imr::<__Foo_id<X>>(&mut fields);
            ::rorm::internal::field::push_imr::<__Foo_x<X>>(&mut fields);
            ::rorm::imr::Model {
                name: Self::TABLE.to_string(),
                fields,
                source_defined_at: None,
            }
        }
        type Permissions = __Permissions;
        type InsertPermission = __InsertPermission;
        type QueryPermission = __QueryPermission;
        type UpdatePermission = __UpdatePermission;
        type DeletePermission = __DeletePermission;
    }
    #[derive(Default)]
    pub struct __Permissions;
    pub type __InsertPermission = ::rorm::model::Unrestricted;
    pub type __QueryPermission = ::rorm::model::Unrestricted;
    pub type __UpdatePermission = ::rorm::model::Unrestricted;
    pub type __DeletePermission = ::rorm::model::Unrestricted;
    impl __Permissions {
        pub fn insert_permission(&self) -> __InsertPermission {
            ::rorm::model::Unrestricted(::std::marker::PhantomData)
        }
        pub fn query_permission(&self) -> __QueryPermission {
            ::rorm::model::Unrestricted(::std::marker::PhantomData)
        }
        pub fn update_permission(&self) -> __UpdatePermission {
            ::rorm::model::Unrestricted(::std::marker::PhantomData)
        }
        pub fn delete_permission(&self) -> __DeletePermission {
            ::rorm::model::Unrestricted(::std::marker::PhantomData)
        }
    }
    use ::rorm::internal::field::decoder::FieldDecoder;
    use ::rorm::fields::traits::FieldType;
    pub struct __Foo_Decoder<X: 'static + FieldType> {
        id: <i64 as ::rorm::fields::traits::FieldType>::Decoder,
        x: <X as ::rorm::fields::traits::FieldType>::Decoder,
    }
    impl<X: 'static + FieldType> ::rorm::crud::decoder::Decoder for __Foo_Decoder<X> {
        type Result = Foo<X>;
        fn by_name<'index>(
            &'index self,
            row: &'_ ::rorm::db::Row,
        ) -> Result<Self::Result, ::rorm::db::row::RowError<'index>> {
            Ok(Foo {
                id: self.id.by_name(row)?,
                x: self.x.by_name(row)?,
            })
        }
        fn by_index<'index>(
            &'index self,
            row: &'_ ::rorm::db::Row,
        ) -> Result<Self::Result, ::rorm::db::row::RowError<'index>> {
            Ok(Foo {
                id: self.id.by_index(row)?,
                x: self.x.by_index(row)?,
            })
        }
    }
    impl<X: 'static + FieldType> ::rorm::model::Patch for Foo<X> {
        type Model = Foo<X>;
        type Decoder = __Foo_Decoder<X>;
        fn select<P: ::rorm::internal::relation_path::Path>(
            ctx: &mut ::rorm::internal::query_context::QueryContext,
        ) -> Self::Decoder {
            __Foo_Decoder {
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
    impl<'a, X: 'static + FieldType> ::rorm::internal::patch::IntoPatchCow<'a>
    for Foo<X> {
        type Patch = Foo<X>;
        fn into_patch_cow(self) -> ::rorm::internal::patch::PatchCow<'a, Foo<X>> {
            ::rorm::internal::patch::PatchCow::Owned(self)
        }
    }
    impl<'a, X: 'static + FieldType> ::rorm::internal::patch::IntoPatchCow<'a>
    for &'a Foo<X> {
        type Patch = Foo<X>;
        fn into_patch_cow(self) -> ::rorm::internal::patch::PatchCow<'a, Foo<X>> {
            ::rorm::internal::patch::PatchCow::Borrowed(self)
        }
    }
};
impl<X: 'static + FieldType> ::rorm::model::FieldByIndex<{ 0usize }> for Foo<X> {
    type Field = __Foo_id<X>;
}
impl<X: 'static + FieldType> ::rorm::model::GetField<__Foo_id<X>> for Foo<X> {
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
impl<X: 'static + FieldType> ::rorm::model::FieldByIndex<{ 1usize }> for Foo<X> {
    type Field = __Foo_x<X>;
}
impl<X: 'static + FieldType> ::rorm::model::GetField<__Foo_x<X>> for Foo<X> {
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
impl<X: 'static + FieldType> ::rorm::model::UpdateField<__Foo_x<X>> for Foo<X> {
    fn update_field<'m, T>(
        &'m mut self,
        update: impl FnOnce(&'m i64, &'m mut X) -> T,
    ) -> T {
        update(&self.id, &mut self.x)
    }
}
