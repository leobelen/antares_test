use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use once_cell::sync::OnceCell;
use r2d2::Pool;

///A convenient way to have the DB Connection abstracted out.
pub type DbConnection = PgConnection;
pub type ConnManager = ConnectionManager<DbConnection>;

///A Database pool for use on the system.
pub struct DatabasePool {
    pub pool: Pool<ConnManager>,
}

static INSTANCE: OnceCell<DatabasePool> = OnceCell::new();

impl DatabasePool {
    /// Creates a new database pool from the URL
    pub fn new(url: &str) -> anyhow::Result<&'static DatabasePool> {
        Ok(INSTANCE.get_or_init(|| {
            let manager = ConnectionManager::<DbConnection>::new(url);
            let pool: Pool<ConnManager> =
                r2d2::Pool::builder().max_size(30).build(manager).unwrap();
            DatabasePool { pool }
        }))
    }
    /// gets the instance that was previously created to avoid passing the
    /// connection around in cases in which transactions are not needed.
    pub fn instance() -> anyhow::Result<&'static DatabasePool> {
        Ok(INSTANCE.get().unwrap())
    }
}
