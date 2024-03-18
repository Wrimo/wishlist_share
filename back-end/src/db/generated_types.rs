
use rocket::serde::{Serialize, json::Json};

pub struct familyData {
  pub id: i32,
  pub name: String,

}
pub struct familypersonData {
  pub id: i32,
  pub person_id: i32,
  pub family_id: i32,

}
pub struct personData {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
  pub email: String,

}
pub struct wishlistData {
  pub id: i32,
  pub person_id: i32,
  pub comment: String,

}
pub struct wishlistitemData {
  pub id: i32,
  pub wishlist_id: i32,
  pub name: String,
  pub link: String,
  pub description: String,
  pub purchased: bool,

}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct get_person_familyData {
  pub person_id: i32,
  pub first_name: String,
  pub last_name: String,

}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct get_person_wishlist_itemsData {
  pub wishlist_id: i32,
  pub name: String,
  pub description: String,
  pub link: String,
  pub purchased: bool,

}