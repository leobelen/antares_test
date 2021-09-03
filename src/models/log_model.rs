use crate::schema::core_log::dsl::*;
use chrono::{DateTime, Utc};
use diesel::{Associations, Identifiable, Insertable, Queryable};

///The application Struct. This contains the details of the applications to which
/// every role is subscribed and it is meant to allow the clients to display the links
/// dynamically on the user interface.
#[derive(Queryable, Debug, Clone, Associations)]
#[table_name = "core_log"]
pub struct Log {
    pub id: i64,
    pub log_content: String,
    pub post_date: DateTime<Utc>,
}
