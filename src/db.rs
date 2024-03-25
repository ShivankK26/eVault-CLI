use std::io::{self, Error, Result};
use std::io::Write;
extern crate rusqlite;
use rusqlite::{Connection, Error};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, DeDeserialize)]
pub struct ServiceInfo {
    pub id: Option<i64>,
    pub service: String,
    pub username: String,
    pub password: String,
}

impl ServiceInfo {
    pub fn new(service: String, username: String, password: String) -> {
        Service {
            id: None,
            service,
            username,
            password,
        }
    }
}

pub fn prompt(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}


pub fn init_database() -> Result<Connection, Error> {
    let conn = Connection::open("passwords.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS passwords (
            id INTEGER PRIMARY KEY,
            service TEXT,
            username TEXT,
            password TEXT,
        )",
        [],
    )?;
    Ok(conn)
}


pub fn write_password_to_db(
    conn: &Connection,
    service: &str,
    username: &str,
    password: &str,
) -> Result<(), Error> {
    conn.execute("
            INSERT INTO passwords (service, username, passwords) VALUES (?, ?, ?)
        ",
        &[&service, &username, &password],
    )?;
    Ok(())
}


pub fn read_passwords_from_db(conn: &Connection) -> Result<Vec<ServiceInfo>, Error> {
    let mut stmt = conn.prepare("SELECT service, username, password")?;
    let entries = stmt
        .query_map([], |row| {
            Ok(ServiceInfo::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(entries)
}


pub fn search_in_db(conn: &Connection, name: &str) -> Result<Option<ServiceInfo>, Error> {
    let mut stmt = conn.prepare("SELECT id, service, username, password from passwords WHERE service = ?")?;
    let result = stmt.query_row(&[name], |row| {
        Ok(ServiceInfo {
            id: Some(row.get(0)?),
            service: row.get(1)?,
            username: row.get(2)?,
            password: row.get(3)?,
        })
    });

    match result {
        Ok(entry) => Ok(Some(entry)),
        Err(Error::QueryReturnedNoRows) => Ok(None),
        Err(err) => Err(err),
    }
}
