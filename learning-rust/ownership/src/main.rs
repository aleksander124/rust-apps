use rusqlite::{Connection, Error};

struct DatabaseConnection{
    connection: Connection,
}

impl DatabaseConnection{
    fn new(db_name: &str) -> Result<DatabaseConnection, Error>{
        let connection = Connection::open(db_name)?;
        Ok(DatabaseConnection { connection })
    }
}

fn main() -> Result<(), Error> {
    let connection = DatabaseConnection::new("example.db")?;
    // Perform database operations
    Ok(())
}
