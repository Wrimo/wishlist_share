use sqlx::Row;
use sqlx::postgres::{PgRow};
use super::generated_types::*;


pub async fn get_person_family(pool: &sqlx::Pool<sqlx::Postgres>, person_id: i32) -> Result<Vec<get_person_familyData>, sqlx::Error> {
    let response: Vec<PgRow> = sqlx::query(format!("SELECT * FROM get_person_family({});", person_id).as_str()).fetch_all(pool).await?;
    let mut result: Vec<get_person_familyData> = Vec::new();
    for i in 0..response.len() {
        let x = get_person_familyData {
            person_id: response[i].try_get(0)?,
            first_name: response[i].try_get(1)?,
            last_name: response[i].try_get(2)?,
    };
    result.push(x);
    }
	Ok(result)  
}

pub async fn get_person_wishlist_items(pool: &sqlx::Pool<sqlx::Postgres>, person_id: i32) -> Result<Vec<get_person_wishlist_itemsData>, sqlx::Error> {
    let response: Vec<PgRow> = sqlx::query(format!("SELECT * FROM get_person_wishlist_items({});", person_id).as_str()).fetch_all(pool).await?;
    let mut result: Vec<get_person_wishlist_itemsData> = Vec::new();
    for i in 0..response.len() {
        let x = get_person_wishlist_itemsData {
            wishlist_id: response[i].try_get(0)?,
            name: response[i].try_get(1)?,
            description: response[i].try_get(2)?,
            link: response[i].try_get(3)?,
            purchased: response[i].try_get(4)?,
    };
    result.push(x);
    }
	Ok(result)  
}
