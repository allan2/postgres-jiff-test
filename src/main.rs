use jiff::{
    civil::{Date, DateTime, Time},
    Timestamp, Zoned,
};
use postgres::{Config, NoTls};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();
    config.host("/var/run/postgresql");
    let mut client = config.connect(NoTls)?;

    let row = client.query_one(
        "SELECT NOW()::timestamp, NOW(), CURRENT_DATE, CURRENT_TIME::time",
        &[],
    )?;

    // timestamp
    let datetime: DateTime = row.get(0);

    // timestamptz
    let ts: Timestamp = row.get(1);
    let zoned: Zoned = row.get(1);

    let date: Date = row.get(2);
    let time: Time = row.get(3);

    println!("{datetime}\n{ts}\n{zoned}\n{date}\n{time}");
    Ok(())
}
