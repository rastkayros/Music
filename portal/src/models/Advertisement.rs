use diesel::{Insertable, Queryable, QueryResult, RunQueryDsl, PgConnection};
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;
use rand::Rng;

// Структура для модели Объявления
#[derive(Debug, Queryable)]
pub struct Advertisement {
    pub id: i32,                         // Идентификатор объявления
    pub user_id: i32,                    // Идентификатор пользователя, разместившего объявление
    pub community_id: Option<i32>,       // Идентификатор сообщества, если применимо
    pub title: String,                   // Заголовок объявления
    pub description: Option<String>,     // Описание объявления
    pub category: Option<String>,        // Категория объявления
    pub price: Option<BigDecimal>,       // Цена, если применимо
    pub currency: Option<String>,        // Валюта, если есть
    pub location: Option<String>,        // Местоположение
    pub contact_name: Option<String>,    // Имя контактного лица
    pub contact_phone: Option<String>,   // Телефон контактного лица
    pub contact_email: Option<String>,   // Email контактного лица
    pub images: Vec<String>,             // Ссылки на изображения объявления, массив
    pub video: Option<String>,           // Ссылка на видео объявления, если есть
    pub is_active: bool,                // Флаг активности объявления
    pub created_at: NaiveDateTime,      // Дата и время создания
}

// Структура для создания новых объявлений
#[derive(Debug, Insertable)]
#[table_name = "advertisements"]
pub struct NewAdvertisement {
    pub user_id: i32,
    pub community_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub price: Option<BigDecimal>,
    pub currency: Option<String>,
    pub location: Option<String>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub images: Vec<String>,
    pub video: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}

impl Advertisement {
    /// Создает новое объявление и сохраняет его в базу данных.
    pub fn create(new_advertisement: NewAdvertisement, connection: &PgConnection) -> QueryResult<Advertisement> {
        use crate::schema::advertisements::dsl::*;
        diesel::insert_into(advertisements)
            .values(&new_advertisement)
            .get_result(connection)
    }

    /// Возвращает список всех объявлений пользователя.
    pub fn find_by_user(user_id: i32, connection: &PgConnection) -> QueryResult<Vec<Advertisement>> {
        use crate::schema::advertisements::dsl::*;
        advertisements.filter(user_id.eq(user_id))
            .load::<Advertisement>(connection)
    }

    /// Возвращает список всех активных объявлений.
    pub fn find_active(connection: &PgConnection) -> QueryResult<Vec<Advertisement>> {
        use crate::schema::advertisements::dsl::*;
        advertisements.filter(is_active.eq(true))
            .load::<Advertisement>(connection)
    }

    /// Метод для поиска объявления по его идентификатору
    pub fn find_by_id(id: i32, connection: &PgConnection) -> QueryResult<Option<Advertisement>> {
        use crate::schema::advertisements::dsl::*;
        advertisements.filter(id.eq(id))
            .first(connection)
            .optional()
    }
    /// Метод для получения всех объявлений в указанной категории
    pub fn find_by_category(category: &str, connection: &PgConnection) -> QueryResult<Vec<Advertisement>> {
        use crate::schema::advertisements::dsl::*;
        advertisements.filter(category.eq(category))
            .load::<Advertisement>(connection)
    }

    /// Метод для установки статуса активности объявления
    pub fn set_active_status(id: i32, is_active: bool, connection: &PgConnection) -> QueryResult<()> {
        use crate::schema::advertisements::dsl::*;
        diesel::update(advertisements.find(id))
            .set(is_active.eq(is_active))
            .execute(connection)?;
        Ok(())
    }

    /// Метод для получения всех объявлений в сообществе
    pub fn find_by_community(community_id: i32, connection: &PgConnection) -> QueryResult<Vec<Advertisement>> {
        use crate::schema::advertisements::dsl::*;
        advertisements.filter(community_id.eq(community_id))
            .load::<Advertisement>(connection)
    }

    /// Метод для поиска объявлений по ключевому слову в заголовке или описании
    pub fn search(keyword: &str, connection: &PgConnection) -> QueryResult<Vec<Advertisement>> {
        use crate::schema::advertisements::dsl::*;
        advertisements.filter(title.ilike(format!("%{}%", keyword)))
            .or_filter(description.ilike(format!("%{}%", keyword)))
            .load::<Advertisement>(connection)
    }

    /// Метод для получения самых новых объявлений
    pub fn find_newest(limit: i64, connection: &PgConnection) -> QueryResult<Vec<Advertisement>> {
        use crate::schema::advertisements::dsl::*;
        advertisements.order(created_at.desc())
            .limit(limit)
            .load::<Advertisement>(connection)
    }

    /// Метод для получения количества объявлений в указанной категории
    pub fn count_by_category(category: &str, connection: &PgConnection) -> QueryResult<i64> {
        use crate::schema::advertisements::dsl::*;
        advertisements.filter(category.eq(category))
            .count()
            .get_result(connection)
    }

    /// Метод для получения всех активных объявлений пользователя
    pub fn find_active_by_user(user_id: i32, connection: &PgConnection) -> QueryResult<Vec<Advertisement>> {
        use crate::schema::advertisements::dsl::*;
        advertisements.filter(user_id.eq(user_id))
            .filter(is_active.eq(true))
            .load::<Advertisement>(connection)
    }

    /// Метод для сортировки объявлений по цене (по возрастанию).
    pub fn sort_by_price_asc(connection: &PgConnection) -> QueryResult<Vec<Advertisement>> {
        use crate::schema::advertisements::dsl::*;
        advertisements.order(price.asc())
            .load::<Advertisement>(connection)
    }

    /// Метод для сортировки объявлений по цене (по убыванию).
    pub fn sort_by_price_desc(connection: &PgConnection) -> QueryResult<Vec<Advertisement>> {
        use crate::schema::advertisements::dsl::*;
        advertisements.order(price.desc())
            .load::<Advertisement>(connection)
    }

    /// Метод для сортировки объявлений по дате создания (по возрастанию).
    pub fn sort_by_created_at_asc(connection: &PgConnection) -> QueryResult<Vec<Advertisement>> {
        use crate::schema::advertisements::dsl::*;
        advertisements.order(created_at.asc())
            .load::<Advertisement>(connection)
    }

    /// Метод для сортировки объявлений по дате создания (по убыванию).
    pub fn sort_by_created_at_desc(connection: &PgConnection) -> QueryResult<Vec<Advertisement>> {
        use crate::schema::advertisements::dsl::*;
        advertisements.order(created_at.desc())
            .load::<Advertisement>(connection)
    }

     // Метод для удаления объявления по его идентификатору.
     pub fn delete_by_id(id: i32, connection: &PgConnection) -> QueryResult<()> {
        use crate::schema::advertisements::dsl::*;
        diesel::delete(advertisements.filter(id.eq(id)))
            .execute(connection)?;
        Ok(())
    }

    
}