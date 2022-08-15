use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Page {
    pub id: uuid::Uuid,
    pub title: String,
    pub page_url: String,
    pub published: bool,
}

#[derive(Queryable)]
pub struct Comment {
    pub id: uuid::Uuid,
    pub page_id: uuid::Uuid,
    pub reply_to: Option<uuid::Uuid>,
    pub ip_addr: ipnetwork::IpNetwork,
    pub display_name: String,
    pub site_url: Option<String>,
    pub mail_addr: Option<String>,
    pub content: String,
    pub delete_key: String,
    pub flags: i32,
    pub created_time: chrono::DateTime<chrono::Utc>,
}
