use tokio_postgres::{Error, NoTls};

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(
        format!(
            "host={} user={} password={} dbname={} port={}",
            "127.0.0.1", "postgres", "postgres", "postgres", "5432"
        )
        .as_str(),
        NoTls,
    )
    .await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    // let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await?;

    // And then check that we got back the same string we sent over.
    // let value: &str = rows[0].get(0);
    // dbg!(value);

    let students = client.query("SELECT * FROM students", &[]).await?;

    for row in students.iter() {
        // Assume the "student" table has columns named "id" and "name"
        let roll_number: i32 = row.get("roll number");
        let name: String = row.get("name");

        println!("Roll Number: {}, name: {}", roll_number, name);
    }

    Ok(())
}
