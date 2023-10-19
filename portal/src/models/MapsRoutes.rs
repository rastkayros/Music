



//-------------Не написаны методы-----------------------










use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::Serialize;

// Структура для таблицы "maps"----------------------------------------------------------------------------------------------1
#[derive(Queryable, Identifiable, Serialize)]
pub struct Map {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub location: Point,
    pub public: bool,
    pub likes: i32,
    pub views: i32,
}

// Структуры типа "New" для создания новых записей
#[derive(Insertable)]
#[table_name = "maps"]
pub struct NewMap<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub user_id: i32,
    pub location: Point,
    pub public: bool,
}





// Структура для таблицы "layers"---------------------------------------------------------------------------------------------------2
#[derive(Queryable, Identifiable, Serialize)]
pub struct Layer {
    pub id: i32,
    pub map_id: i32,
    pub title: String,
    pub description: String,
    pub layer_type: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub visibility: bool,
    pub opacity: f64,
}



#[derive(Insertable)]
#[table_name = "layers"]
pub struct NewLayer<'a> {
    pub map_id: i32,
    pub title: &'a str,
    pub description: &'a str,
    pub layer_type: &'a str,
    pub user_id: i32,
    pub visibility: bool,
    pub opacity: f64,
}









// Структура для таблицы "user_maps"------------------------------------------------------------------------------------------------------3
#[derive(Queryable, Identifiable, Serialize)]
pub struct UserMap {
    pub id: i32,
    pub user_id: i32,
    pub map_id: i32,
    pub can_edit: bool,
    pub can_view: bool,
}



#[derive(Insertable)]
#[table_name = "user_maps"]
pub struct NewUserMap {
    pub user_id: i32,
    pub map_id: i32,
    pub can_edit: bool,
    pub can_view: bool,
}














// Структура для таблицы "routes"--------------------------------------------------------------------------------------------------------4
#[derive(Queryable, Identifiable, Serialize)]
pub struct Route {
    pub id: i32,
    pub map_id: i32,
    pub title: String,
    pub description: String,
    pub user_id: i32,
    pub route_data: serde_json::Value,
    pub created_at: NaiveDateTime,
}



#[derive(Insertable)]
#[table_name = "routes"]
pub struct NewRoute<'a> {
    pub map_id: i32,
    pub title: &'a str,
    pub description: &'a str,
    pub user_id: i32,
    pub route_data: serde_json::Value,
}




















// Структура для таблицы "route_points"------------------------------------------------------------------------------------------------------5
#[derive(Queryable, Identifiable, Serialize)]
pub struct RoutePoint {
    pub id: i32,
    pub route_id: i32,
    pub title: String,
    pub description: String,
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
}


#[derive(Insertable)]
#[table_name = "route_points"]
pub struct NewRoutePoint<'a> {
    pub route_id: i32,
    pub title: &'a str,
    pub description: &'a str,
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
}













// Структура для таблицы "route_reviews"--------------------------------------------------------------------------------------------------------6
#[derive(Queryable, Identifiable, Serialize)]
pub struct RouteReview {
    pub id: i32,
    pub route_id: i32,
    pub user_id: i32,
    pub rating: f64,
    pub comment: String,
    pub created_at: NaiveDateTime,
}


#[derive(Insertable)]
#[table_name = "route_reviews"]
pub struct NewRouteReview<'a> {
    pub route_id: i32,
    pub user_id: i32,
    pub rating: f64,
    pub comment: &'a str,
}














// Структура для таблицы "route_tags"---------------------------------------------------------------------------------------------------------------7
#[derive(Queryable, Identifiable, Serialize)]
pub struct RouteTag {
    pub id: i32,
    pub route_id: i32,
    pub tag_name: String,
    pub category: Option<String>,
}



#[derive(Insertable)]
#[table_name = "route_tags"]
pub struct NewRouteTag<'a> {
    pub route_id: i32,
    pub tag_name: &'a str,
    pub category: Option<&'a str>,
}











// Структура для таблицы "route_images"--------------------------------------------------------------------------------------------------------8
#[derive(Queryable, Identifiable, Serialize)]
pub struct RouteImage {
    pub id: i32,
    pub route_id: i32,
    pub image_url: String,
    pub caption: Option<String>,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "route_images"]
pub struct NewRouteImage<'a> {
    pub route_id: i32,
    pub image_url: &'a str,
    pub caption: Option<&'a str>,
    pub user_id: i32,
}












// Структура для таблицы "route_comments"-------------------------------------------------------------------------------------------------9
#[derive(Queryable, Identifiable, Serialize)]
pub struct RouteComment {
    pub id: i32,
    pub route_id: i32,
    pub user_id: i32,
    pub comment_text: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "route_comments"]
pub struct NewRouteComment<'a> {
    pub route_id: i32,
    pub user_id: i32,
    pub comment_text: &'a str,
}


