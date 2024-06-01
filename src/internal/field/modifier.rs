use fancy_const::{const_fn, Contains};

use crate::internal::const_concat::ConstString;
use crate::internal::hmr::annotations::Annotations;

const_fn! {
    pub fn merge_annotations<Implicit: Contains<Annotations>>(field: Annotations) -> [Annotations; 1] {
        match field.merge(Implicit::ITEM) {
            Ok(annotations) => [annotations],
            Err(duplicate) => {
                let error = ConstString::error(&[
                    "The annotation ",
                    duplicate,
                    " is implied by its field's type and can't be set explicitly",
                ]);
                panic!("{}", error.as_str());
            }
        }
    }
}

const_fn! {
    pub fn forward_annotations<const N: usize>(field: Annotations) -> [Annotations; N] {
        [field; N]
    }
}

const_fn! {
    pub fn set_null_annotations<const N: usize>(field: Annotations) -> [Annotations; N] {
        let mut field = field;
        field.nullable = true;
        [field; N]
    }
}

const_fn! {
    pub fn no_check<const N: usize>(_field: Annotations, _columns: [Annotations; N]) -> Result<(), ConstString<1024>> {
        Ok(())
    }
}

const_fn! {
    pub fn shared_linter_check<const N: usize>(_field: Annotations, columns: [Annotations; N]) -> Result<(), ConstString<1024>> {
        let mut columns = columns.as_slice();
        while let [column, tail @ ..] = columns {
            columns = tail;

            if let Err(err) = column.as_lint().check() {
                return Err(ConstString::error(&["invalid annotations: ", err]));
            }
        }
        Ok(())
    }
}

const_fn! {
    pub fn string_check(_field: Annotations, [column]: [Annotations; 1]) -> Result<(), ConstString<1024>> {
        if let Err(error) = shared_linter_check(_field, [column]) {
            return Err(error);
        }

        if column.max_length.is_none() {
            return Err(ConstString::error(&[
                "missing annotation: max_length",
            ]));
        }

        Ok(())
    }
}

const_fn! {
    pub fn no_columns_names(_field_name: &'static str) -> [&'static str; 0] {
        []
    }
}

const_fn! {
    pub fn single_column_name(field_name: &'static str) -> [&'static str; 1] {
        [field_name]
    }
}
