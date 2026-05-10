mod constants;

use turso::Builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Builder::new_local(constants::DB_FILENAME).build().await?;
    let conn = db.connect()?;
    println!("Connected to {}", constants::DB_FILENAME);

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL
        )",
        (),
    )
    .await?;

    let name1 = "peter";
    let name2 = "paul";

    conn.execute("INSERT INTO users (username) VALUES (?)", (name1,))
        .await?;
    conn.execute("INSERT INTO users (username) VALUES (?)", (name2,))
        .await?;

    let _res = conn.query("SELECT * FROM users", ()).await?;

    Ok(())
}
