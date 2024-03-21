use sqlx::Row;
use sqlx::postgres::{PgRow};
use super::generated_types::*;


pub async fn get_person_family(pool: &sqlx::Pool<sqlx::Postgres>, person_id: i32) -> Result<Vec<get_person_familyData>, sqlx::Error> {
    let response: Vec<PgRow> = sqlx::query(format!("SELECT * FROM get_person_family({});", person_id).as_str()).fetch_all(pool).await?;
    let mut result: Vec<get_person_familyData> = Vec::new();
    for i in 0..response.len() {
        let x = get_person_familyData {
            person_id: response[i].try_get(0).ok(),
            first_name: response[i].try_get(1).ok(),
            last_name: response[i].try_get(2).ok(),
    };
    result.push(x);
    }
	Ok(result)  
}

pub async fn get_person_wishlist_items(pool: &sqlx::Pool<sqlx::Postgres>, person_id: i32) -> Result<Vec<wishlistitemData>, sqlx::Error> {
    let response: Vec<PgRow> = sqlx::query(format!("SELECT * FROM get_person_wishlist_items({});", person_id).as_str()).fetch_all(pool).await?;
    let mut result: Vec<wishlistitemData> = Vec::new();
    for i in 0..response.len() {
        let x = wishlistitemData {
            id: response[i].try_get(0)?,
            wishlist_id: response[i].try_get(1).ok(),
            name: response[i].try_get(2)?,
            description: response[i].try_get(3).ok(),
            link: response[i].try_get(4).ok(),
            purchased: response[i].try_get(5).ok(),
    };
    result.push(x);
    }
	Ok(result)  
}

pub async fn set_purchased(pool: &sqlx::Pool<sqlx::Postgres>, item_id: i32) -> Result<(), sqlx::Error>
{
    sqlx::raw_sql(format!("CALL set_purchased({});", item_id).as_str()).execute(pool).await?;
    Ok(())
}
