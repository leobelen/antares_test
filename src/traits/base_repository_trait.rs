use crate::db::DbConnection;
use diesel::dsl;
use diesel::insertable::CanInsertInSingleQuery;
use diesel::prelude::*;
use diesel::query_builder::{
    AsChangeset, AsQuery, InsertStatement, IntoUpdateTarget, QueryFragment, QueryId,
    UndecoratedInsertRecord, UpdateStatement,
};
use diesel::query_dsl::methods::{FindDsl, LimitDsl, LoadQuery};
use diesel::query_dsl::RunQueryDsl;
use diesel::sql_types::HasSqlType;

pub trait BaseRepositoryTrait<TABLE, MODEL>
where
    TABLE: diesel::Table + diesel::associations::HasTable<Table = TABLE>,
{
    /// Gets the role by ID or none if none found
    /// # Arguments
    /// * `connection` the db connection object.
    /// * `obj_id` the id of the object to find.
    /// # Returns
    /// An optional object if found
    fn find_by_id(
        connection: &DbConnection,
        id: TABLE::PrimaryKey,
    ) -> Result<MODEL, diesel::result::Error>
    where
        TABLE: diesel::query_dsl::methods::FindDsl<<TABLE as diesel::Table>::PrimaryKey>,
        dsl::Find<TABLE, TABLE::PrimaryKey>: RunQueryDsl<DbConnection> + LimitDsl,
        dsl::Limit<dsl::Find<TABLE, TABLE::PrimaryKey>>: LoadQuery<DbConnection, MODEL>,
    {
        TABLE::table().find(id).first(connection)
    }

    /// Returns all objects in the table.
    /// # Arguments
    /// * `connection` the db connection object.
    /// # Returns
    /// The list of all objects
    fn find_all(connection: &DbConnection) -> Result<Vec<MODEL>, diesel::result::Error>
    where
        TABLE: diesel::query_dsl::methods::FindDsl<<TABLE as diesel::Table>::PrimaryKey>,
        dsl::Find<TABLE, TABLE::PrimaryKey>: RunQueryDsl<DbConnection> + LimitDsl,
        dsl::Limit<dsl::Find<TABLE, TABLE::PrimaryKey>>: LoadQuery<DbConnection, MODEL>,
        diesel::pg::Pg: HasSqlType<<TABLE as AsQuery>::SqlType>,
        <TABLE as AsQuery>::Query: QueryFragment<<DbConnection as Connection>::Backend>,
        <TABLE as AsQuery>::Query: QueryId,
        MODEL: Queryable<<TABLE as AsQuery>::SqlType, <DbConnection as Connection>::Backend>,
    {
        TABLE::table().load::<MODEL>(connection)
    }

    /// Inserts an object into the database table.
    /// # Arguments
    /// * `connection` the db connection object.
    /// * `object` the object to insert or update.
    /// # Returns
    /// Ok() or error
    fn insert<'a>(connection: &DbConnection, data: &'a MODEL) -> Result<(), diesel::result::Error>
    where
        &'a MODEL: Insertable<TABLE> + UndecoratedInsertRecord<TABLE>,
        <&'a MODEL as Insertable<TABLE>>::Values: QueryFragment<<DbConnection as Connection>::Backend>
            + CanInsertInSingleQuery<<DbConnection as Connection>::Backend>,
        TABLE::FromClause: QueryFragment<<DbConnection as Connection>::Backend>,
        InsertStatement<TABLE, <&'a MODEL as Insertable<TABLE>>::Values>: RunQueryDsl<DbConnection>,
    {
        diesel::insert_into(TABLE::table())
            .values(data)
            .execute(connection)?;
        Ok(())
    }

    /// Deletes an object into the database table.
    /// # Arguments
    /// * `connection` the db connection object.
    /// * `obj_id` the object id delete.
    /// # Returns
    /// The count of objects deleted.
    fn delete_by_id<'a>(
        connection: &DbConnection,
        id: &'a TABLE::PrimaryKey,
    ) -> Result<(), diesel::result::Error>
    where
        TABLE: diesel::query_dsl::methods::FindDsl<&'a <TABLE as diesel::Table>::PrimaryKey>,
        dsl::Find<TABLE, &'a TABLE::PrimaryKey>: IntoUpdateTarget<Table = TABLE>,
        TABLE::FromClause: QueryFragment<<DbConnection as Connection>::Backend>,
        <dsl::Find<TABLE, &'a TABLE::PrimaryKey> as IntoUpdateTarget>::WhereClause:
            QueryFragment<<DbConnection as Connection>::Backend> + QueryId,
        TABLE: QueryId,
    {
        diesel::delete(TABLE::table().find(id)).execute(connection)?;
        Ok(())
    }
}

pub trait BaseUpdateRepositoryTrait<TABLE, MODEL>
where
    TABLE:
        diesel::Table + diesel::associations::HasTable<Table = TABLE> + IntoUpdateTarget + QueryId,
{
    /// Inserts or updates an object into the database table.
    /// # Arguments
    /// * `connection` the db connection object.
    /// * `object` the object to insert or update.
    /// # Returns
    /// The count of objects inserted.
    fn update<'a>(connection: &DbConnection, object: &'a MODEL) -> Result<(), diesel::result::Error>
    where
        <TABLE as QuerySource>::FromClause: QueryFragment<<DbConnection as Connection>::Backend>,
        <TABLE as diesel::Table>::PrimaryKey: Column,
        <TABLE::Table as diesel::QuerySource>::FromClause:
            diesel::query_builder::QueryFragment<<DbConnection as Connection>::Backend>,
        &'a MODEL: diesel::Insertable<TABLE::Table> + AsChangeset,
        <&'a MODEL as diesel::Insertable<TABLE>>::Values: UndecoratedInsertRecord<TABLE>,
        &'a MODEL: diesel::Insertable<TABLE>,
    {
        diesel::insert_into(TABLE::table())
            .values(object)
            .on_conflict(TABLE::table().primary_key())
            .do_update()
            .set(object)
            .execute(connection)
    }
}
