// use sqlx::{Connection, Executor, PgConnection, PgPool};
use diesel::r2d2::{self, ConnectionManager};
use diesel::{Connection, PgConnection, RunQueryDsl};
use uuid::Uuid;

use app::configuration::{get_configuration, DatabaseSettings};
use app::startup::{get_connection_pool, Application};

type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        println!("spawning with dburl: {} ", c.database.without_db());
        c
    };
    configure_database(&configuration.database);

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address: format!("http://127.0.0.1:{}", application_port),
        port: application_port,
        db_pool: get_connection_pool(&configuration.database),
    }
}

fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create
    let connection =
        PgConnection::establish(&config.without_db()).expect("Failed to connect Postgres.------");
    diesel::sql_query(format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .execute(&connection)
        .expect("Failed to create database.");

    // Migrate
    let manager = ConnectionManager::<PgConnection>::new(config.with_db());
    let pool = r2d2::Pool::builder()
        .connection_timeout(std::time::Duration::from_secs(2))
        .build(manager)
        .expect("Failed to create pool.");
    let connection_pool = pool.get().expect("Failed to connect to Postgres.");
    diesel_migrations::run_pending_migrations(&connection_pool)
        .expect("Failed to migrate the database.");

    pool
}
