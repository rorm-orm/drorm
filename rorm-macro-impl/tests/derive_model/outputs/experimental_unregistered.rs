///rorm's representation of [`Test`]'s `id` field
#[allow(non_camel_case_types)]
pub struct __Test_id(::std::marker::PhantomData<()>);
impl ::std::clone::Clone for __Test_id {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::marker::Copy for __Test_id {}
impl ::rorm::internal::field::Field for __Test_id {
    type Type = i64;
    type Model = Test;
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
///[`Test`]'s [`Fields`](::rorm::model::Model::Fields) struct.
#[allow(non_camel_case_types)]
pub struct __Test_Fields_Struct<Path: 'static> {
    ///[`Test`]'s `id` field
    pub id: ::rorm::internal::field::FieldProxy<__Test_id, Path>,
}
impl<Path: 'static> ::rorm::model::ConstNew for __Test_Fields_Struct<Path> {
    const NEW: Self = Self {
        id: ::rorm::internal::field::FieldProxy::new(),
    };
    const REF: &'static Self = &Self::NEW;
}
const _: () = {
    impl ::rorm::model::Model for Test {
        type Primary = __Test_id;
        type Fields<P: ::rorm::internal::relation_path::Path> = __Test_Fields_Struct<P>;
        const F: __Test_Fields_Struct<Self> = ::rorm::model::ConstNew::NEW;
        const FIELDS: __Test_Fields_Struct<Self> = ::rorm::model::ConstNew::NEW;
        const TABLE: &'static str = "test";
        fn get_imr() -> ::rorm::imr::Model {
            use ::rorm::internal::field::Field;
            let mut fields = Vec::new();
            ::rorm::internal::field::push_imr::<__Test_id>(&mut fields);
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
    pub struct __Test_Decoder {
        id: <i64 as ::rorm::fields::traits::FieldType>::Decoder,
    }
    impl ::rorm::crud::decoder::Decoder for __Test_Decoder {
        type Result = Test;
        fn by_name<'index>(
            &'index self,
            row: &'_ ::rorm::db::Row,
        ) -> Result<Self::Result, ::rorm::db::row::RowError<'index>> {
            Ok(Test { id: self.id.by_name(row)? })
        }
        fn by_index<'index>(
            &'index self,
            row: &'_ ::rorm::db::Row,
        ) -> Result<Self::Result, ::rorm::db::row::RowError<'index>> {
            Ok(Test { id: self.id.by_index(row)? })
        }
    }
    impl ::rorm::model::Patch for Test {
        type Model = Test;
        type Decoder = __Test_Decoder;
        fn select<P: ::rorm::internal::relation_path::Path>(
            ctx: &mut ::rorm::internal::query_context::QueryContext,
        ) -> Self::Decoder {
            __Test_Decoder {
                id: FieldDecoder::new(
                    ctx,
                    <<Self as ::rorm::model::Patch>::Model as ::rorm::model::Model>::FIELDS
                        .id
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
        }
        fn push_references<'a>(
            &'a self,
            values: &mut Vec<::rorm::conditions::Value<'a>>,
        ) {
            values.extend(self.id.as_values());
        }
        fn push_values(self, values: &mut Vec<::rorm::conditions::Value>) {
            values.extend(self.id.into_values());
        }
    }
    impl<'a> ::rorm::internal::patch::IntoPatchCow<'a> for Test {
        type Patch = Test;
        fn into_patch_cow(self) -> ::rorm::internal::patch::PatchCow<'a, Test> {
            ::rorm::internal::patch::PatchCow::Owned(self)
        }
    }
    impl<'a> ::rorm::internal::patch::IntoPatchCow<'a> for &'a Test {
        type Patch = Test;
        fn into_patch_cow(self) -> ::rorm::internal::patch::PatchCow<'a, Test> {
            ::rorm::internal::patch::PatchCow::Borrowed(self)
        }
    }
};
impl ::rorm::model::FieldByIndex<{ 0usize }> for Test {
    type Field = __Test_id;
}
impl ::rorm::model::GetField<__Test_id> for Test {
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
