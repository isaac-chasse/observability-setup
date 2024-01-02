use chrono::Utc;
use dotenv::dotenv;
use rand::Rng;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::time::{sleep, Duration};
use tracing::{info, error};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // Set up tracing
    setup_tracing();

    // Load environment variables from .env file
    dotenv().ok();

    // Get the database URL from the environment
    let db_url = env!("DATABASE_URL");

    // Create a PostgreSQL connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create pool");

    // Stream random data to the database
    stream_data_to_database(pool).await;
}

async fn stream_data_to_database(pool: PgPool) {
    // Create a loop to continuously stream data
    loop {
        // Generate random data
        let random_number = rand::thread_rng().gen_range(0..=100);
        let time = Utc::now();

        // Log generated data
        info!("Generated data: Time = {:?}, Value = {}", time, random_number);

        // Insert the data into the database
        if let Err(err) = sqlx::query!(
            "INSERT INTO random_process_real_time (time, value) VALUES ($1, $2)",
            time,
            random_number
        )
        .execute(&pool)
        .await
        {
            error!("Error inserting data into the database: {:?}", err);
        }

        // Sleep for 1 second before generating the next data point
        sleep(Duration::from_secs(1)).await;
    }
}

fn setup_tracing() {
    // Set up tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    // Initialize tracing
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");
}

