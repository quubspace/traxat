use anyhow::{anyhow, Context, Result};
use rusqlite::{params, Connection};
use std::fmt::{self, Display, Formatter};

const DB: &str = "./grover.db";

#[derive(Debug)]
pub struct Position {
    id: i32,
    pub name: String,
    pub position: Vec<u8>,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Name: {:?}, Vector: {:?}", self.name, self.position)
    }
}

pub fn query_all_positions() -> Result<()> {
    let conn = Connection::open(DB).map_err(|_| anyhow!("Cannot connect to database file!"))?;

    let mut stmt = conn.prepare("SELECT id, name, position FROM position")?;
    let position_iter = stmt.query_map(params![], |row| {
        Ok(Position {
            id: row.get(0)?,
            name: row.get(1)?,
            position: row.get(2)?,
        })
    })?;

    for position in position_iter {
        println!("{}", position.context("Error selecting positions.")?);
    }

    Ok(())
}

pub fn query_position(name: &str) -> Result<Position> {
    let conn = Connection::open(DB).map_err(|_| anyhow!("Cannot connect to database file!"))?;

    conn.query_row(
        "SELECT id, name, position FROM position WHERE LOWER(name)=?",
        params![name],
        {
            |row| {
                Ok(Position {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    position: row.get(2)?,
                })
            }
        },
    )
    .context("Position could not be found")
}

pub fn add_position(name: &str, position: Vec<u8>) -> Result<()> {
    let conn = Connection::open(DB).map_err(|_| anyhow!("Cannot connect to database file!"))?;

    conn.execute(
        "INSERT INTO position (name, position) values (?1, ?2)",
        params![name.to_lowercase(), position],
    )
    .map_err(|_| {
        anyhow!("Please ensure this position does not already exist, unique constraint failed!")
    })?;

    println!(
        "Successfully added position {} with vector {:?}!",
        name, position
    );

    Ok(())
}

pub fn remove_position(name: &str) -> Result<()> {
    let conn = Connection::open(DB).map_err(|_| anyhow!("Cannot connect to database file!"))?;

    conn.execute(
        "DELETE FROM position WHERE name=?",
        params![name.to_lowercase()],
    )
    .map_err(|_| {
        anyhow!("Please ensure this position exists before removing it, deletion failed!")
    })?;

    println!("If present, position {} was successfully removed!", name);

    Ok(())
}

pub fn migrations() -> Result<()> {
    let conn = Connection::open(DB)?;

    let stmt = conn.prepare("SELECT id, name, position FROM position LIMIT 1");
    if stmt.is_err() {
        let mut conn = Connection::open(DB)?;
        default_migration(&mut conn)?;
    }

    Ok(())
}

fn default_migration(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute(
        "CREATE TABLE IF NOT EXISTS position (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL UNIQUE,
                  position            BLOB
                  )",
        params![],
    )?;
    let top = Position {
        id: 0,
        name: "top".to_string(),
        position: vec![1, 2, 3],
    };
    let bottom = Position {
        id: 1,
        name: "bottom".to_string(),
        position: vec![1, 2, 3],
    };
    let forward = Position {
        id: 2,
        name: "forward".to_string(),
        position: vec![1, 2, 3],
    };
    tx.execute(
        "INSERT INTO position (name, position) VALUES (?1, ?2), (?3, ?4), (?5, ?6)",
        params![
            top.name,
            top.position,
            bottom.name,
            bottom.position,
            forward.name,
            forward.position
        ],
    )?;

    tx.commit().context("Error committing to database.")
}
