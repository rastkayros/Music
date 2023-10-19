// Подключаем необходимые модули
use diesel::prelude::*;
use chrono::NaiveDateTime; 

// Структура для модели пользователя
#[derive(Queryable)]
pub struct User {
    pub id: i32,                // Уникальный идентификатор
    pub first_name: String,     // Имя пользователя
    pub last_name: String,      // Фамилия пользователя
    pub middle_name: String,    // Отчество пользователя
    pub email: String,          // Адрес электронной почты
    pub phone_number: String,   // Номер телефона пользователя
    pub description: String,    // Описание пользователя
    pub photo_link: String,     // Ссылка на фотографию пользователя
}

// Структура для создания новых объектов пользователя
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: String,     // Имя пользователя
    pub last_name: String,      // Фамилия пользователя
    pub middle_name: String,    // Отчество пользователя
    pub email: String,          // Адрес электронной почты
    pub phone_number: String,   // Номер телефона пользователя
    pub description: String,    // Описание пользователя
    pub photo_link: String,     // Ссылка на фотографию пользователя
}

use diesel::prelude::*;
use diesel::result::Error;
use crate::schema::users; // Замените `schema::users` на фактический путь к схеме таблицы в вашем проекте

// Методы для структуры User
impl User {
    // Метод для создания нового объекта структуры User
    pub fn new() -> User {
        User {
            id: 0,
            first_name: String::new(),
            last_name: String::new(),
            middle_name: String::new(),
            email: String::new(),
            phone_number: String::new(),
            description: String::new(),
            photo_link: String::new(),
        }
    }

    // Метод для поиска объекта по идентификатору
    pub fn find_by_id(id: i32, connection: &PgConnection) -> Result<Option<User>, Error> {
        users::table.find(id).first(connection).optional()
    }

    // Метод для получения всех объектов данной структуры
    pub fn find_all(connection: &PgConnection) -> Result<Vec<User>, Error> {
        users::table.load(connection)
    }

    // Метод для обновления существующего объекта
    pub fn update(&self, connection: &PgConnection) -> Result<(), Error> {
        diesel::update(users::table.find(self.id))
            .set(self)
            .execute(connection)?;
        Ok(())
    }

    // Метод для удаления объекта
    pub fn delete(&self, connection: &PgConnection) -> Result<(), Error> {
        diesel::delete(users::table.find(self.id)).execute(connection)?;
        Ok(())
    }

    // Метод для поиска объектов по значению определенного поля
    pub fn find_by_field(field_value: &str, connection: &PgConnection) -> Result<Vec<User>, Error> {
        users::table.filter(users::first_name.eq(field_value)
            .or(users::last_name.eq(field_value))
            .or(users::middle_name.eq(field_value))
            .or(users::email.eq(field_value))
            .or(users::phone_number.eq(field_value)))
            .load(connection)
    }

    // Метод для подсчета общего количества объектов данной структуры
    pub fn count(connection: &PgConnection) -> Result<usize, Error> {
        users::table.count().get_result(connection)
    }
}

// Метод для создания нового объекта на основе данных из структуры NewUser
impl NewUser {
    pub fn create(new_data: NewUser, connection: &PgConnection) -> Result<User, Error> {
        diesel::insert_into(users::table)
            .values(&new_data)
            .get_result(connection)
    }
}

//----------------------------------------------------------------------------------------

// Подключаем необходимые модули
use diesel::prelude::*;
use chrono::NaiveDateTime;

// Структура для модели пользователя
#[derive(Queryable)]
pub struct UserList {
    pub id: i32,                // Уникальный идентификатор
    pub user_id: String,        // ID пользователя
    pub review_ids: Vec<i32>,   // Идентификаторы оставленных отзывов
    pub comments_ids: Vec<i32>, // Идентификаторы оставленных комментариев
    pub organizations_ids: Vec<i32>, // Идентификаторы зарегистрированных организаций
    pub deceaseds_ids: Vec<i32>, // Идентификаторы зарегистрированных усопших
}

// Структура для создания новых объектов пользователя
#[derive(Insertable)]
#[table_name = "userslist"]
pub struct NewUserList {
    pub user_id: String,        // ID пользователя
    pub review_ids: Vec<i32>,   // Идентификаторы оставленных отзывов
    pub comments_ids: Vec<i32>, // Идентификаторы оставленных комментариев
    pub organizations_ids: Vec<i32>, // Идентификаторы зарегистрированных организаций
    pub deceaseds_ids: Vec<i32>, // Идентификаторы зарегистрированных усопших
}


impl UserList {
    // Метод для создания нового объекта структуры UserList.
    pub fn new(id: i32, user_id: String, review_ids: Vec<i32>, comments_ids: Vec<i32>,
               organizations_ids: Vec<i32>, deceaseds_ids: Vec<i32>) -> Self {
        UserList {
            id,
            user_id,
            review_ids,
            comments_ids,
            organizations_ids,
            deceaseds_ids,
        }
    }

    // Метод для поиска объекта по идентификатору.
    pub fn find_by_id(id: i32, connection: &PgConnection) -> QueryResult<Option<Self>> {
        userslist::table.find(id).first(connection).optional()
    }

    // Метод для получения всех объектов данной структуры.
    pub fn find_all(connection: &PgConnection) -> QueryResult<Vec<Self>> {
        userslist::table.load(connection)
    }

    // Метод для обновления существующего объекта.
    pub fn update(&self, connection: &PgConnection) -> Result<(), Error> {
        diesel::update(userslist::table.find(self.id))
            .set(self)
            .execute(connection)?;
        Ok(())
    }

    // Метод для удаления объекта.
    pub fn delete(&self, connection: &PgConnection) -> Result<(), Error> {
        diesel::delete(userslist::table.find(self.id))
            .execute(connection)?;
        Ok(())
    }

    // Метод для поиска объектов по значению определенного поля.
    pub fn find_by_field(field_value: &str, connection: &PgConnection) -> QueryResult<Vec<Self>> {
        userslist::table.filter(userslist::user_id.eq(field_value))
            .load(connection)
    }

    // Метод для подсчета общего количества объектов данной структуры.
    pub fn count(connection: &PgConnection) -> QueryResult<usize> {
        userslist::table.count().get_result(connection)
    }
}

impl NewUserList {
    // Метод для создания нового объекта на основе данных из структуры NewUserList.
    pub fn create(new_data: NewUserList, connection: &PgConnection) -> QueryResult<Self> {
        diesel::insert_into(userslist::table)
            .values(&new_data)
            .get_result(connection)
    }
}

//--------------------------------------------------------------------------------------------------

// Подключаем необходимые модули
use diesel::prelude::*;
use chrono::NaiveDateTime;

// Структура для модели пользователя
#[derive(Queryable)]
pub struct UserSecrets {
    pub id: i32,                // Уникальный идентификатор
    pub user_id: String,        // ID пользователя
    pub password: String,       // Пароль пользователя
    pub phone_number: String,   // Номер телефона пользователя
}

// Структура для создания новых объектов пользователя
#[derive(Insertable)]
#[table_name = "userssecrets"]
pub struct NewUserSecrets {
    pub user_id: String,        // ID пользователя
    pub password: String,       // Пароль пользователя
    pub phone_number: String,   // Номер телефона пользователя
}


impl UserSecrets {
    // Метод для создания нового объекта структуры.
    pub fn new(user_id: String, password: String, phone_number: String) -> Self {
        UserSecrets {
            id: 0,  // Вы можете установить значение id по умолчанию здесь
            user_id,
            password,
            phone_number,
        }
    }

    // Метод для поиска объекта по идентификатору.
    pub fn find_by_id(id: i32, connection: &PgConnection) -> QueryResult<Option<Self>> {
        use crate::schema::userssecrets::dsl::*;

        userssecrets
            .filter(id.eq(id))
            .first(connection)
            .optional()
    }

    // Метод для получения всех объектов данной структуры.
    pub fn find_all(connection: &PgConnection) -> QueryResult<Vec<Self>> {
        use crate::schema::userssecrets::dsl::*;

        userssecrets.load(connection)
    }

    // Метод для обновления существующего объекта.
    pub fn update(&self, connection: &PgConnection) -> QueryResult<usize> {
        use crate::schema::userssecrets::dsl::*;

        diesel::update(userssecrets.filter(id.eq(self.id)))
            .set(self)
            .execute(connection)
    }

    // Метод для удаления объекта.
    pub fn delete(&self, connection: &PgConnection) -> QueryResult<usize> {
        use crate::schema::userssecrets::dsl::*;

        diesel::delete(userssecrets.filter(id.eq(self.id)))
            .execute(connection)
    }

    // Метод для поиска объектов по значению определенного поля.
    pub fn find_by_field(field_value: &str, field_name: &str, connection: &PgConnection) -> QueryResult<Vec<Self>> {
        use crate::schema::userssecrets::dsl::*;

        userssecrets
            .filter(diesel::dsl::sql(&format!("{} = ?", field_name)), field_value)
            .load(connection)
    }

    // Метод для подсчета общего количества объектов данной структуры.
    pub fn count(connection: &PgConnection) -> QueryResult<usize> {
        use crate::schema::userssecrets::dsl::*;

        userssecrets.count().get_result(connection)
    }
}

impl NewUserSecrets {
    // Метод для создания нового объекта на основе данных из структуры NewUser.
    pub fn create(new_data: NewUserSecrets, connection: &PgConnection) -> QueryResult<Self> {
        diesel::insert_into(userssecrets::table)
            .values(&new_data)
            .get_result(connection)
    }
}