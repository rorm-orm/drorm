use std::fmt::Write;

use crate::error::Error;
use crate::{value, SQLCreateColumn};

/**
Representation of operations to execute in the context of an ALTER TABLE statement.
*/
pub enum SQLAlterTableOperation<'post_build> {
    /// Use this operation to rename a table
    RenameTo {
        /// New name of the table
        name: String,
    },
    /// Use this operation to rename a column within a table
    RenameColumnTo {
        /// Current column name
        column_name: String,
        /// New column name
        new_column_name: String,
    },
    /// Use this operation to add a column to an existing table.
    /// Can be generated by using [crate::create_column::SQLCreateColumn]
    AddColumn {
        /// Operation to use for adding the column
        operation: SQLCreateColumn<'post_build>,
    },
    /// Use this operation to drop an existing column.
    DropColumn {
        /// Name of the column to drop
        name: String,
    },
}

impl<'post_build> SQLAlterTableOperation<'post_build> {
    fn build(
        self,
        s: &mut String,
        trigger: &mut Vec<(String, Vec<value::Value<'post_build>>)>,
    ) -> Result<(), Error> {
        match self {
            SQLAlterTableOperation::RenameTo { name } => write!(s, "RENAME TO {}", name).unwrap(),
            SQLAlterTableOperation::RenameColumnTo {
                column_name,
                new_column_name,
            } => write!(s, "RENAME COLUMN {} TO {}", column_name, new_column_name).unwrap(),
            SQLAlterTableOperation::AddColumn { operation } => {
                write!(s, "ADD COLUMN ").unwrap();
                operation.build(s, trigger)?;
            }
            SQLAlterTableOperation::DropColumn { name } => {
                write!(s, "DROP COLUMN {}", name).unwrap();
            }
        };
        Ok(())
    }
}

/**
Representation of an ALTER TABLE statement.
*/
pub struct SQLAlterTable<'post_build> {
    /// Name of the table to operate on
    pub(crate) name: String,
    /// Operation to execute
    pub(crate) operation: SQLAlterTableOperation<'post_build>,
    pub(crate) lookup: Vec<value::Value<'post_build>>,
    pub(crate) trigger: Vec<(String, Vec<value::Value<'post_build>>)>,
}

impl<'post_build> SQLAlterTable<'post_build> {
    /**
    This method is used to build the alter table statement.
    */
    pub fn build(mut self) -> Result<(String, Vec<value::Value<'post_build>>), Error> {
        let mut s = format!("ALTER TABLE {} ", self.name.as_str());
        self.operation.build(&mut s, &mut self.trigger)?;

        self.trigger
            .into_iter()
            .map(|(trigger, bind_params)| {
                self.lookup.extend(bind_params);
                trigger
            })
            .collect::<Vec<String>>()
            .join(" ");

        Ok((s, self.lookup))
    }
}
