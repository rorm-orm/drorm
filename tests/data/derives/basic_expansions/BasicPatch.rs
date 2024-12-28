const _: () = {
    use ::rorm::internal::field::decoder::FieldDecoder;
    use ::rorm::fields::traits::FieldType;
    pub struct __BasicPatch_Decoder {}
    impl ::rorm::crud::decoder::Decoder for __BasicPatch_Decoder {
        type Result = BasicPatch;
        fn by_name<'index>(
            &'index self,
            row: &'_ ::rorm::db::Row,
        ) -> Result<Self::Result, ::rorm::db::row::RowError<'index>> {
            Ok(BasicPatch {})
        }
        fn by_index<'index>(
            &'index self,
            row: &'_ ::rorm::db::Row,
        ) -> Result<Self::Result, ::rorm::db::row::RowError<'index>> {
            Ok(BasicPatch {})
        }
    }
    impl ::rorm::model::Patch for BasicPatch {
        type Model = BasicModel;
        type Decoder = __BasicPatch_Decoder;
        fn select<P: ::rorm::internal::relation_path::Path>(
            ctx: &mut ::rorm::internal::query_context::QueryContext,
        ) -> Self::Decoder {
            __BasicPatch_Decoder {}
        }
        fn push_columns(columns: &mut Vec<&'static str>) {}
        fn push_references<'a>(
            &'a self,
            values: &mut Vec<::rorm::conditions::Value<'a>>,
        ) {}
        fn push_values(self, values: &mut Vec<::rorm::conditions::Value>) {}
    }
    impl<'a> ::rorm::internal::patch::IntoPatchCow<'a> for BasicPatch {
        type Patch = BasicPatch;
        fn into_patch_cow(self) -> ::rorm::internal::patch::PatchCow<'a, BasicPatch> {
            ::rorm::internal::patch::PatchCow::Owned(self)
        }
    }
    impl<'a> ::rorm::internal::patch::IntoPatchCow<'a> for &'a BasicPatch {
        type Patch = BasicPatch;
        fn into_patch_cow(self) -> ::rorm::internal::patch::PatchCow<'a, BasicPatch> {
            ::rorm::internal::patch::PatchCow::Borrowed(self)
        }
    }
};
