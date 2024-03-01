pub mod person;
pub mod finance;

use diesel::prelude::*;
use diesel::{
    associations::HasTable,
    query_dsl::{
        LoadQuery,
        methods::{
            FindDsl,
//             ExecuteDsl
        },
    },
//     Insertable,
    helper_types::{
        Find,
//         Filter
    },
//     // Queryable,
    query_builder::{
        InsertStatement,
//         DeleteStatement,
//         IntoUpdateTarget, AsQuery,
//         AsChangeset,
//         QueryFragment,
    },
//     query_source::Table,
//     Identifiable,
//     QuerySource,
};
use crate::error::AppError;

// pub fn fetch_all_items<'a, T, FullStruct>(
//     conn: &mut PgConnection,
//     table: T
//     ) -> Result<Vec<FullStruct>, AppError>
//     where
//         T: Table + AsQuery,
//         T::Query: LoadQuery<'a, PgConnection, FullStruct>,
// {
//     let results = table
//         .load::<FullStruct>(conn)
//         .map_err(AppError::from)?;
//
//     Ok(results)
// }

// Help from: https://stackoverflow.com/questions/48487871/generic-function-using-diesel-causes-overflow
pub fn fetch_item_by_pk<'a, T, FullStruct>(
    conn: &mut PgConnection,
    table: T,
    pk: i32
    ) -> Result<FullStruct, AppError>
    where
        T: FindDsl<i32>,
        Find<T, i32>: LoadQuery<'a, PgConnection, FullStruct>,
{
    let results = table
        .find(pk)
        .get_result::<FullStruct>(conn)
        .map_err(AppError::from)?;

    Ok(results)
}

// Help from: https://www.reddit.com/r/rust/comments/afkuko/porting_go_to_rust_how_to_implement_a_generic/
pub fn insert_into_table<'a, T, InsertStruct, FullStruct, Values>(
    conn: &mut PgConnection,
    table: T,
    new_record: InsertStruct,
) -> Result<FullStruct, AppError>
    where
        T: diesel::QuerySource + diesel::Table,
        InsertStruct: Insertable<T, Values=Values>,
        InsertStatement<T, Values>: LoadQuery<'a, PgConnection, FullStruct>,
{
    let result = new_record.insert_into(table)
        .get_result::<FullStruct>(conn)
        .map_err(AppError::from)?;

    Ok(result)
}

// pub fn delete_item_by_pk<'a, T, Pk>(conn: &mut PgConnection, table: T, pk: Pk) -> Result<usize, AppError>
// where
//     T: Table + HasTable + FindDsl<Pk>,
//     // Delete<T, Find<T, Pk>>: LoadQuery<'a, PgConnection, usize>,
// {
//     let target = table.find(pk);
//     let num_deleted = diesel::delete(target)
//         .execute(conn)
//         .map_err(AppError::from)?;
//
//     Ok(num_deleted)
// }
