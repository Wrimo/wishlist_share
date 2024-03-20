use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct familyData {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct familypersonData {
    pub id: i32,
    pub person_id: Option<i32>,
    pub family_id: Option<i32>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct personData {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct wishlistData {
    pub id: i32,
    pub person_id: Option<i32>,
    pub comment: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct wishlistitemData {
    pub id: i32,
    pub wishlist_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub link: Option<String>,
    pub purchased: Option<bool>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct get_person_familyData {
    pub person_id: Option<i32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct get_person_wishlist_itemsData {
    pub wishlist_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub link: Option<String>,
    pub purchased: Option<bool>,
}
