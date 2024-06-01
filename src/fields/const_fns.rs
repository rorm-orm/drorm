//! [`ConstFn`](fancy_const::ConstFn)s used to implement [`FieldType`]
//!
//! These functions represent some common implementations of `FieldType`'s "const methods".
//! Feel free to select from them or compose them when implementing `FieldType` yourself.

use fancy_const::{const_fn, Contains};

#[cfg(doc)]
use crate::fields::traits::FieldType;
use crate::internal::const_concat::ConstString;
use crate::internal::hmr::annotations::Annotations;

const_fn! {
    /// [`FieldType::GetAnnotations`] which merges the field's explicit annotations
    /// with a set of implicit ones provided by `Implicit`.
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
    /// [`FieldType::GetAnnotations`] which forwards the field's explicit annotations to every column.
    pub fn forward_annotations<const N: usize>(field: Annotations) -> [Annotations; N] {
        [field; N]
    }
}

const_fn! {
    /// [`FieldType::GetAnnotations`] which adds `nullable` to the explicit annotations.
    pub fn set_null_annotations(field: Annotations) -> [Annotations; 1] {
        let mut field = field;
        field.nullable = true;
        [field]
    }
}

const_fn! {
    /// [`FieldType::Check`] which checks the explicit annotations to be empty.
    pub fn disallow_annotations_check<const N: usize>(field: Annotations, _columns: [Annotations; N]) -> Result<(), ConstString<1024>> {
        match field {
            Annotations {
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
            } => Ok(()),
            _ => Err(ConstString::error(&["BackRef doesn't take any annotations"])),
        }
    }
}

const_fn! {
    /// [`FieldType::Check`] which runs the linter shared with `rorm-cli` on every column.
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
    /// [`FieldType::Check`] which runs the linter shared with `rorm-cli` on every column
    /// and checks `max_length` to be set.
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
    /// [`FieldType::GetNames`] for fields without columns
    pub fn no_columns_names(_field_name: &'static str) -> [&'static str; 0] {
        []
    }
}

const_fn! {
    /// [`FieldType::GetNames`] for fields with a single column
    pub fn single_column_name(field_name: &'static str) -> [&'static str; 1] {
        [field_name]
    }
}
