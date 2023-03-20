use super::super::client::get_main_pool_client;
use sqlx::{
    query, query_as,
    sqlite::{Sqlite, SqlitePool, SqliteQueryResult},
    Error as SqliteError, FromRow,
};
use std::env::var;

#[derive(FromRow, Debug)]
pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub salary: i32,
    pub department_id: i32,
}

impl Employee {
    pub async fn add_employee(
        first_name: &str,
        last_name: &str,
        email: &str,
        salary: i32,
        department_id: i32,
    ) -> Result<(), SqliteError> {
        let pool_client: SqlitePool =
            get_main_pool_client(var("SQLITE_MAIN_DB").unwrap().as_str()).await;
        let sql_query: &str = r"INSERT INTO Employees(first_name,last_name,email,salary,department_id) VALUES(?,?,?,?,?);";
        let result: Result<SqliteQueryResult, SqliteError> = query::<Sqlite>(sql_query)
            .bind(first_name)
            .bind(last_name)
            .bind(email)
            .bind(salary)
            .bind(department_id)
            .execute(&pool_client)
            .await;
        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(err),
        };
    }

    pub async fn get_all_employees() -> Result<Vec<Self>, SqliteError> {
        let pool_client: SqlitePool =
            get_main_pool_client(var("SQLITE_MAIN_DB").unwrap().as_str()).await;
        let sql_query: &str = r"SELECT * FROM Employees; ";
        let result: Result<Vec<Self>, SqliteError> = query_as::<Sqlite, Self>(sql_query)
            .fetch_all(&pool_client)
            .await;
        return result;
    }

    pub async fn get_employee_by_id(id: i32) -> Result<Option<Self>, SqliteError> {
        let pool_client: SqlitePool =
            get_main_pool_client(var("SQLITE_MAIN_DB").unwrap().as_str()).await;
        let sql_query = r"SELECT * FROM Employees WHERE id = ? ; ";
        let result: Result<Option<Self>, SqliteError> = query_as::<Sqlite, Self>(sql_query)
            .bind(id)
            .fetch_optional(&pool_client)
            .await;
        return result;
    }

    pub async fn delete_employee_id(id: i32) -> Result<(), SqliteError> {
        let pool_client: SqlitePool =
            get_main_pool_client(var("SQLITE_MAIN_DB").unwrap().as_str()).await;
        let sql_query: &str = r"DELETE FROM Employees WHERE id = ?; ";
        let result = query::<Sqlite>(sql_query)
            .bind(id)
            .execute(&pool_client)
            .await;
        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(err),
        };
    }

    pub async fn update_employee_by_id(
        id: i32,
        first_name: Option<&str>,
        last_name: Option<&str>,
        email: Option<&str>,
        salary: Option<i32>,
        department_id: Option<i32>,
    ) -> Result<(), SqliteError> {
        let employee: Self;
        match Self::get_employee_by_id(id).await {
            Ok(result) => match result {
                Some(employee_info) => employee = employee_info,
                None => return Err(SqliteError::RowNotFound),
            },
            Err(err) => return Err(err),
        };

        let first_name: &str = first_name.unwrap_or(&employee.first_name.as_str());
        let last_name: &str = last_name.unwrap_or(&employee.last_name.as_str());
        let email: &str = email.unwrap_or(&employee.email.as_str());
        let salary: i32 = salary.unwrap_or(employee.salary);
        let department_id: i32 = department_id.unwrap_or(employee.department_id);

        let pool_client: SqlitePool =
            get_main_pool_client(var("SQLITE_MAIN_DB").unwrap().as_str()).await;

        let sql_query: &str = r"UPDATE Employees SET first_name=? , last_name=? , email=? , salary=?, department_id=?  WHERE id = ? ;";
        let result: Result<SqliteQueryResult, SqliteError> =
            query::<Sqlite>(sql_query).bind(first_name).bind(last_name).bind(email).bind(salary).bind(department_id).bind(id).execute(&pool_client).await;
        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(err),
        };
    }
}
