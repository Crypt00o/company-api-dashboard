## Simple Project Simulate Company Managament through Cli 

- Created with rust and useing sqlite3 database as Company datastore , You Don,t Need to install `sqlite3` to run this cli app 
- I Implement My Knowledge in rust through simple project
- if you want to specify another 'database name and path' just set in `.env` file the path only to database with name and assign it to `SQLITE_MAIN_DB`
like this : 
```shell
SQLITE_MAIN_DB=/path/to/new_db.db
```
- by default db will be in the current dir you run app on and it,s name will be `main.db`
