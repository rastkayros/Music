// models.rs

use diesel::sql_types::*;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use postgis::ewkb::Point;
use std::vec::Vec;

// Структура представляющая город------------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct City {
    pub id: i32,                    // Идентификатор города
    pub name: String,               // Название города
    pub population: Option<i32>,    // Численность населения (Option, так как может быть неизвестной)
    pub area_sq_km: Option<BigDecimal>, // Площадь в квадратных километрах (Option, так как может быть неизвестной)
    pub mayor: Option<String>,      // Имя мэра (Option, так как может быть неизвестным)
    pub founding_date: Option<NaiveDate>, // Дата основания (Option, так как может быть неизвестной)
}

// Структура для вставки новых записей о городах в базу данных
#[derive(Debug, Clone, Insertable)]
#[table_name = "cities"]
pub struct NewCity {
    pub name: String,               // Название города
    pub population: Option<i32>,    // Численность населения (Option, так как может быть неизвестной)
    pub area_sq_km: Option<BigDecimal>, // Площадь в квадратных километрах (Option, так как может быть неизвестной)
    pub mayor: Option<String>,      // Имя мэра (Option, так как может быть неизвестным)
    pub founding_date: Option<NaiveDate>, // Дата основания (Option, так как может быть неизвестной)
}

impl City {
    // Метод для получения всех районов города
     pub fn get_districts(&self, connection: &PgConnection) -> QueryResult<Vec<District>> {
        districts::table
            .filter(districts::city_id.eq(self.id))
            .load(connection)
    }
    // Метод для получения всех улиц данного города
    pub fn get_streets(&self, connection: &PgConnection) -> QueryResult<Vec<Street>> {
        streets::table.filter(streets::district_id.eq_any(
            districts::table.select(districts::id).filter(districts::city_id.eq(self.id)),
        ))
        .load(connection)
    }
    // Метод для получения всех организаций города
    pub fn get_organizations(&self, connection: &PgConnection) -> QueryResult<Vec<Organization>> {
        Organization::belonging_to(self)
            .load(connection)
    }

}

// Структура представляющая районы---------------------------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct District {
    pub id: i32,                    // Идентификатор района
    pub city_id: i32,               // Идентификатор города
    pub name: String,               // Название района
    pub population: i32,            // Численность населения в районе
    pub area_sq_km: Option<BigDecimal>, // Площадь района в квадратных километрах (Option, так как может быть неизвестной)
}

// Структура для вставки новых записей о районах в базу данных
#[derive(Debug, Clone, Insertable)]
#[table_name = "districts"]
pub struct NewDistrict {
    pub city_id: i32,               // Идентификатор города
    pub name: String,               // Название района
    pub population: i32,            // Численность населения в районе
    pub area_sq_km: Option<BigDecimal>, // Площадь района в квадратных километрах (Option, так как может быть неизвестной)
}
// Методы для структуры NewDistrict
impl NewDistrict {
    // Метод для создания новой записи о районе
    pub fn create(new_district: NewDistrict, connection: &PgConnection) -> QueryResult<District> {
        diesel::insert_into(districts::table)
            .values(&new_district)
            .get_result(connection)
    }
    
    // Добавьте здесь другие методы, если они вам нужны
}

impl District {
    
    // Метод для создания нового района
    pub fn create(new_district: NewDistrict, connection: &PgConnection) -> QueryResult<District> {
        diesel::insert_into(districts::table)
            .values(&new_district)
            .get_result(connection)
    }

    // Метод для чтения района по идентификатору
    pub fn read_by_id(district_id: i32, connection: &PgConnection) -> QueryResult<District> {
        districts::table.find(district_id).first(connection)
    }

    // Метод для обновления данных о районе
    pub fn update(&self, connection: &PgConnection) -> QueryResult<District> {
        diesel::update(self)
            .set(self)
            .get_result(connection)
    }

    // Метод для удаления района по идентификатору
    pub fn delete(district_id: i32, connection: &PgConnection) -> QueryResult<()> {
        diesel::delete(districts::table.filter(districts::id.eq(district_id))).execute(connection)?;
        Ok(())
    }

    // Метод для получения города, к которому принадлежит район
    pub fn get_city(&self, connection: &PgConnection) -> QueryResult<City> {
        cities::table
            .filter(cities::id.eq(self.city_id))
            .first(connection)
    }

// Методы для структуры District
impl District {
    // Метод для получения всех улиц района
    pub fn get_streets(&self, connection: &PgConnection) -> QueryResult<Vec<Street>> {
        streets::table
            .filter(streets::district_id.eq(self.id))
            .load(connection)
    }

    // Метод для получения всех организаций района
    pub fn get_organizations(&self, connection: &PgConnection) -> QueryResult<Vec<Organization>> {
        organizations::table
            .filter(organizations::street_id.eq_any(
                streets::table.select(streets::id).filter(streets::district_id.eq(self.id)),
            ))
            .load(connection)
    }

    // Метод для получения всех товаров и услуг района
    pub fn get_goods_services(&self, connection: &PgConnection) -> QueryResult<Vec<GoodsService>> {
        goods_services::table
            .filter(goods_services::street_id.eq_any(
                streets::table.select(streets::id).filter(streets::district_id.eq(self.id)),
            ))
            .load(connection)
    }

    // Метод для получения всех бесплатных Wi-Fi точек и зарядок района
    pub fn get_wifi_charging(&self, connection: &PgConnection) -> QueryResult<Vec<WifiCharging>> {
        wifi_charging::table
            .filter(wifi_charging::street_id.eq_any(
                streets::table.select(streets::id).filter(streets::district_id.eq(self.id)),
            ))
            .load(connection)
    }

    // Метод для получения всех пользовательских отметок района
    pub fn get_user_points(&self, connection: &PgConnection) -> QueryResult<Vec<UserPoint>> {
        user_points::table
            .filter(user_points::street_id.eq_any(
                streets::table.select(streets::id).filter(streets::district_id.eq(self.id)),
            ))
            .load(connection)
    }

    // Метод для вывода информации о районе
    pub fn display_info(&self) {
        println!("Район: {}", &self.name);
        // Вывод другой информации о районе, если необходимо
    }
    
    // Добавьте здесь другие методы, если они вам нужны
}



}

// Структура представляющая улицы------------------------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Street {
    pub id: i32,                    // Идентификатор улицы
    pub district_id: i32,           // Идентификатор района
    pub name: String,               // Название улицы
    pub length_km: Option<BigDecimal>, // Длина улицы в километрах (Option, так как может быть неизвестной)
    pub surface_type: Option<String>, // Тип покрытия улицы (Option, так как может быть неизвестным)
    pub speed_limit: Option<i16>,    // Ограничение скорости на улице (Option, так как может быть неизвестным)
}

// Структура для вставки новых записей о улицах в базу данных
#[derive(Debug, Clone, Insertable)]
#[table_name = "streets"]
pub struct NewStreet {
    pub district_id: i32,           // Идентификатор района
    pub name: String,               // Название улицы
    pub length_km: Option<BigDecimal>, // Длина улицы в километрах (Option, так как может быть неизвестной)
    pub surface_type: Option<String>, // Тип покрытия улицы (Option, так как может быть неизвестным)
    pub speed_limit: Option<i16>,    // Ограничение скорости на улице (Option, так как может быть неизвестным)
}
// Методы для структуры NewStreet
impl NewStreet {
    // Метод для создания новой улицы
    pub fn create(new_street: NewStreet, connection: &PgConnection) -> QueryResult<Street> {
        diesel::insert_into(streets::table)
            .values(&new_street)
            .get_result(connection)
    }

    // Добавьте здесь другие методы, если они вам нужны
}

// Методы для структуры Street
impl Street {
    // Метод для получения всех организаций на улице
    pub fn get_organizations(&self, connection: &PgConnection) -> QueryResult<Vec<Organization>> {
        organizations::table
            .filter(organizations::street_id.eq(self.id))
            .load(connection)
    }

    // Метод для получения всех товаров и услуг на улице
    pub fn get_goods_services(&self, connection: &PgConnection) -> QueryResult<Vec<GoodsService>> {
        goods_services::table
            .filter(goods_services::street_id.eq(self.id))
            .load(connection)
    }

    // Метод для получения всех бесплатных Wi-Fi точек и зарядок на улице
    pub fn get_wifi_charging(&self, connection: &PgConnection) -> QueryResult<Vec<WifiCharging>> {
        wifi_charging::table
            .filter(wifi_charging::street_id.eq(self.id))
            .load(connection)
    }

    // Метод для получения всех пользовательских отметок на улице
    pub fn get_user_points(&self, connection: &PgConnection) -> QueryResult<Vec<UserPoint>> {
        user_points::table
            .filter(user_points::street_id.eq(self.id))
            .load(connection)
    }

    // Метод для вывода информации о улице
    pub fn display_info(&self) {
        println!("Улица: {}", &self.name);
        // Вывод другой информации о улице, если необходимо
    }

    // Метод для вывода фотографий улицы (если есть поле для фотографий в структуре Street)
    pub fn display_photos(&self) {
        if let Some(photos) = &self.photos {
            for photo in photos {
                // Вывод фотографий (здесь можно реализовать вывод в нужном формате)
                println!("Фотография: {:?}", photo);
            }
        }
    }

    // Добавьте здесь другие методы, если они вам нужны
}



// Структура представляющая организации на улицах-----------------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Organization {
    pub id: i32,                    // Идентификатор организации
    pub street_id: i32,             // Идентификатор улицы
    pub name: String,               // Название организации
    pub description: Option<String>, // Описание организации (Option, так как может быть неизвестным)
    pub contact_info: Option<String>, // Контактная информация (Option, так как может быть неизвестной)
    pub operating_hours: Option<String>, // Часы работы (Option, так как может быть неизвестными)
    pub website: Option<String>,     // Веб-сайт (Option, так как может быть неизвестным)
    pub email: Option<String>,       // Электронная почта (Option, так как может быть неизвестной)
    pub phone: Option<String>,       // Телефон (Option, так как может быть неизвестным)
    pub category: Option<String>,    // Категория организации (Option, так как может быть неизвестной)
    pub rating: Option<BigDecimal>,  // Рейтинг организации (Option, так как может быть неизвестным)
}

// Структура для вставки новых записей об организациях в базу данных
#[derive(Debug, Clone, Insertable)]
#[table_name = "organizations"]
pub struct NewOrganization {
    pub street_id: i32,             // Идентификатор улицы
    pub name: String,               // Название организации
    pub description: Option<String>, // Описание организации (Option, так как может быть неизвестным)
    pub contact_info: Option<String>, // Контактная информация (Option, так как может быть неизвестной)
    pub operating_hours: Option<String>, // Часы работы (Option, так как может быть неизвестными)
    pub website: Option<String>,     // Веб-сайт (Option, так как может быть неизвестным)
    pub email: Option<String>,       // Электронная почта (Option, так как может быть неизвестной)
    pub phone: Option<String>,       // Телефон (Option, так как может быть неизвестным)
    pub category: Option<String>,    // Категория организации (Option, так как может быть неизвестной)
    pub rating: Option<BigDecimal>,  // Рейтинг организации (Option, так как может быть неизвестным)
}

// Методы для структуры NewOrganization
impl NewOrganization {
    // Метод для создания новой записи об организации
    pub fn create(new_organization: NewOrganization, connection: &PgConnection) -> QueryResult<Organization> {
        diesel::insert_into(organizations::table)
            .values(&new_organization)
            .get_result(connection)
    }

    // Добавьте здесь другие методы, если они вам нужны
}


// Методы для структуры Organization
impl Organization {
    // Метод для создания новой записи об организации
    pub fn create(new_organization: NewOrganization, connection: &PgConnection) -> QueryResult<Organization> {
        diesel::insert_into(organizations::table)
            .values(&new_organization)
            .get_result(connection)
    }

    // Метод для чтения организации по идентификатору
    pub fn read_by_id(organization_id: i32, connection: &PgConnection) -> QueryResult<Organization> {
        organizations::table.find(organization_id).first(connection)
    }

    // Метод для обновления данных об организации
    pub fn update(&self, connection: &PgConnection) -> QueryResult<Organization> {
        diesel::update(self)
            .set(self)
            .get_result(connection)
    }

    // Метод для удаления организации по идентификатору
    pub fn delete(organization_id: i32, connection: &PgConnection) -> QueryResult<()> {
        diesel::delete(organizations::table.filter(organizations::id.eq(organization_id)))
            .execute(connection)?;
        Ok(())
    }

    // Метод для получения улицы, на которой находится организация
    pub fn get_street(&self, connection: &PgConnection) -> QueryResult<Street> {
        streets::table
            .filter(streets::id.eq(self.street_id))
            .first(connection)
    }

    // Метод для вывода информации об организации
    pub fn display_info(&self) {
        println!("Организация: {}", &self.name);
        println!("Описание: {:?}", &self.description);
        println!("Контактная информация: {:?}", &self.contact_info);
        println!("Часы работы: {:?}", &self.operating_hours);
        println!("Веб-сайт: {:?}", &self.website);
        println!("Электронная почта: {:?}", &self.email);
        println!("Телефон: {:?}", &self.phone);
        println!("Категория: {:?}", &self.category);
        println!("Рейтинг: {:?}", &self.rating);
    }

    // Добавьте здесь другие методы, если они вам нужны
}


// Структура представляющая товары и услуги на улицах----------------------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Goods {
    pub id: i32,                    // Идентификатор товара или услуги
    pub street_id: i32,             // Идентификатор улицы
    pub name: String,               // Название товара или услуги
    pub description: Option<String>, // Описание товара (Option, так как может быть неизвестным)
}

// Структура для вставки новых записей о товарах и услугах на улицах в базу данных
#[derive(Debug, Clone, Insertable)]
#[table_name = "goods"]
pub struct NewGoods {
    pub street_id: i32,             // Идентификатор улицы
    pub name: String,               // Название товара или услуги
    pub description: Option<String>, // Описание товара (Option, так как может быть неизвестным)
}
// Методы для структуры NewGoods
impl NewGoods {
    // Метод для создания новой записи о товаре или услуге
    pub fn create(new_goods: NewGoods, connection: &PgConnection) -> QueryResult<Goods> {
        diesel::insert_into(goods::table)
            .values(&new_goods)
            .get_result(connection)
    }

    // Добавьте здесь другие методы, если они вам нужны
}

i// Методы для структуры Goods
impl Goods {
    // Метод для создания новой записи о товаре или услуге
    pub fn create(new_goods: NewGoods, connection: &PgConnection) -> QueryResult<Goods> {
        diesel::insert_into(goods::table)
            .values(&new_goods)
            .get_result(connection)
    }

    // Метод для чтения товара или услуги по идентификатору
    pub fn read_by_id(goods_id: i32, connection: &PgConnection) -> QueryResult<Goods> {
        goods::table.find(goods_id).first(connection)
    }

    // Метод для обновления данных о товаре или услуге
    pub fn update(&self, connection: &PgConnection) -> QueryResult<Goods> {
        diesel::update(self)
            .set(self)
            .get_result(connection)
    }

    // Метод для удаления товара или услуги по идентификатору
    pub fn delete(goods_id: i32, connection: &PgConnection) -> QueryResult<()> {
        diesel::delete(goods::table.filter(goods::id.eq(goods_id)))
            .execute(connection)?;
        Ok(())
    }

    // Метод для получения улицы, на которой находится товар или услуга
    pub fn get_street(&self, connection: &PgConnection) -> QueryResult<Street> {
        streets::table
            .filter(streets::id.eq(self.street_id))
            .first(connection)
    }

    // Добавьте здесь другие методы, если они вам нужны
}

// Структура представляющая точки отмеченные пользователем и комментарии----------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct UserPoint {
    pub id: i32,                         // Идентификатор точки отмеченной пользователем
    pub street_id: i32,                  // Идентификатор улицы
    pub name: String,                    // Название точки
    pub description: Option<String>,     // Описание точки (Option, так как может быть неизвестным)
    pub category: Option<String>,        // Категория точки (Option, так как может быть неизвестной)
    pub rating: Option<BigDecimal>,      // Рейтинг точки (Option, так как может быть неизвестным)
    pub photo: Option<Vec<u8>>,          // Фотография точки в виде BLOB (Option, так как может быть неизвестной)
    pub latitude: f64,                   // Широта точки
    pub longitude: f64,                  // Долгота точки
    pub created_at: NaiveDateTime,       // Дата и время создания точки
    pub user_id: i32,                    // Идентификатор пользователя, создавшего точку
    pub external_link: Option<String>,    // Ссылка на внешний ресурс (Option, так как может быть неизвестной)
    pub average_rating: Option<f32>,     // Средний рейтинг точки (Option, так как может быть неизвестным)
    pub reviews: Option<Vec<String>>,    // Отзывы о точке (Option, так как может быть неизвестными)
}

// Структура для вставки новых записей о точках в базу данных
#[derive(Debug, Clone, Insertable)]
#[table_name = "user_points"]
pub struct NewUserPoint {
    pub street_id: i32,                  // Идентификатор улицы
    pub name: String,                    // Название точки
    pub description: Option<String>,     // Описание точки (Option, так как может быть неизвестным)
    pub category: Option<String>,        // Категория точки (Option, так как может быть неизвестной)
    pub rating: Option<BigDecimal>,      // Рейтинг точки (Option, так как может быть неизвестным)
    pub photo: Option<Vec<u8>>,          // Фотография точки в виде BLOB (Option, так как может быть неизвестной)
    pub latitude: f64,                   // Широта точки
    pub longitude: f64,                  // Долгота точки
    pub created_at: NaiveDateTime,       // Дата и время создания точки
    pub user_id: i32,                    // Идентификатор пользователя, создавшего точку
    pub external_link: Option<String>,    // Ссылка на внешний ресурс (Option, так как может быть неизвестной)
    pub average_rating: Option<f32>,     // Средний рейтинг точки (Option, так как может быть неизвестным)
    pub reviews: Option<Vec<String>>,    // Отзывы о точке (Option, так как может быть неизвестными)
}

// Методы для структуры NewUserPoint
impl NewUserPoint {
    // Метод для создания новой записи о точке, отмеченной пользователем
    pub fn create(new_user_point: NewUserPoint, connection: &PgConnection) -> QueryResult<UserPoint> {
        diesel::insert_into(user_points::table)
            .values(&new_user_point)
            .get_result(connection)
    }

    // Добавьте здесь другие методы, если они вам нужны
}


// Методы для структуры UserPoint
impl UserPoint {
    // Метод для создания новой записи о точке, отмеченной пользователем
    pub fn create(new_user_point: NewUserPoint, connection: &PgConnection) -> QueryResult<UserPoint> {
        diesel::insert_into(user_points::table)
            .values(&new_user_point)
            .get_result(connection)
    }

    // Метод для чтения точки по идентификатору
    pub fn read_by_id(user_point_id: i32, connection: &PgConnection) -> QueryResult<UserPoint> {
        user_points::table.find(user_point_id).first(connection)
    }

    // Метод для обновления данных о точке
    pub fn update(&self, connection: &PgConnection) -> QueryResult<UserPoint> {
        diesel::update(self)
            .set(self)
            .get_result(connection)
    }

    // Метод для удаления точки по идентификатору
    pub fn delete(user_point_id: i32, connection: &PgConnection) -> QueryResult<()> {
        diesel::delete(user_points::table.filter(user_points::id.eq(user_point_id)))
            .execute(connection)?;
        Ok(())
    }

    // Метод для получения улицы, на которой находится точка
    pub fn get_street(&self, connection: &PgConnection) -> QueryResult<Street> {
        streets::table
            .filter(streets::id.eq(self.street_id))
            .first(connection)
    }

    // Метод для добавления отзыва о точке
    pub fn add_review(&mut self, review: String) {
        if self.reviews.is_none() {
            self.reviews = Some(vec![review]);
        } else {
            if let Some(reviews) = &mut self.reviews {
                reviews.push(review);
            }
        }
    }

    // Метод для вывода информации о точке
    pub fn display_info(&self) {
        println!("Точка: {}", &self.name);
        println!("Описание: {:?}", &self.description);
        println!("Категория: {:?}", &self.category);
        println!("Рейтинг: {:?}", &self.rating);
        println!("Широта: {:?}", &self.latitude);
        println!("Долгота: {:?}", &self.longitude);
        println!("Дата и время создания: {:?}", &self.created_at);
        println!("Идентификатор пользователя: {:?}", &self.user_id);
        println!("Внешняя ссылка: {:?}", &self.external_link);
        if let Some(reviews) = &self.reviews {
            for (index, review) in reviews.iter().enumerate() {
                println!("Отзыв {}: {}", index + 1, review);
            }
        }
    }

    // Добавьте здесь другие методы, если они вам нужны
}


// Структура представляющая бесплатные Wi-Fi точки и зарядки------------------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct WifiChargingPoint {
    pub id: i32,                         // Идентификатор точки
    pub street_id: i32,                  // Идентификатор улицы
    pub name: String,                    // Название точки
    pub description: Option<String>,     // Описание точки (Option, так как может быть неизвестным)
    pub services: Option<String>,        // Доступные услуги (Option, так как может быть неизвестным)
    pub location: Option<Point>,         // Координаты точки на карте (Option, так как может быть неизвестным)
    pub rating: Option<BigDecimal>,      // Рейтинг точки (Option, так как может быть неизвестным)
}

// Структура для вставки новых записей о бесплатных Wi-Fi точках и зарядках в базу данных
#[derive(Debug, Clone, Insertable)]
#[table_name = "wifi_charging"]
pub struct NewWifiChargingPoint {
    pub street_id: i32,                  // Идентификатор улицы
    pub name: String,                    // Название точки
    pub description: Option<String>,     // Описание точки (Option, так как может быть неизвестным)
    pub services: Option<String>,        // Доступные услуги (Option, так как может быть неизвестным)
    pub location: Option<Point>,         // Координаты точки на карте (Option, так как может быть неизвестным)
    pub rating: Option<BigDecimal>,      // Рейтинг точки (Option, так как может быть неизвестным)
}
// Методы для структуры NewWifiChargingPoint
impl NewWifiChargingPoint {
    // Метод для создания новой записи о бесплатной Wi-Fi точке и зарядке
    pub fn create(new_wifi_charging_point: NewWifiChargingPoint, connection: &PgConnection) -> QueryResult<WifiChargingPoint> {
        diesel::insert_into(wifi_charging::table)
            .values(&new_wifi_charging_point)
            .get_result(connection)
    }

    // Добавьте здесь другие методы, если они вам нужны
}

// Методы для структуры WifiChargingPoint
impl WifiChargingPoint {
    // Метод для создания новой записи о бесплатной Wi-Fi точке и зарядке
    pub fn create(new_wifi_charging_point: NewWifiChargingPoint, connection: &PgConnection) -> QueryResult<WifiChargingPoint> {
        diesel::insert_into(wifi_charging::table)
            .values(&new_wifi_charging_point)
            .get_result(connection)
    }

    // Метод для чтения точки по идентификатору
    pub fn read_by_id(wifi_charging_point_id: i32, connection: &PgConnection) -> QueryResult<WifiChargingPoint> {
        wifi_charging::table.find(wifi_charging_point_id).first(connection)
    }

    // Метод для обновления данных о точке
    pub fn update(&self, connection: &PgConnection) -> QueryResult<WifiChargingPoint> {
        diesel::update(self)
            .set(self)
            .get_result(connection)
    }

    // Метод для удаления точки по идентификатору
    pub fn delete(wifi_charging_point_id: i32, connection: &PgConnection) -> QueryResult<()> {
        diesel::delete(wifi_charging::table.filter(wifi_charging::id.eq(wifi_charging_point_id)))
            .execute(connection)?;
        Ok(())
    }

    // Добавьте здесь другие методы, если они вам нужны
}

