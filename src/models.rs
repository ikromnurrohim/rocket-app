use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::schema::rustaceans;

/*
in order to return data to json from the database,
so we need make serialize the data that from database to our rocket to be serialize to format json.

in order we using query builder which the return type of tuple or array, we also
need to derive QueryAble
When using the query builder, the return type can be a tuple of the values, or a struct which implements Queryable.
 */
#[derive(Serialize, Queryable)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = rustaceans)]
/*
diesel will guest the table name by struct name,
since our struct name is NewRustacean and diesel make this to snak_case which has like thi new_rustacean,
but then we don't have any table with name new_rustacean so it will be error.
to resolve that we need to tell diesel which table we want to.
 */
pub struct NewRustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
}