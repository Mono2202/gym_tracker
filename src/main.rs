use rusqlite::Connection;
use anyhow::Result;
mod db;

fn main() -> Result<()> {
    let conn = Connection::open("./db/gym_tracker.db")?;
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS workouts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL,
            exercise TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS sets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            workout_id INTEGER NOT NULL,
            reps INTEGER NOT NULL,
            weight REAL NOT NULL,
            FOREIGN KEY(workout_id) REFERENCES workouts(id)
        );
        ")?;
    Ok(())
}
