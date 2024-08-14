use jiff::{
    civil::{Date, DateTime, Time},
    SpanRound, Timestamp, Unit,
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
    // timestamp in Postgres
    let datetime: DateTime = row.get(0);
    // timestamptz in Postgres
    let ts: Timestamp = row.get(1);

    let date: Date = row.get(2);
    let time: Time = row.get(3);

    // test FromSql
    println!("{datetime}\n{ts}\n{date}\n{time}");

    // test ToSql
    // in the query, the Postgres type is text unless we cast
    let row = client.query_one(
        "SELECT $1::timestamp, $2::timestamptz, $3::date, $4::time",
        &[&datetime, &ts, &date, &time],
    )?;
    println!(
        "{datetime}\n{ts}\n{date}\n{time}",
        datetime = row.get::<_, DateTime>(0),
        ts = row.get::<_, Timestamp>(1),
        date = row.get::<_, Date>(2),
        time = row.get::<_, Time>(3)
    );
    Ok(())
}
