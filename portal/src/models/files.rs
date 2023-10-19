// Для работы с actix-web
use actix_web::{web, App, HttpServer, HttpResponse, Responder};

// Для работы с библиотекой diesel
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

// Для работы с PostgreSQL
use dotenv::dotenv;
use std::env;




//---------------------------------------------------------------------------------------------------------------
// Модель для таблицы "file_tags"
#[derive(Queryable, Identifiable, Associations, Debug)]
#[table_name = "file_tags"]
pub struct FileTag {
    pub id: i32,
    pub file_id: i32,
    pub tag_id: i32,
}

//Создание новой записи в таблице "file_tags"
use diesel::{Insertable, Queryable, ExpressionMethods, PgConnection, RunQueryDsl};
use chrono::Utc;

#[derive(Insertable)]
#[table_name = "file_tags"]
struct NewFileTag {
    file_id: i32,
    tag_id: i32,
}

impl NewFileTag {
    fn create(file_id: i32, tag_id: i32, conn: &PgConnection) -> Result<(), diesel::result::Error> {
        let new_file_tag = NewFileTag { file_id, tag_id };
        diesel::insert_into(file_tags::table)
            .values(&new_file_tag)
            .execute(conn)?;
        Ok(())
    }
}

//Запрос всех тегов для определенного файла по его file_id

impl FileTag {
    fn find_by_file_id(file_id: i32, conn: &PgConnection) -> Result<Vec<FileTag>, diesel::result::Error> {
        file_tags::table.filter(file_tags::file_id.eq(file_id))
            .load(conn)
    }
}

//Запрос всех файлов, к которым применен определенный тег по его tag_id

impl FileTag {
    fn find_files_by_tag_id(tag_id: i32, conn: &PgConnection) -> Result<Vec<File>, diesel::result::Error> {
        file_tags::table
            .filter(file_tags::tag_id.eq(tag_id))
            .inner_join(files::table)
            .select(files::all_columns)
            .load(conn)
    }
}

//Удаление тега для определенного файла по file_id и tag_id

impl FileTag {
    fn delete(file_id: i32, tag_id: i32, conn: &PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(file_tags::table
            .filter(file_tags::file_id.eq(file_id))
            .filter(file_tags::tag_id.eq(tag_id)))
            .execute(conn)
    }
}

//Получение количества тегов для определенного файла

impl FileTag {
    fn count_tags_for_file(file_id: i32, conn: &PgConnection) -> Result<i64, diesel::result::Error> {
        file_tags::table
            .filter(file_tags::file_id.eq(file_id))
            .count()
            .get_result(conn)
    }
}

//---------------------------------------------------------------------------------------------------------------
// Модель для таблицы "files"
#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "files"]
pub struct File {
    pub id: i32,
    pub user_id: i32,
    pub community_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub file_type: String,
    pub file_url: Option<String>,
    pub is_private: bool,
    pub is_approved: Option<bool>,
    pub is_deleted: bool,
    pub uploaded_at: chrono::NaiveDateTime,
}

// Структура для создания новой записи файла
#[derive(Insertable)]
#[table_name = "files"]
struct NewFile<'a> {
    user_id: i32,
    title: &'a str,
    description: Option<&'a str>,
    file_type: &'a str,
    file_url: Option<&'a str>,
    is_private: bool,
    is_approved: Option<bool>,
    is_deleted: bool,
    uploaded_at: chrono::NaiveDateTime,
}

impl<'a> NewFile<'a> {
    fn create(
        user_id: i32,
        title: &'a str,
        description: Option<&'a str>,
        file_type: &'a str,
        file_url: Option<&'a str>,
        is_private: bool,
        conn: &PgConnection,
    ) -> Result<File, diesel::result::Error> {
        use diesel::RunQueryDsl;

        let uploaded_at = chrono::Utc::now().naive_utc();
        let new_file = NewFile {
            user_id,
            title,
            description,
            file_type,
            file_url,
            is_private,
            is_approved: None,
            is_deleted: false,
            uploaded_at,
        };
        diesel::insert_into(files::table)
            .values(&new_file)
            .get_result(conn)
    }
}

// Метод для поиска файла по его идентификатору (id)
fn find_file_by_id(file_id: i32, conn: &PgConnection) -> Result<Option<File>, diesel::result::Error> {

}

// Метод для обновления информации о файле
fn update_file(
    file_id: i32,
    new_title: String,
    new_description: Option<String>,
    new_file_type: String,
    new_file_url: Option<String>,
    new_is_private: bool,
    conn: &PgConnection,
) -> Result<(), diesel::result::Error> {
    
}

// Метод для удаления файла по его идентификатору (id)
fn delete_file(file_id: i32, conn: &PgConnection) -> Result<(), diesel::result::Error> {
    
}

// Метод для получения списка всех файлов пользователя
fn get_user_files(user_id: i32, conn: &PgConnection) -> Result<Vec<File>, diesel::result::Error> {
    
}

// Метод для получения списка всех файлов в сообществе (если применимо)
fn get_community_files(
    community_id: i32,
    conn: &PgConnection,
) -> Result<Vec<File>, diesel::result::Error> {
    
}

// Метод для поиска файлов по их типу (например, "изображение", "документ")
fn search_files_by_type(
    file_type: String,
    conn: &PgConnection,
) -> Result<Vec<File>, diesel::result::Error> {
    
}

// Метод для одобрения файла администратором
fn approve_file(file_id: i32, conn: &PgConnection) -> Result<(), diesel::result::Error> {

}

// Метод для получения списка всех файлов с учетом флага приватности
fn get_all_files_with_privacy(
    is_private: bool,
    conn: &PgConnection,
) -> Result<Vec<File>, diesel::result::Error> {
    
}

// Метод для получения списка файлов, одобренных администратором
fn get_approved_files(conn: &PgConnection) -> Result<Vec<File>, diesel::result::Error> {
    
}

// Метод для получения списка файлов, загруженных в определенное время
fn get_files_uploaded_between_dates(
    start_date: chrono::NaiveDateTime,
    end_date: chrono::NaiveDateTime,
    conn: &PgConnection,
) -> Result<Vec<File>, diesel::result::Error> {
    
}

//---------------------------------------------------------------------------------------------------------------


// Модель для таблицы "file_comments"
#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(File, foreign_key = "file_id")]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "file_comments"]
pub struct FileComment {
    pub id: i32,
    pub file_id: i32,
    pub user_id: i32,
    pub comment_text: Option<String>,
    pub is_deleted: bool,
    pub created_at: chrono::NaiveDateTime,
}

//Создание нового комментария к файлу

use diesel::Insertable;

#[derive(Insertable)]
#[table_name = "file_comments"]
struct NewFileComment<'a> {
    file_id: i32,
    user_id: i32,
    comment_text: &'a str,
    is_deleted: bool,
    created_at: chrono::NaiveDateTime,
}

impl<'a> NewFileComment<'a> {
    fn create(
        file_id: i32,
        user_id: i32,
        comment_text: &'a str,
        conn: &PgConnection,
    ) -> Result<FileComment, diesel::result::Error> {
        use diesel::RunQueryDsl;

        let created_at = chrono::Utc::now().naive_utc();
        let new_comment = NewFileComment {
            file_id,
            user_id,
            comment_text,
            is_deleted: false,
            created_at,
        };
        diesel::insert_into(file_comments::table)
            .values(&new_comment)
            .get_result(conn)
    }
}

//Получение комментариев к файлу по его file_id

impl FileComment {
    fn find_comments_by_file_id(
        file_id: i32,
        conn: &PgConnection,
    ) -> Result<Vec<FileComment>, diesel::result::Error> {
        file_comments::table
            .filter(file_comments::file_id.eq(file_id))
            .load(conn)
    }
}

//Удаление комментария к файлу по его id

impl FileComment {
    fn delete_comment_by_id(
        comment_id: i32,
        conn: &PgConnection,
    ) -> Result<(), diesel::result::Error> {
        diesel::delete(file_comments::table.filter(file_comments::id.eq(comment_id)))
            .execute(conn)?;
        Ok(())
    }
}

//Получение всех комментариев пользователя по его user_id

impl FileComment {
    fn find_comments_by_user_id(
        user_id: i32,
        conn: &PgConnection,
    ) -> Result<Vec<FileComment>, diesel::result::Error> {
        file_comments::table
            .filter(file_comments::user_id.eq(user_id))
            .load(conn)
    }
}

//Удаление всех комментариев для определенного файла по его file_id

impl FileComment {
    fn delete_comments_for_file(
        file_id: i32,
        conn: &PgConnection,
    ) -> Result<(), diesel::result::Error> {
        diesel::delete(file_comments::table.filter(file_comments::file_id.eq(file_id)))
            .execute(conn)?;
        Ok(())
    }
}

//---------------------------------------------------------------------------------------------------------------


// Модель для таблицы "file_likes"
#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(File, foreign_key = "file_id")]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "file_likes"]
pub struct FileLike {
    pub id: i32,
    pub file_id: i32,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
}
impl FileLike{
// Метод для создания нового лайка для файла
fn create_like_for_file(
    file_id: i32,
    user_id: i32,
    conn: &PgConnection,
) -> Result<FileLike, diesel::result::Error> {
    
}


// Метод для получения списка пользователей, поставивших лайки для файла
fn get_users_who_liked_file(
    file_id: i32,
    conn: &PgConnection,
) -> Result<Vec<User>, diesel::result::Error> {

}

// Метод для получения количества лайков для файла
fn count_likes_for_file(
    file_id: i32,
    conn: &PgConnection,
) -> Result<i64, diesel::result::Error> {
    
}

// Метод для удаления лайка пользователя для файла
fn delete_like_for_user(
    user_id: i32,
    file_id: i32,
    conn: &PgConnection,
) -> Result<(), diesel::result::Error> {
    
}

}
//---------------------------------------------------------------------------------------------------------------



// Модель для таблицы "file_downloads"
#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(File, foreign_key = "file_id")]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "file_downloads"]
pub struct FileDownload {
    pub id: i32,
    pub file_id: i32,
    pub user_id: i32,
    pub downloaded_at: chrono::NaiveDateTime,
}
impl FileDownload{
    // Метод для создания новой записи о загрузке файла
    fn create_download_record(
        file_id: i32,
        user_id: i32,
        conn: &PgConnection,
    ) -> Result<FileDownload, diesel::result::Error> {
    
    }

    // Метод для получения списка пользователей, скачавших файл
    fn get_users_who_downloaded_file(
        file_id: i32,
        conn: &PgConnection,
    ) -> Result<Vec<User>, diesel::result::Error> {
    
    }

// Метод для получения количества загрузок файла
fn count_downloads_for_file(
    file_id: i32,
    conn: &PgConnection,
) -> Result<i64, diesel::result::Error> {
    
}

// Метод для проверки, скачал ли пользователь файл
fn user_downloaded_file(
    user_id: i32,
    file_id: i32,
    conn: &PgConnection,
) -> Result<bool, diesel::result::Error> {
    
}

// Метод для удаления записи о загрузке файла пользователем
fn delete_download_record_for_user(
    user_id: i32,
    file_id: i32,
    conn: &PgConnection,
) -> Result<(), diesel::result::Error> {
    
}

// Метод для получения списка файлов, которые пользователь скачал
fn get_user_downloaded_files(
    user_id: i32,
    conn: &PgConnection,
) -> Result<Vec<File>, diesel::result::Error> {
    
}

// Метод для получения списка загрузок для определенного файла
fn get_downloads_for_file(
    file_id: i32,
    conn: &PgConnection,
) -> Result<Vec<FileDownload>, diesel::result::Error> {
    
}

// Метод для получения даты и времени последней загрузки файла пользователем
fn get_last_download_time_for_user(
    user_id: i32,
    file_id: i32,
    conn: &PgConnection,
) -> Result<Option<chrono::NaiveDateTime>, diesel::result::Error> {
    
}

// Метод для удаления всех записей о загрузках для определенного файла
fn delete_all_download_records_for_file(
    file_id: i32,
    conn: &PgConnection,
) -> Result<(), diesel::result::Error> {
    
}

// Метод для получения всех записей о загрузках для пользователя
fn get_all_downloads_for_user(
    user_id: i32,
    conn: &PgConnection,
) -> Result<Vec<FileDownload>, diesel::result::Error> {
    
}

// Метод для получения всех записей о загрузках файла в определенный период времени
fn get_downloads_for_file_in_period(
    file_id: i32,
    start_time: chrono::NaiveDateTime,
    end_time: chrono::NaiveDateTime,
    conn: &PgConnection,
) -> Result<Vec<FileDownload>, diesel::result::Error> {
    
}

// Метод для удаления всех записей о загрузках для пользователя
fn delete_all_download_records_for_user(
    user_id: i32,
    conn: &PgConnection,
) -> Result<(), diesel::result::Error> {
    
}

}
//---------------------------------------------------------------------------------------------------------------



// Модель для таблицы "file_report"
#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(File, foreign_key = "file_id")]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "file_report"]
pub struct FileReport {
    pub id: i32,
    pub file_id: i32,
    pub user_id: i32,
    pub reason: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

use crate::schema::{file_report, files, users}; 

impl FileReport {
    // Метод для создания новой записи в таблице file_report
    pub fn create_report(conn: &PgConnection, file_id: i32, user_id: i32, reason: String, description: Option<String>, status: Option<String>) -> QueryResult<FileReport> {
        let new_report = NewFileReport {
            file_id,
            user_id,
            reason,
            description,
            status,
        };

        diesel::insert_into(file_report::table)
            .values(&new_report)
            .get_result(conn)
    }

    // Метод для получения отчета по идентификатору
    pub fn get_report_by_id(conn: &PgConnection, report_id: i32) -> QueryResult<FileReport> {
        file_report::table.find(report_id).first(conn)
    }

}

// Структура для вставки нового отчета
#[derive(Insertable)]
#[table_name = "file_report"]
pub struct NewFileReport {
    pub file_id: i32,
    pub user_id: i32,
    pub reason: String,
    pub description: Option<String>,
    pub status: Option<String>,
}


impl NewFileReport {
    // Метод для создания новой записи отчета
    pub fn new(file_id: i32, user_id: i32, reason: String, description: Option<String>, status: Option<String>) -> Self {
        NewFileReport {
            file_id,
            user_id,
            reason,
            description,
            status,
        }
    }

    // Метод для вставки нового отчета в базу данных
    pub fn insert(&self, conn: &PgConnection) -> QueryResult<FileReport> {
        diesel::insert_into(file_report::table)
            .values(self)
            .get_result(conn)
    }
}

use diesel::prelude::*;
use crate::schema::{file_report, files, users}; // Подключение необходимых схем и таблиц

impl FileReport {
    // Метод для получения всех отчетов для конкретного файла
    pub fn get_reports_for_file(conn: &PgConnection, file_id: i32) -> QueryResult<Vec<FileReport>> {
        file_report::table.filter(file_report::file_id.eq(file_id)).load(conn)
    }

    // Метод для получения всех отчетов, созданных конкретным пользователем
    pub fn get_reports_by_user(conn: &PgConnection, user_id: i32) -> QueryResult<Vec<FileReport>> {
        file_report::table.filter(file_report::user_id.eq(user_id)).load(conn)
    }

    // Метод для обновления статуса отчета по его идентификатору
    pub fn update_report_status(conn: &PgConnection, report_id: i32, new_status: String) -> QueryResult<FileReport> {
        diesel::update(file_report::table.filter(file_report::id.eq(report_id)))
            .set(file_report::status.eq(new_status))
            .get_result(conn)
    }

    // Метод для удаления отчета по его идентификатору
    pub fn delete_report(conn: &PgConnection, report_id: i32) -> QueryResult<()> {
        diesel::delete(file_report::table.filter(file_report::id.eq(report_id))).execute(conn)?;
        Ok(())
    }
}

//---------------------------------------------------------------------------------------------------------------
