use diesel::{self, prelude::*};
use chrono::NaiveDateTime;
use crate::schema::{tags, tag_object}; // Подключение схемы базы данных


//--------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Queryable, Identifiable)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub created_by: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTag {
    pub name: String,
    pub description: Option<String>,
    pub created_by: Option<i32>,
}

#[derive(AsChangeset)]
#[table_name = "tags"]
pub struct UpdateTag {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub created_by: Option<Option<i32>>,
}

impl Tag {
    // Метод для создания нового тега
    pub fn create(new_tag: NewTag, connection: &PgConnection) -> Result<Tag, diesel::result::Error> {
        diesel::insert_into(tags::table)
            .values(&new_tag)
            .get_result(connection)
    }

    // Метод для поиска тега по ID
    pub fn find_by_id(tag_id: i32, connection: &PgConnection) -> Result<Tag, diesel::result::Error> {
        tags::table.find(tag_id).get_result(connection)
    }

    // Метод для обновления существующего тега по ID
    pub fn update(tag_id: i32, updated_tag: UpdateTag, connection: &PgConnection) -> Result<Tag, diesel::result::Error> {
        diesel::update(tags::table.find(tag_id))
            .set(&updated_tag)
            .get_result(connection)
    }

    // Метод для удаления тега по ID
    pub fn delete(tag_id: i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        diesel::delete(tags::table.find(tag_id)).execute(connection)?;
        Ok(())
    }

    // Метод для получения всех тегов
    pub fn all(connection: &PgConnection) -> Result<Vec<Tag>, diesel::result::Error> {
        tags::table.load::<Tag>(connection)
    }
}

//--------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Queryable, Identifiable)]
pub struct TagObject {
    pub id: i32,
    pub tag_id: i32,
    pub object_id: i32,
    pub object_type: String,
    pub created_at: NaiveDateTime,
    pub created_by: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "tag_object"]
pub struct NewTagObject {
    pub tag_id: i32,
    pub object_id: i32,
    pub object_type: String,
    pub created_by: Option<i32>,
}

#[derive(AsChangeset)]
#[table_name = "tag_object"]
pub struct UpdateTagObject {
    pub tag_id: Option<i32>,
    pub object_id: Option<i32>,
    pub object_type: Option<String>,
    pub created_by: Option<Option<i32>>,
}

impl TagObject {
    // Метод для создания новой связи объекта с тегом
    pub fn create(new_tag_object: NewTagObject, connection: &PgConnection) -> Result<TagObject, diesel::result::Error> {
        diesel::insert_into(tag_object::table)
            .values(&new_tag_object)
            .get_result(connection)
    }

    // Метод для поиска связи объекта с тегом по ID
    pub fn find_by_id(tag_object_id: i32, connection: &PgConnection) -> Result<TagObject, diesel::result::Error> {
        tag_object::table.find(tag_object_id).get_result(connection)
    }

    // Метод для обновления существующей связи объекта с тегом по ID
    pub fn update(tag_object_id: i32, updated_tag_object: UpdateTagObject, connection: &PgConnection) -> Result<TagObject, diesel::result::Error> {
        diesel::update(tag_object::table.find(tag_object_id))
            .set(&updated_tag_object)
            .get_result(connection)
    }

    // Метод для удаления связи объекта с тегом по ID
    pub fn delete(tag_object_id: i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        diesel::delete(tag_object::table.find(tag_object_id)).execute(connection)?;
        Ok(())
    }
}


//--------------------------------------------------------------------------------------------------------------------
