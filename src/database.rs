use std::{env, error::Error, io::ErrorKind};

use rusqlite::Connection;

const DB_FILE: &str = "db.db";

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
    const QUERY: &str = "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, name STRING, password_hash STRING)";
    conn.execute(QUERY, [])
        .expect("could not create table users");
    const QUERY2: &str = "CREATE TABLE IF NOT EXISTS flags (id INTEGER PRIMARY KEY AUTOINCREMENT, name STRING, description STRING, points INTEGER, flag STRING)";
    conn.execute(QUERY2, [])
        .expect("could not create table flags");
    const QUERY3: &str = "CREATE TABLE IF NOT EXISTS cleared (id INTEGER PRIMARY KEY AUTOINCREMENT, uid INTEGER, fid INTEGER, FOREIGN KEY (uid) REFERENCES users(id), FOREIGN KEY (fid) REFERENCES flags(id))";
    conn.execute(QUERY3, [])
        .expect("could not create table cleared");
}

pub struct User {
    name: String,
    pub id: i32,
}

impl User {
    fn new(name: String, id: i32) -> Self {
        Self { name, id }
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn create_user(name: &String, password: &String) -> Result<User, Box<dyn Error>> {
        let conn = conn();

        const QUERY: &str = "SELECT EXISTS (SELECT * FROM users WHERE name = (?1))";

        let mut stmt = conn.prepare(QUERY)?;
        let mut user = stmt.query([&name])?;

        if user.next()?.is_some_and(|x| x.get(0).is_ok_and(|x| x)) {
            return Err(Box::new(std::io::Error::new(
                ErrorKind::AlreadyExists,
                "user already exists",
            )));
        }

        const QUERY2: &str = "INSERT INTO users (name, password_hash) VALUES (?1,?2)";
        let mut statement = conn.prepare(QUERY2)?;

        let hash = bcrypt::hash(password, 13)?;

        statement.execute([name, &hash])?;

        // explicit drop so that we can borrow again
        drop(user);
        let mut user = stmt.query([&name])?;
        let user = user
            .next()?
            .ok_or("Error getting the newly registered user, try login in")?;
        let user = User::new(user.get(1)?, user.get(0)?);

        Ok(user)
    }

    // TODO: this function is vulnerable to timing attacks that allow user enumeration. This isn't really a concern but it might still be fun to look into fixing it.
    pub fn login(username: &str, password: &str) -> Result<User, Box<dyn Error>> {
        let conn = conn();

        const QUERY: &str = "SELECT * FROM users WHERE name = (?1)";

        let mut stmt = conn.prepare(QUERY)?;
        let mut user = stmt.query([&username])?;
        let user = user.next()?.ok_or("invalid password or username")?;
        let hash: String = user.get(2)?;

        if bcrypt::verify(password, &hash)? {
            Ok(User::new(user.get(1)?, user.get(0)?))
        } else {
            Err("invalid password or username".into())
        }
    }

    pub fn points(&self) -> Result<i32, Box<dyn Error>> {
        let conn = conn();
        let QUERY: &str = "SELECT IFNULL(SUM(f.points),0) FROM flags f JOIN cleared c ON c.fid = f.id JOIN users u ON u.id = c.uid WHERE u.id = (?1)";
        let mut stmt = conn.prepare(QUERY).unwrap();
        let mut points = stmt.query([&self.id])?;
        let points = match points.next()? {
            Some(r) => r.get(0)?,
            None => 0,
        };
        Ok(points)
    }
}

pub struct Flag {
    name: String,
    description: String,
    flag: String,
    points: i32,
    id: i32,
}

impl Flag {
    pub fn get_all() -> Result<Vec<Flag>, Box<dyn Error>> {
        let conn = conn();
        const QUERY: &str = "SELECT * FROM flags";
        let mut stmt = conn.prepare(QUERY)?;
        let res = stmt.query_map([], |row| {
            Ok(Flag::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;
        let res = res.filter_map(|x| x.ok()).collect();
        Ok(res)
    }

    fn new(id: i32, name: String, description: String, points: i32, flag: String) -> Self {
        Self {
            name,
            description,
            flag,
            points,
            id,
        }
    }

    pub fn row_parts(&self) -> [&str; 3] {
        [
            &self.name,
            &self.description,
            format!("{}", self.points).leak(),
        ]
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn points(&self) -> i32 {
        self.points
    }

    pub fn flag(&self) -> &str {
        &self.flag
    }

    pub fn clear_for_user(&self, id: i32) -> Result<(), Box<dyn Error>> {
        let conn = conn();
        const QUERY: &str = "SELECT EXISTS ( SELECT * FROM cleared WHERE uid = ?1 AND fid = ?2)";

        let mut stmt = conn.prepare(QUERY)?;
        let mut entry = stmt.query([&id, &self.id])?;

        if entry.next()?.is_some_and(|x| x.get(0).is_ok_and(|x| x)) {
            return Err(Box::new(std::io::Error::new(
                ErrorKind::AlreadyExists,
                "flag already solved",
            )));
        }

        const QUERY2: &str = "INSERT INTO cleared (uid, fid) VALUES (?1,?2)";
        let mut statement = conn.prepare(QUERY2)?;

        statement.execute([&id, &self.id])?;
        Ok(())
    }
}

pub fn clear_flags() {
    let conn = conn();
    const QUERY: &str = "DROP TABLE IF EXISTS flags";
    conn.execute(QUERY, []).unwrap();
    create_missing_db();
}

pub fn create_test_flags() {
    let conn = conn();
    const QUERY: &str = "INSERT INTO flags (name, description,points, flag) VALUES (?1,?2,?3,?4)";
    let mut stmt = conn.prepare(QUERY).unwrap();
    stmt.execute(["flag0", "easy test flag", "100", "ctf{flag}"])
        .unwrap();
    stmt.execute(["flag1", "hard flag", "500", "ctf{flag_1337}"])
        .unwrap();
    stmt.execute(["flag2", "medium flag", "300", "ctf{flag_leet}"])
        .unwrap();
    stmt.execute([
        "mdflag",
        "# markdown flag\n[with links](https://github.com)",
        "300",
        "ctf{flag_md}",
    ])
    .unwrap();
    stmt.execute([
        "longflag",
        "A REALLY LONG FLAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\nAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n\n\n\n\nAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAG!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\n\n\n\n\n\n\n\n\n\n\n(very long)",
        "300",
        "ctf{flag_md}",
    ])
    .unwrap();
}
