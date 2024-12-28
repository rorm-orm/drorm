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
    const SOURCE: ::rorm::internal::hmr::Source = ::rorm::internal::hmr::Source {
        file: ::std::file!(),
        line: ::std::line!() as usize,
        column: ::std::column!() as usize,
    };
    fn new() -> Self {
        Self(::std::marker::PhantomData)
    }
}
const _: () = {
    if let Err(err) = ::rorm::internal::field::check::<__Test_id>() {
        panic!("{}", err.as_str());
    }
};
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
        const SOURCE: ::rorm::internal::hmr::Source = ::rorm::internal::hmr::Source {
            file: ::std::file!(),
            line: ::std::line!() as usize,
            column: ::std::column!() as usize,
        };
        fn push_fields_imr(fields: &mut Vec<::rorm::imr::Field>) {
            ::rorm::internal::field::push_imr::<__Test_id>(&mut *fields);
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
#[doc = concat!("Constant representing the model [`", stringify!(Test), "`] as a value")]
#[allow(non_upper_case_globals)]
pub const Test: __Test_Fields_Struct<Test> = ::rorm::model::ConstNew::NEW;
const _: () = {
    #[::rorm::linkme::distributed_slice(::rorm::MODELS)]
    #[linkme(crate = ::rorm::linkme)]
    static __get_imr: fn() -> ::rorm::imr::Model = <Test as ::rorm::model::Model>::get_imr;
    let mut count_auto_increment = 0;
    let mut annos_slice = <__Test_id as ::rorm::internal::field::Field>::EFFECTIVE_ANNOTATIONS
        .as_slice();
    while let [annos, tail @ ..] = annos_slice {
        annos_slice = tail;
        if annos.auto_increment.is_some() {
            count_auto_increment += 1;
        }
    }
    assert!(
        count_auto_increment <= 1, "\"auto_increment\" can only be set once per model"
    );
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
