use std::{env, error::Error, io::ErrorKind};

use rusqlite::Connection;

const DB_FILE: &'static str = "db.db";

fn conn() -> Connection {
    let mut path = env::home_dir().expect("no home dir found");
    path.push(".shacker");
    path.push(DB_FILE);
    Connection::open(path).expect("couldn't open db")
}

pub fn create_missing_db() {
    let mut path = env::home_dir().expect("no home dir found");
    path.push(".shacker");

    std::fs::create_dir_all(&path).expect("failed to create db file");

    path.push(DB_FILE);
    match std::fs::File::create_new(path) {
        Ok(_) => (),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => (),
        Err(_) => panic!("failed to create db file"),
    };
    let conn = conn();
    const QUERY: &'static str = "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, name STRING, password_hash STRING)";
    conn.execute(QUERY, [])
        .expect("could not create table users");
}

pub fn create_user(name: &String, password: &String) -> Result<(), Box<dyn Error>> {
    let conn = conn();

    const QUERY: &'static str = "SELECT * FROM users WHERE name = (?1)";

    let mut stmt = conn.prepare(QUERY)?;
    let mut user = stmt.query([&name])?;

    if user.next()?.is_some() {
        return Err(Box::new(std::io::Error::new(
            ErrorKind::AlreadyExists,
            "user already exists",
        )));
    }

    const QUERY2: &'static str = "INSERT INTO users (name, password_hash) VALUES (?1,?2)";
    let mut statement = conn.prepare(QUERY2)?;

    let hash = bcrypt::hash(password, 13)?;

    statement.execute([name, &hash])?;

    Ok(())
}

pub fn validate_login(username: &str, password: &str) -> Result<bool, Box<dyn Error>> {
    let conn = conn();

    const QUERY: &'static str = "SELECT * FROM users WHERE name = (?1)";

    let mut stmt = conn.prepare(QUERY)?;
    let mut user = stmt.query([&username])?;
    let user = user.next()?.ok_or("invalid password or username")?;
    let hash: String = user.get(2)?;

    Ok(bcrypt::verify(password, &hash).map_err(|_| "invalid password or username")?)
}
