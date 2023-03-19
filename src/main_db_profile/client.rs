use sqlx::{
    query,
    sqlite::{Sqlite, SqliteConnectOptions},
    ConnectOptions, Error as SQL_Error, SqlitePool,
};
use std::path::Path;

async fn create_main_db_if_not_exists(db_path: &str) {
    if Path::new(db_path).exists() {
        return ();
    } else {
        match SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(true)
            .connect()
            .await
        {
            Ok(_) => {
                create_migration(db_path).await;
                return ();
            }
            Err(err) => panic!("[-] Error While Createing {} : {}", db_path, err),
        };
    }
}

pub async fn get_main_pool_client(db_path: &str) -> SqlitePool {
    create_main_db_if_not_exists(db_path).await;

    let db_url: String = format!("sqlite:{}", db_path);

    match SqlitePool::connect(db_url.as_str()).await {
        Ok(client) => client,
        Err(err) => panic!("[-] Error While Connecting {} : {}", db_url, err),
    }
}

async fn create_migration(db_path: &str) {
    let migrate_query: &str = r"
    
    CREATE TABLE Departments(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        department_name VARCHAR NOT NULL,
        description TEXT NOT NULL
    );

    CREATE TABLE Employees(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        first_name VARCHAR(50),
        last_name VARCHAR(50),
        email VARCHAR(50) NOT NULL CHECK( email LIKE '%@%.%'),
        salary INTEGER NOT NULL DEFAULT(0),
        department_id INTEGER NOT NULL,
        FOREIGN KEY(department_id) REFERENCES Departments(id)
    );
            ";

    let db_url: String = format!("sqlite:{}", db_path);
    let pool_client = SqlitePool::connect(db_url.as_str())
        .await
        .expect("[-] Error While Connecting ");
    query(migrate_query)
        .execute(&pool_client)
        .await
        .expect("[-] Error While Migeration");
}
