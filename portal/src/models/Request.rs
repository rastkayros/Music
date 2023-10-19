

use chrono::NaiveDateTime;
use diesel::sql_types::{Integer, Text};
use geo_types::Point;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Request {
    pub id: i32,
    pub user_id: i32,
    pub service_id: i32,
    pub request_date: NaiveDateTime,
    pub description: Option<String>,
    pub status: String,
    pub assigned_to: Option<i32>,
    pub priority: i16,
    pub due_date: Option<chrono::NaiveDate>,
    pub location: Option<Point<f64>>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub comments: Option<String>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "requests"]
pub struct NewRequest {
    pub user_id: i32,
    pub service_id: i32,
    pub request_date: NaiveDateTime,
    pub description: Option<String>,
    pub status: String,
    pub assigned_to: Option<i32>,
    pub priority: i16,
    pub due_date: Option<chrono::NaiveDate>,
    pub location: Option<Point<f64>>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub comments: Option<String>,
}


use crate::schema::requests;
impl Request {
// Метод для вставки новой Заявки в базу данных
pub fn create_request(
    conn: &diesel::PgConnection,
    new_request: &NewRequest,
) -> Result<Request, diesel::result::Error> {
    diesel::insert_into(requests::table)
        .values(new_request)
        .get_result(conn)
}

// Метод для получения списка всех Заявок
pub fn get_all_requests(conn: &diesel::PgConnection) -> Result<Vec<Request>, diesel::result::Error> {
    requests::table.load::<Request>(conn)
}

// Метод для получения Заявки по её ID
pub fn get_request_by_id(
    conn: &diesel::PgConnection,
    request_id: i32,
) -> Result<Request, diesel::result::Error> {
    requests::table.find(request_id).first(conn)
}

// Метод для обновления Заявки по её ID
pub fn update_request_by_id(
    conn: &diesel::PgConnection,
    request_id: i32,
    updated_request: &NewRequest,
) -> Result<Request, diesel::result::Error> {
    diesel::update(requests::table.find(request_id))
        .set(updated_request)
        .get_result(conn)
}

// Метод для удаления Заявки по её ID
pub fn delete_request_by_id(
    conn: &diesel::PgConnection,
    request_id: i32,
) -> Result<(), diesel::result::Error> {
    diesel::delete(requests::table.find(request_id)).execute(conn)?;
    Ok(())
}

// Метод для получения всех Заявок пользователя по user_id
pub fn get_requests_by_user_id(
    conn: &diesel::PgConnection,
    user_id: i32,
) -> Result<Vec<Request>, diesel::result::Error> {
    requests::table
        .filter(requests::user_id.eq(user_id))
        .load::<Request>(conn)
}

// Метод для получения всех Заявок с определенным статусом
pub fn get_requests_by_status(
    conn: &diesel::PgConnection,
    status: &str,
) -> Result<Vec<Request>, diesel::result::Error> {
    requests::table
        .filter(requests::status.eq(status))
        .load::<Request>(conn)
}

// Метод для получения всех Заявок, которые назначены определенному сотруднику
pub fn get_requests_by_assigned_to(
    conn: &diesel::PgConnection,
    assigned_to: i32,
) -> Result<Vec<Request>, diesel::result::Error> {
    requests::table
        .filter(requests::assigned_to.eq(assigned_to))
        .load::<Request>(conn)
}

// Метод для получения всех Заявок с определенным приоритетом
pub fn get_requests_by_priority(
    conn: &diesel::PgConnection,
    priority: i16,
) -> Result<Vec<Request>, diesel::result::Error> {
    requests::table
        .filter(requests::priority.eq(priority))
        .load::<Request>(conn)
}

// Метод для поиска Заявок по ключевым словам в описании или комментариях
pub fn search_requests_by_keyword(
    conn: &diesel::PgConnection,
    keyword: &str,
) -> Result<Vec<Request>, diesel::result::Error> {
    requests::table
        .filter(
            requests::description
                .like(format!("%{}%", keyword))
                .or(requests::comments.like(format!("%{}%", keyword))),
        )
        .load::<Request>(conn)
}

// Метод для получения количества Заявок в определенном статусе
pub fn count_requests_by_status(
    conn: &diesel::PgConnection,
    status: &str,
) -> Result<i64, diesel::result::Error> {
    requests::table
        .filter(requests::status.eq(status))
        .count()
        .get_result(conn)
}

// Метод для получения всех Заявок, отсортированных по дате подачи
pub fn get_requests_sorted_by_date(
    conn: &diesel::PgConnection,
) -> Result<Vec<Request>, diesel::result::Error> {
    requests::table
        .order(requests::request_date)
        .load::<Request>(conn)
}

// Метод для удаления всех Заявок, у которых срок выполнения истек
pub fn delete_expired_requests(
    conn: &diesel::PgConnection,
    current_date: chrono::NaiveDate,
) -> Result<(), diesel::result::Error> {
    diesel::delete(requests::table.filter(requests::due_date.le(current_date)))
        .execute(conn)?;
    Ok(())
}
// Метод для получения списка всех уникальных пользователей, которые создали Заявки
pub fn get_unique_request_users(
    conn: &diesel::PgConnection,
) -> Result<Vec<String>, diesel::result::Error> {
    requests::table
        .select(requests::contact_name)
        .distinct()
        .filter(requests::contact_name.is_not_null())
        .load::<String>(conn)
}

// Метод для получения списка всех Заявок, у которых есть контактный email
pub fn get_requests_with_email(
    conn: &diesel::PgConnection,
) -> Result<Vec<Request>, diesel::result::Error> {
    requests::table
        .filter(requests::contact_email.is_not_null())
        .load::<Request>(conn)
}

// Метод для получения списка всех Заявок с определенным типом услуги
pub fn get_requests_by_service_type(
    conn: &diesel::PgConnection,
    service_type: &str,
) -> Result<Vec<Request>, diesel::result::Error> {
    requests::table
        .filter(requests::service_id.eq(service_type))
        .load::<Request>(conn)
}
}
