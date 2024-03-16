use sqlx::Row;
use sqlx::postgres::{PgRow};
use super::generated_types::*;


pub async fn get_person_wishlist_items(pool: &sqlx::Pool<sqlx::Postgres>, first_name: String, last_name: String) -> Result<Vec<get_person_wishlist_itemsData>, sqlx::Error> {
    let response: Vec<PgRow> = sqlx::query(format!("SELECT * FROM get_person_wishlist_items({}, {});", first_name, last_name).as_str()).fetch_all(pool).await?;
    let mut result: Vec<get_person_wishlist_itemsData> = Vec::new();
    for i in 0..response.len() {
        let x = get_person_wishlist_itemsData {
            name: response[i].try_get(0)?,
            description: response[i].try_get(1)?,
    };
    result.push(x);
    }
	Ok(result)  
}
