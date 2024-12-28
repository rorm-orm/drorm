const _: () = {
    use ::rorm::internal::field::decoder::FieldDecoder;
    use ::rorm::fields::traits::FieldType;
    pub struct __TestPatch_Decoder {
        id: <i64 as ::rorm::fields::traits::FieldType>::Decoder,
    }
    impl ::rorm::crud::decoder::Decoder for __TestPatch_Decoder {
        type Result = TestPatch;
        fn by_name<'index>(
            &'index self,
            row: &'_ ::rorm::db::Row,
        ) -> Result<Self::Result, ::rorm::db::row::RowError<'index>> {
            Ok(TestPatch {
                id: self.id.by_name(row)?,
            })
        }
        fn by_index<'index>(
            &'index self,
            row: &'_ ::rorm::db::Row,
        ) -> Result<Self::Result, ::rorm::db::row::RowError<'index>> {
            Ok(TestPatch {
                id: self.id.by_index(row)?,
            })
        }
    }
    impl ::rorm::model::Patch for TestPatch {
        type Model = TestModel;
        type Decoder = __TestPatch_Decoder;
        fn select<P: ::rorm::internal::relation_path::Path>(
            ctx: &mut ::rorm::internal::query_context::QueryContext,
        ) -> Self::Decoder {
            __TestPatch_Decoder {
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
    impl<'a> ::rorm::internal::patch::IntoPatchCow<'a> for TestPatch {
        type Patch = TestPatch;
        fn into_patch_cow(self) -> ::rorm::internal::patch::PatchCow<'a, TestPatch> {
            ::rorm::internal::patch::PatchCow::Owned(self)
        }
    }
    impl<'a> ::rorm::internal::patch::IntoPatchCow<'a> for &'a TestPatch {
        type Patch = TestPatch;
        fn into_patch_cow(self) -> ::rorm::internal::patch::PatchCow<'a, TestPatch> {
            ::rorm::internal::patch::PatchCow::Borrowed(self)
        }
    }
    impl ::rorm::model::GetField<::rorm::get_field!(TestPatch, id)> for TestPatch {
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
};
