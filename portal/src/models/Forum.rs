// Подключаем необходимые модули
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::schema::forums; // Здесь подставьте имя вашего модуля с описанием схемы базы данных

// Создаем структуру Форум
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Forum {
    pub id: i32,                     // Идентификатор форума
    pub name: String,                // Название форума
    pub description: Option<String>, // Описание форума (может быть пустым)
    pub category: Option<String>,    // Категория форума (может быть пустой)
    pub is_private: bool,            // Флаг приватности форума
    pub created_at: NaiveDateTime,   // Дата и время создания форума
    pub updated_at: NaiveDateTime,   // Дата и время последнего обновления форума
}

// Добавляем поддержку вставки данных в базу (Insertable)
#[derive(Insertable)]
#[table_name = "forums"]
pub struct NewForum {
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub is_private: bool,
}

impl Forum {
// Функция для создания нового форума
pub fn create_forum(new_forum: NewForum, connection: &PgConnection) -> Result<Forum, diesel::result::Error> {
    diesel::insert_into(forums::table)
        .values(&new_forum)
        .get_result(connection)
}

// Функция для получения списка всех форумов
pub fn get_all_forums(connection: &PgConnection) -> Result<Vec<Forum>, diesel::result::Error> {
    forums::table.load::<Forum>(connection)
}

// Получение форума по его идентификатору
pub fn get_forum_by_id(forum_id: i32, connection: &PgConnection) -> Result<Option<Forum>, diesel::result::Error> {
    forums::table.find(forum_id).first(connection).optional()
}

// Удаление форума по его идентификатору
pub fn delete_forum(forum_id: i32, connection: &PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(forums::table.find(forum_id))
        .execute(connection)
}


//Получение списка форумов по категории

pub fn get_forums_by_category(category: &str, connection: &PgConnection) -> Result<Vec<Forum>, diesel::result::Error> {
    forums::table.filter(forums::category.eq(category))
        .load::<Forum>(connection)
}

//Поиск форумов по названию

pub fn search_forums_by_name(query: &str, connection: &PgConnection) -> Result<Vec<Forum>, diesel::result::Error> {
    forums::table.filter(forums::name.like(format!("%{}%", query)))
        .load::<Forum>(connection)
}

//Получение количества тем на каждом форуме

use diesel::dsl::sql;
use diesel::sql_types::Integer;

pub fn get_forum_topic_count(connection: &PgConnection) -> Result<Vec<(i32, i32)>, diesel::result::Error> {
    sql::<(Integer, Integer)>("SELECT f.id, COUNT(t.id) FROM forums AS f
        LEFT JOIN topics AS t ON f.id = t.forum_id
        GROUP BY f.id")
        .load(connection)
}

//Получение последних созданных форумов (например, для отображения на главной странице)

use diesel::prelude::*;
use diesel::dsl::max;

pub fn get_latest_forums(limit: i64, connection: &PgConnection) -> Result<Vec<Forum>, diesel::result::Error> {
    forums::table
        .order_by(forums::created_at.desc())
        .limit(limit)
        .load::<Forum>(connection)
}

//Получение количества активных участников на форуме

use diesel::dsl::{count, sql};
use diesel::sql_types::BigInt;

pub fn get_forum_active_users(forum_id: i32, connection: &PgConnection) -> Result<i64, diesel::result::Error> {
    sql::<BigInt>("SELECT COUNT(DISTINCT user_id) FROM posts WHERE forum_id = ?")
        .bind::<diesel::sql_types::Int4, _>(forum_id)
        .get_result(connection)
}

//Получение списка последних обновлений на форуме (например, новых сообщений)

use diesel::prelude::*;

pub fn get_forum_latest_updates(forum_id: i32, limit: i64, connection: &PgConnection) -> Result<Vec<ForumUpdate>, diesel::result::Error> {
    forums::table
        .filter(forums::id.eq(forum_id))
        .inner_join(posts::table)
        .order_by(posts::created_at.desc())
        .limit(limit)
        .select((
            forums::id,
            forums::name,
            posts::created_at,
            posts::content,
        ))
        .load::<ForumUpdate>(connection)
}
}
//--------------------------------------------------------------------------------------------------------------

// Подключаем необходимые модули
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::schema::forum_topics; 

// Создаем структуру "forum_topics" (Темы форума)
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ForumTopic {
    pub id: i32,                     // Идентификатор темы
    pub forum_id: i32,               // Идентификатор форума, к которому принадлежит тема
    pub title: String,               // Заголовок темы
    pub description: Option<String>, // Описание темы (может быть пустым)
    pub user_id: i32,                // Идентификатор пользователя, создавшего тему
    pub created_at: NaiveDateTime,   // Дата и время создания темы
    pub updated_at: NaiveDateTime,   // Дата и время последнего обновления темы
}

// Добавляем поддержку вставки данных в базу (Insertable)
#[derive(Insertable)]
#[table_name = "forum_topics"]
pub struct NewForumTopic {
    pub forum_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub user_id: i32,
}

// Это описание структуры "forum_topics" и функций, которые могут пригодиться
impl ForumTopic {
// Функция для создания новой темы
pub fn create_forum_topic(new_topic: NewForumTopic, connection: &PgConnection) -> Result<ForumTopic, diesel::result::Error> {
    diesel::insert_into(forum_topics::table)
        .values(&new_topic)
        .get_result(connection)
}

// Функция для получения списка всех тем форума
pub fn get_all_forum_topics(connection: &PgConnection) -> Result<Vec<ForumTopic>, diesel::result::Error> {
    forum_topics::table
    .load::<ForumTopic>(connection)
}

//Получение списка тем по идентификатору форума

pub fn get_topics_by_forum_id(forum_id: i32, connection: &PgConnection) -> Result<Vec<ForumTopic>, diesel::result::Error> {
    forum_topics::table
        .filter(forum_topics::forum_id.eq(forum_id))
        .load::<ForumTopic>(connection)
}

//Получение темы по её идентификатору

pub fn get_topic_by_id(topic_id: i32, connection: &PgConnection) -> Result<Option<ForumTopic>, diesel::result::Error> {
    forum_topics::table
        .find(topic_id)
        .first(connection)
        .optional()
}

//Удаление темы по её идентификатору

pub fn delete_topic(topic_id: i32, connection: &PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(forum_topics::table.find(topic_id))
        .execute(connection)
}

//Получение списка тем, созданных определенным пользователем

pub fn get_topics_by_user_id(user_id: i32, connection: &PgConnection) -> Result<Vec<ForumTopic>, diesel::result::Error> {
    forum_topics::table
        .filter(forum_topics::user_id.eq(user_id))
        .load::<ForumTopic>(connection)
}

//Получение последних активных тем форума

pub fn get_latest_active_topics(forum_id: i32, limit: i64, connection: &PgConnection) -> Result<Vec<ForumTopic>, diesel::result::Error> {
    forum_topics::table
        .filter(forum_topics::forum_id.eq(forum_id))
        .order_by(forum_topics::updated_at.desc())
        .limit(limit)
        .load::<ForumTopic>(connection)
}

//Получение количества сообщений в каждой теме форума

use diesel::dsl::{sql, count};
use diesel::sql_types::Integer;

pub fn get_topic_message_counts(forum_id: i32, connection: &PgConnection) -> Result<Vec<(i32, i32)>, diesel::result::Error> {
    sql::<(Integer, Integer)>("SELECT t.id, COUNT(m.id) FROM forum_topics AS t
        LEFT JOIN forum_messages AS m ON t.id = m.topic_id
        WHERE t.forum_id = ?
        GROUP BY t.id")
        .bind::<diesel::sql_types::Int4, _>(forum_id)
        .load(connection)
}
//Обратите внимание, что некоторые из этих методов предполагают наличие таблицы forum_messages для хранения сообщений в темах. 
//Получение списка тем с наибольшим количеством сообщений

pub fn get_topics_with_most_messages(limit: i64, connection: &PgConnection) -> Result<Vec<ForumTopic>, diesel::result::Error> {
    let subquery = sql::<diesel::sql_types::Int8>("SELECT topic_id, COUNT(id) FROM forum_messages GROUP BY topic_id");
    
    forum_topics::table
        .filter(forum_topics::id.eq_any(subquery))
        .order_by(sql::<diesel::sql_types::Int8>("COUNT(id) DESC"))
        .limit(limit)
        .load::<ForumTopic>(connection)
}


//Получение списка популярных тем на форуме (например, наиболее просматриваемых или комментируемых)


pub fn get_popular_topics(limit: i64, connection: &PgConnection) -> Result<Vec<ForumTopic>, diesel::result::Error> {
    forum_topics::table
        .order_by(sql::<diesel::sql_types::Int8>("(SELECT COUNT(id) FROM forum_messages WHERE topic_id = forum_topics.id) DESC"))
        .limit(limit)
        .load::<ForumTopic>(connection)
}

//Обратите внимание, что для некоторых из этих методов может потребоваться настройка соответствующих таблиц и моделей, 
//таких как таблица для подписок пользователей (user_subscriptions) и статус модерации (moderation_status).

//Получение списка тем, на которые пользователь подписан

pub fn get_topics_user_subscribed(user_id: i32, connection: &PgConnection) -> Result<Vec<ForumTopic>, diesel::result::Error> {
    use crate::schema::user_subscriptions;

    user_subscriptions::table
        .filter(user_subscriptions::user_id.eq(user_id))
        .select(user_subscriptions::topic_id)
        .load::<i32>(connection)
        .and_then(|topic_ids| {
            forum_topics::table
                .filter(forum_topics::id.eq_any(topic_ids))
                .load(connection)
        })
}

//Получение списка тем, на которые пользователь подписан

pub fn get_topics_user_subscribed(user_id: i32, connection: &PgConnection) -> Result<Vec<ForumTopic>, diesel::result::Error> {
    use crate::schema::user_subscriptions;

    user_subscriptions::table
        .filter(user_subscriptions::user_id.eq(user_id))
        .select(user_subscriptions::topic_id)
        .load::<i32>(connection)
        .and_then(|topic_ids| {
            forum_topics::table
                .filter(forum_topics::id.eq_any(topic_ids))
                .load(connection)
        })
}

//Получение списка популярных тем, отсортированных по количеству просмотров

pub fn get_popular_topics_by_views(limit: i64, connection: &PgConnection) -> Result<Vec<ForumTopic>, diesel::result::Error> {
    forum_topics::table
        .order_by(forum_topics::views.desc())
        .limit(limit)
        .load::<ForumTopic>(connection)
}

//Получение списка популярных тем, отсортированных по количеству просмотров

pub fn get_popular_topics_by_views(limit: i64, connection: &PgConnection) -> Result<Vec<ForumTopic>, diesel::result::Error> {
    forum_topics::table
        .order_by(forum_topics::views.desc())
        .limit(limit)
        .load::<ForumTopic>(connection)
}

//Получение списка тем, на которые пользователь не отвечал, чтобы побудить его активность

pub fn get_unanswered_topics_for_user(user_id: i32, connection: &PgConnection) -> Result<Vec<ForumTopic>, diesel::result::Error> {
    use diesel::dsl::{not, exists};
    use crate::schema::forum_messages;

    forum_topics::table
        .filter(not(exists(
            forum_messages::table
                .filter(forum_messages::topic_id.eq(forum_topics::id).and(forum_messages::user_id.eq(user_id)))
        )))
        .load::<ForumTopic>(connection)
}

//Получение списка тем, ожидающих модерацию

pub fn get_topics_pending_moderation(connection: &PgConnection) -> Result<Vec<ForumTopic>, diesel::result::Error> {
    forum_topics::table
        .filter(forum_topics::moderation_status.eq("pending"))
        .load::<ForumTopic>(connection)
}
}
//-----------------------------------------------------------------------------------

// Подключаем необходимые модули
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::schema::forum_posts; // Здесь подставьте имя вашего модуля с описанием схемы базы данных

// Создаем структуру "forum_posts" (Сообщения на форуме)
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ForumPost {
    pub id: i32,                     // Идентификатор сообщения
    pub topic_id: i32,               // Идентификатор темы, к которой принадлежит сообщение
    pub user_id: i32,                // Идентификатор пользователя, написавшего сообщение
    pub content: String,             // Содержание сообщения
    pub created_at: NaiveDateTime,   // Дата и время создания сообщения
    pub updated_at: NaiveDateTime,   // Дата и время последнего обновления сообщения
}

// Добавляем поддержку вставки данных в базу (Insertable)
#[derive(Insertable)]
#[table_name = "forum_posts"]
pub struct NewForumPost {
    pub topic_id: i32,
    pub user_id: i32,
    pub content: String,
}

// Это описание структуры "forum_posts" и функций, которые могут пригодиться
impl ForumPost {
// Функция для создания нового сообщения
pub fn create_forum_post(new_post: NewForumPost, connection: &PgConnection) -> Result<ForumPost, diesel::result::Error> {
    diesel::insert_into(forum_posts::table)
        .values(&new_post)
        .get_result(connection)
}

// Функция для получения сообщения по его идентификатору
pub fn get_post_by_id(post_id: i32, connection: &PgConnection) -> Result<Option<ForumPost>, diesel::result::Error> {
    forum_posts::table.find(post_id).first(connection).optional()
}

//Получение списка сообщений по идентификатору темы

pub fn get_posts_by_topic_id(topic_id: i32, connection: &PgConnection) -> Result<Vec<ForumPost>, diesel::result::Error> {
    forum_posts::table
        .filter(forum_posts::topic_id.eq(topic_id))
        .load::<ForumPost>(connection)
}

//Обновление содержания сообщения по его идентификатору

pub fn update_post_content(post_id: i32, updated_content: &str, connection: &PgConnection) -> Result<ForumPost, diesel::result::Error> {
    diesel::update(forum_posts::table.find(post_id))
        .set(forum_posts::content.eq(updated_content))
        .get_result(connection)
}

//Удаление сообщения по его идентификатору

pub fn delete_post_by_id(post_id: i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
    diesel::delete(forum_posts::table.find(post_id))
    .execute(connection)?;
    Ok(())
}

//Получение количества сообщений, созданных пользователем

pub fn get_post_count_by_user_id(user_id: i32, connection: &PgConnection) -> Result<i64, diesel::result::Error> {
    forum_posts::table
        .filter(forum_posts::user_id.eq(user_id))
        .count()
        .get_result(connection)
}

//Поиск сообщений по ключевому слову в содержании

pub fn search_posts_by_keyword(keyword: &str, connection: &PgConnection) -> Result<Vec<ForumPost>, diesel::result::Error> {
    forum_posts::table
        .filter(forum_posts::content.ilike(format!("%{}%", keyword)))
        .load::<ForumPost>(connection)
}

//Получение списка последних сообщений на форуме

pub fn get_latest_posts(limit: i64, connection: &PgConnection) -> Result<Vec<ForumPost>, diesel::result::Error> {
    forum_posts::table
        .order_by(forum_posts::created_at.desc())
        .limit(limit)
        .load::<ForumPost>(connection)
}

//Получение последних сообщений пользователя

use diesel::dsl::sql;
use diesel::sql_types::Integer;

pub fn get_latest_posts_by_user(user_id: i32, limit: i64, connection: &PgConnection) -> Result<Vec<ForumPost>, diesel::result::Error> {
    forum_posts::table
        .filter(forum_posts::user_id.eq(user_id))
        .order_by(forum_posts::created_at.desc())
        .limit(limit)
        .load::<ForumPost>(connection)
}

//Получение списка сообщений с наибольшим количеством лайков

pub fn get_posts_with_most_likes(limit: i64, connection: &PgConnection) -> Result<Vec<(ForumPost, i64)>, diesel::result::Error> {
    use diesel::dsl::count;

    forum_posts::table
        .inner_join(likes::table.on(likes::post_id.eq(forum_posts::id)))
        .group_by(forum_posts::id)
        .order_by(count(likes::id).desc())
        .limit(limit)
        .select((forum_posts::all_columns, count(likes::id)))
        .load(connection)
}

//Получение списка сообщений, на которые пользователь оставил лайк

pub fn get_posts_user_liked(user_id: i32, connection: &PgConnection) -> Result<Vec<ForumPost>, diesel::result::Error> {
    use crate::schema::likes;

    forum_posts::table
        .inner_join(likes::table.on(likes::post_id.eq(forum_posts::id)))
        .filter(likes::user_id.eq(user_id))
        .load::<ForumPost>(connection)
}
}
//---------------------------------------------------------------------------------------------------------------------------
// Также удостоверьтесь, что у вас есть модуль schema с описанием схемы базы данных и таблицей forum_likes.
// Подключаем необходимые модули
use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::forum_likes; 
// Создаем структуру "forum_likes" (Лайки к сообщениям на форуме)
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ForumLike {
    pub id: i32,                     // Идентификатор лайка
    pub post_id: i32,                // Идентификатор сообщения, к которому поставлен лайк
    pub user_id: i32,                // Идентификатор пользователя, поставившего лайк
    pub created_at: NaiveDateTime,   // Дата и время поставки лайка
}

// Добавляем поддержку вставки данных в базу (Insertable)
#[derive(Insertable)]
#[table_name = "forum_likes"]
pub struct NewForumLike {
    pub post_id: i32,
    pub user_id: i32,
}
impl ForumLike {
// Это описание структуры "forum_likes" и функций, которые могут пригодиться

// Функция для создания нового лайка
pub fn create_forum_like(new_like: NewForumLike, connection: &PgConnection) -> Result<ForumLike, diesel::result::Error> {
    diesel::insert_into(forum_likes::table)
        .values(&new_like)
        .get_result(connection)
}

//Получение количества лайков для определенного сообщения

pub fn get_like_count_by_post_id(post_id: i32, connection: &PgConnection) -> Result<i64, diesel::result::Error> {
    forum_likes::table
        .filter(forum_likes::post_id.eq(post_id))
        .count()
        .get_result(connection)
}
}
//----------------------------------------------------------------------------------------------------------------------------


use crate::schema::forum_subscriptions; 

use chrono::NaiveDateTime;
use diesel::prelude::*;

// Структура "forum_subscriptions" для описания подписок на темы форума
#[derive(Queryable)]
pub struct ForumSubscription {
    pub id: i32,                // Идентификатор подписки
    pub user_id: i32,           // Идентификатор пользователя, подписавшегося
    pub topic_id: i32,          // Идентификатор темы, на которую подписан
    pub created_at: NaiveDateTime, // Дата и время подписки
}

use crate::schema::forum_subscriptions; // Замените на имя вашего модуля schema

#[derive(Insertable)]
#[table_name = "forum_subscriptions"]
pub struct NewForumSubscription {
    pub user_id: i32,           // Идентификатор пользователя, подписавшегося
    pub topic_id: i32,          // Идентификатор темы, на которую подписан
    pub created_at: NaiveDateTime, // Дата и время подписки
}

impl NewForumSubscription {
    pub fn new(user_id: i32, topic_id: i32, created_at: NaiveDateTime) -> Self {
        NewForumSubscription {
            user_id,
            topic_id,
            created_at,
        }
    }
}

//Метод для получения списка подписок пользователя

use crate::schema::forum_subscriptions; // Замените на имя вашего модуля schema

impl ForumSubscription {
    // Метод для получения списка подписок пользователя по его идентификатору
    pub fn get_subscriptions_by_user_id(
        conn: &PgConnection,
        user_id: i32,
    ) -> Result<Vec<ForumSubscription>, diesel::result::Error> {
        forum_subscriptions::table
            .filter(forum_subscriptions::user_id.eq(user_id))
            .load::<ForumSubscription>(conn)
    }
}


impl ForumSubscription {
    // Метод для отмены подписки пользователя на тему
    pub fn unsubscribe(
        conn: &PgConnection,
        user_id: i32,
        topic_id: i32,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(
            forum_subscriptions::table
                .filter(forum_subscriptions::user_id.eq(user_id))
                .filter(forum_subscriptions::topic_id.eq(topic_id)),
        )
        .execute(conn)
    }
}

use crate::schema::forum_subscriptions; 
use diesel::dsl::sql;

impl ForumSubscription {
    // Метод для получения списка пользователей, подписанных на тему по ее идентификатору
    pub fn get_users_subscribed_to_topic(
        conn: &PgConnection,
        topic_id: i32,
    ) -> Result<Vec<i32>, diesel::result::Error> {
        forum_subscriptions::table
            .filter(forum_subscriptions::topic_id.eq(topic_id))
            .select(sql("user_id"))
            .distinct()
            .load::<i32>(conn)
    }
}


impl ForumSubscription {
    // Метод для получения списка всех подписок на форуме
    pub fn get_all_subscriptions(
        conn: &PgConnection,
    ) -> Result<Vec<ForumSubscription>, diesel::result::Error> {
        forum_subscriptions::table.load::<ForumSubscription>(conn)
    }
}

//------------------------------------------------------------------------------------------------------

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::pg::data_types::PgTimestamp;
use crate::schema::*;

// Создаем структуру "forum_reports"
#[derive(Debug, Queryable)]
pub struct ForumReport {
    pub id: i32,
    pub post_id: i32,
    pub user_id: i32,
    pub reason: String,
    pub description: String,
    pub status: String,
    pub created_at: NaiveDateTime,
}

// Определяем методы для структуры "forum_reports"
impl ForumReport {
    // Конструктор для создания новой жалобы
    pub fn new(post_id: i32, user_id: i32, reason: String, description: String) -> Self {
        ForumReport {
            id: 0,  // Это значение будет автоматически установлено в базе данных
            post_id,
            user_id,
            reason,
            description,
            status: String::from("в ожидании"),  // Устанавливаем начальный статус
            created_at: chrono::Local::now().naive_utc(),  // Устанавливаем текущую дату и время
        }
    }
}

//------------------------------------------------------------------------------------------------------




