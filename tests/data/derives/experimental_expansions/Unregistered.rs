///rorm's representation of [`Unregistered`]'s `id` field
#[allow(non_camel_case_types)]
pub struct __Unregistered_id(::std::marker::PhantomData<()>);
impl ::rorm::new::Field for __Unregistered_id {
    type Struct = Unregistered;
    const POSITION: usize = 0usize;
    const ANNOTATIONS: ::rorm::internal::hmr::annotations::Annotations = ::rorm::internal::hmr::annotations::Annotations {
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
    const NAME: &'static str = "id";
    type Type = i64;
    const FILE: &'static str = ::core::file!();
    const LINE: u32 = ::core::line!();
    const COLUMN: u32 = ::core::column!();
}
///[`Unregistered`]'s [`Fields`](::rorm::model::Model::Fields) struct.
#[allow(non_camel_case_types)]
pub struct __Unregistered_Fields_Struct<Path: 'static> {
    ///[`Unregistered`]'s `id` field
    pub id: ::rorm::internal::field::FieldProxy<__Unregistered_id, Path>,
}
impl<Path: 'static> ::rorm::model::ConstNew for __Unregistered_Fields_Struct<Path> {
    const NEW: Self = Self {
        id: ::rorm::internal::field::FieldProxy::new(),
    };
    const REF: &'static Self = &Self::NEW;
}
const _: () = {
    impl ::rorm::new::Struct for Unregistered {}
    impl ::rorm::model::Model for Unregistered {
        type Primary = __Unregistered_id;
        type Fields<P: ::rorm::internal::relation_path::Path> = __Unregistered_Fields_Struct<
            P,
        >;
        const F: __Unregistered_Fields_Struct<Self> = ::rorm::model::ConstNew::NEW;
        const FIELDS: __Unregistered_Fields_Struct<Self> = ::rorm::model::ConstNew::NEW;
        const TABLE: &'static str = "unregistered";
        const SOURCE: ::rorm::internal::hmr::Source = ::rorm::internal::hmr::Source {
            file: ::std::file!(),
            line: ::std::line!() as usize,
            column: ::std::column!() as usize,
        };
        fn push_fields_imr(fields: &mut Vec<::rorm::imr::Field>) {
            ::rorm::internal::field::push_imr::<__Unregistered_id>(&mut *fields);
        }
    }
    use ::rorm::internal::field::decoder::FieldDecoder;
    use ::rorm::fields::traits::FieldType;
    pub struct __Unregistered_Decoder {
        id: <i64 as ::rorm::fields::traits::FieldType>::Decoder,
    }
    impl ::rorm::crud::decoder::Decoder for __Unregistered_Decoder {
        type Result = Unregistered;
        fn by_name<'index>(
            &'index self,
            row: &'_ ::rorm::db::Row,
        ) -> Result<Self::Result, ::rorm::db::row::RowError<'index>> {
            Ok(Unregistered {
                id: self.id.by_name(row)?,
            })
        }
        fn by_index<'index>(
            &'index self,
            row: &'_ ::rorm::db::Row,
        ) -> Result<Self::Result, ::rorm::db::row::RowError<'index>> {
            Ok(Unregistered {
                id: self.id.by_index(row)?,
            })
        }
    }
    impl ::rorm::model::Patch for Unregistered {
        type Model = Unregistered;
        type Decoder = __Unregistered_Decoder;
        fn select<P: ::rorm::internal::relation_path::Path>(
            ctx: &mut ::rorm::internal::query_context::QueryContext,
        ) -> Self::Decoder {
            __Unregistered_Decoder {
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
    impl<'a> ::rorm::internal::patch::IntoPatchCow<'a> for Unregistered {
        type Patch = Unregistered;
        fn into_patch_cow(self) -> ::rorm::internal::patch::PatchCow<'a, Unregistered> {
            ::rorm::internal::patch::PatchCow::Owned(self)
        }
    }
    impl<'a> ::rorm::internal::patch::IntoPatchCow<'a> for &'a Unregistered {
        type Patch = Unregistered;
        fn into_patch_cow(self) -> ::rorm::internal::patch::PatchCow<'a, Unregistered> {
            ::rorm::internal::patch::PatchCow::Borrowed(self)
        }
    }
};
impl ::rorm::model::FieldByIndex<{ 0usize }> for Unregistered {
    type Field = __Unregistered_id;
}
impl ::rorm::model::GetField<__Unregistered_id> for Unregistered {
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
