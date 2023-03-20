use super::super::client::get_main_pool_client;
use sqlx::{
    query, query_as, sqlite::SqliteQueryResult, Error as SqliteError, FromRow, Sqlite, SqlitePool,
};
use std::env::var;

#[derive(FromRow, Debug)]
pub struct Department {
    pub id: i32,
    pub department_name: String,
    pub description: String,
}

impl Department {
    pub async fn add_department(
        department_name: &str,
        description: &str,
    ) -> Result<(), SqliteError> {
        let pool_client: SqlitePool =
            get_main_pool_client(var("SQLITE_MAIN_DB").unwrap().as_str()).await;
        let sql_query: &str = r"INSERT INTO Departments(department_name,description) VALUES(?,?);";
        let result: Result<_, SqliteError> = query::<Sqlite>(sql_query)
            .bind(department_name)
            .bind(description)
            .execute(&pool_client)
            .await;
        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(err),
        };
    }

    pub async fn get_all_departments() -> Result<Vec<Self>, SqliteError> {
        let pool_client: SqlitePool =
            get_main_pool_client(var("SQLITE_MAIN_DB").unwrap().as_str()).await;
        let sql_query: &str = r"SELECT * FROM Departments;";
        let result: Result<Vec<Self>, SqliteError> = query_as::<Sqlite, Self>(sql_query)
            .fetch_all(&pool_client)
            .await;
        return result;
    }

    pub async fn get_department_by_id(id: i32) -> Result<Option<Self>, SqliteError> {
        let pool_client: SqlitePool =
            get_main_pool_client(var("SQLITE_MAIN_DB").unwrap().as_str()).await;
        let sql_query: &str = r"SELECT * FROM Departments WHERE id = ?;";
        let result: Result<Option<Self>, SqliteError> = query_as::<Sqlite, Self>(sql_query)
            .bind(id)
            .fetch_optional(&pool_client)
            .await;
        return result;
    }

    pub async fn delete_department_by_id(id: i32) -> Result<(), SqliteError> {
        let pool_client: SqlitePool =
            get_main_pool_client(var("SQLITE_MAIN_DB").unwrap().as_str()).await;
        let sql_query: &str = r"DELETE FROM Departments WHERE id = ? ;";
        let result: Result<SqliteQueryResult, SqliteError> =
            query(sql_query).bind(id).execute(&pool_client).await;
        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(err),
        };
    }

    pub async fn update_department_by_id(
        id: i32,
        new_department_name: Option<&str>,
        new_description: Option<&str>,
    ) -> Result<(), SqliteError> {
        let department: Self;
        match Self::get_department_by_id(id).await {
            Ok(result) => {
                match result {
                    Some(department_info) => department = department_info,
                    None => return Err(SqliteError::RowNotFound),
                };
            }
            Err(err) => return Err(err),
        };

        let department_name: &str =
            new_department_name.unwrap_or(&department.department_name.as_str());
        let description: &str = new_description.unwrap_or(&department.description.as_str());

        let pool_client: SqlitePool =
            get_main_pool_client(var("SQLITE_MAIN_DB").unwrap().as_str()).await;
        let sql_query: &str =
            r"UPDATE Departments SET department_name=?,description=? WHERE id = ? ;";
        let result: Result<SqliteQueryResult, SqliteError> = query(sql_query)
            .bind(department_name)
            .bind(description)
            .bind(id)
            .execute(&pool_client)
            .await;
        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(err),
        };
    }
}
