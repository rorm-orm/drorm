const _: () = {
    const CHOICES: &'static [&'static str] = &[
        stringify!(Foo),
        stringify!(Bar),
        stringify!(Baz),
    ];
    impl ::rorm::fields::traits::FieldType for Test {
        type Columns = ::rorm::fields::traits::Array<1>;
        const NULL: ::rorm::fields::traits::FieldColumns<
            Self,
            ::rorm::db::sql::value::NullType,
        > = [::rorm::db::sql::value::NullType::String];
        fn into_values<'a>(
            self,
        ) -> ::rorm::fields::traits::FieldColumns<Self, ::rorm::conditions::Value<'a>> {
            [
                ::rorm::conditions::Value::Choice(
                    ::std::borrow::Cow::Borrowed(
                        match self {
                            Self::Foo => stringify!(Foo),
                            Self::Bar => stringify!(Bar),
                            Self::Baz => stringify!(Baz),
                        },
                    ),
                ),
            ]
        }
        fn as_values(
            &self,
        ) -> ::rorm::fields::traits::FieldColumns<Self, ::rorm::conditions::Value<'_>> {
            [
                ::rorm::conditions::Value::Choice(
                    ::std::borrow::Cow::Borrowed(
                        match self {
                            Self::Foo => stringify!(Foo),
                            Self::Bar => stringify!(Bar),
                            Self::Baz => stringify!(Baz),
                        },
                    ),
                ),
            ]
        }
        type Decoder = __Test_Decoder;
        type GetAnnotations = get_db_enum_annotations;
        type Check = ::rorm::fields::utils::check::shared_linter_check<1>;
        type GetNames = ::rorm::fields::utils::get_names::single_column_name;
    }
    ::rorm::new_converting_decoder!(
        #[doc(hidden)] __Test_Decoder, | value : ::rorm::db::choice::Choice | -> Test {
        let value : String = value.0; match value.as_str() { stringify!(Foo) =>
        Ok(Test::Foo), stringify!(Bar) => Ok(Test::Bar), stringify!(Baz) =>
        Ok(Test::Baz), _ => Err(format!("Invalid value '{}' for enum '{}'", value,
        stringify!(Test))), } }
    );
    ::rorm::impl_FieldEq!(
        impl < 'rhs > FieldEq < 'rhs, Test > for Test { | value : Test | { let [value] =
        < Test as ::rorm::fields::traits::FieldType > ::into_values(value); value } }
    );
    ::rorm::const_fn! {
        pub fn get_db_enum_annotations(field :
        ::rorm::internal::hmr::annotations::Annotations) ->
        [::rorm::internal::hmr::annotations::Annotations; 1] { let mut field = field;
        field.choices = Some(::rorm::internal::hmr::annotations::Choices(CHOICES));
        [field] }
    }
};
