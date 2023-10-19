use chrono::NaiveDateTime; // Для работы с датой и временем
use diesel::prelude::*; // Для работы с базой данных
use serde::{Deserialize, Serialize}; // Для сериализации и десериализации данных


//-------------------------------------------------------------------------------------
// Структура для таблицы "event_themes"
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct EventTheme {
    pub id: i32, // Идентификатор тематики
    pub name: String, // Название тематики
    pub parent_id: Option<i32>, // Идентификатор родительской тематики
}

// Структура для создания новой тематики
#[derive(Debug, Deserialize, Insertable)]
#[table_name = "event_themes"]
pub struct NewEventTheme {
    pub name: String,
    pub parent_id: Option<i32>,
}


impl EventTheme {
    // Метод для создания новой тематики
    pub fn create_new_theme(
        conn: &PgConnection,
        new_theme: NewEventTheme,
    ) -> QueryResult<EventTheme> {
        diesel::insert_into(event_themes::table)
            .values(&new_theme)
            .get_result(conn)
    }

    // Метод для обновления информации о тематике
    pub fn update_theme_info(
        conn: &PgConnection,
        theme_id: i32,
        updated_theme: NewEventTheme,
    ) -> QueryResult<EventTheme> {
        diesel::update(event_themes::table.find(theme_id))
            .set(&updated_theme)
            .get_result(conn)
    }

    // Метод для удаления тематики по идентификатору
    pub fn delete_theme_by_id(conn: &PgConnection, theme_id: i32) -> QueryResult<()> {
        diesel::delete(event_themes::table.find(theme_id)).execute(conn)?;
        Ok(())
    }

    // Дополнительный метод для поиска тематики по имени
    pub fn find_theme_by_name(conn: &PgConnection, name: &str) -> QueryResult<Option<EventTheme>> {
        event_themes::table
            .filter(event_themes::name.eq(name))
            .first(conn)
            .optional()
    }
}

//-------------------------------------------------------------------------------------

// Структура для таблицы "events"
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Event {
    pub id: i32, // Идентификатор события
    pub title: String, // Заголовок события
    pub description: String, // Описание события
    pub category: Option<String>, // Категория события
    pub date_start: NaiveDateTime, // Дата и время начала события
    pub date_end: NaiveDateTime, // Дата и время окончания события
    pub location: Option<String>, // Местоположение события
    pub ticket_price: Option<f64>, // Стоимость билета
    pub currency: Option<String>, // Валюта
    pub contact_name: Option<String>, // Имя контактного лица
    pub contact_phone: Option<String>, // Телефон контактного лица
    pub contact_email: Option<String>, // Email контактного лица
    pub image: Option<String>, // Ссылка на изображение события
    pub is_published: bool, // Флаг публикации события
    pub created_at: NaiveDateTime, // Дата и время создания события
    pub updated_at: NaiveDateTime, // Дата и время последнего обновления события
}

// Структура для создания нового события
#[derive(Debug, Deserialize, Insertable)]
#[table_name = "events"]
pub struct NewEvent {
    pub title: String,
    pub description: String,
    pub category: Option<String>,
    pub date_start: NaiveDateTime,
    pub date_end: NaiveDateTime,
    pub location: Option<String>,
    pub ticket_price: Option<f64>,
    pub currency: Option<String>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub image: Option<String>,
    pub is_published: bool,
}

impl Event {
    // Метод для создания нового события
    pub fn create_new_event(
        conn: &PgConnection,
        new_event: NewEvent,
    ) -> QueryResult<Event> {
        diesel::insert_into(events::table)
            .values(&new_event)
            .get_result(conn)
    }

    // Метод для обновления информации о событии
    pub fn update_event_info(
        conn: &PgConnection,
        event_id: i32,
        updated_event: NewEvent,
    ) -> QueryResult<Event> {
        diesel::update(events::table.find(event_id))
            .set(&updated_event)
            .get_result(conn)
    }

    // Метод для удаления события по идентификатору
    pub fn delete_event_by_id(conn: &PgConnection, event_id: i32) -> QueryResult<()> {
        diesel::delete(events::table.find(event_id)).execute(conn)?;
        Ok(())
    }

    // Дополнительный метод для поиска события по названию
    pub fn find_event_by_title(
        conn: &PgConnection,
        title: &str,
    ) -> QueryResult<Option<Event>> {
        events::table
            .filter(events::title.eq(title))
            .first(conn)
            .optional()
    }
}

//Метод для получения всех событий, начинающихся после определенной даты

pub fn get_upcoming_events(
    conn: &PgConnection,
    date: NaiveDateTime,
) -> QueryResult<Vec<Event>> {
    use crate::schema::events::dsl::*;
    events.filter(date_start.gt(date)).load(conn)
}

//Метод для получения всех событий в определенной категории:
pub fn get_events_in_category(
    conn: &PgConnection,
    category: &str,
) -> QueryResult<Vec<Event>> {
    use crate::schema::events::dsl::*;
    events.filter(category.eq(category)).load(conn)
}

//Метод для получения всех событий, созданных определенным пользователем

pub fn get_events_by_user(
    conn: &PgConnection,
    user_id: i32,
) -> QueryResult<Vec<Event>> {
    use crate::schema::events::dsl::*;
    events.filter(user_id.eq(user_id)).load(conn)
}

//Метод для получения всех событий, отсортированных по дате начала в порядке возрастания

pub fn get_events_sorted_by_date_asc(conn: &PgConnection) -> QueryResult<Vec<Event>> {
    use crate::schema::events::dsl::*;
    events.order(date_start.asc()).load(conn)
}

//Метод для получения всех событий, отсортированных по цене билета в порядке возрастания

pub fn get_events_sorted_by_ticket_price_asc(conn: &PgConnection) -> QueryResult<Vec<Event>> {
    use crate::schema::events::dsl::*;
    events
        .filter(ticket_price.is_not_null())
        .order(ticket_price.asc())
        .load(conn)
}

//Метод для поиска всех событий, которые начинаются позже заданной даты и имеют определенную категорию

pub fn get_events_after_date_in_category(
    conn: &PgConnection,
    date: NaiveDateTime,
    category: &str,
) -> QueryResult<Vec<Event>> {
    use crate::schema::events::dsl::*;
    events
        .filter(date_start.gt(date))
        .filter(category.eq(category))
        .load(conn)
}

//-------------------------------------------------------------------------------------

// Структура для таблицы "user_events"
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct UserEvent {
    pub id: i32, // Идентификатор события пользователя
    pub user_id: i32, // Идентификатор пользователя, создавшего событие
    pub title: String, // Заголовок события
    pub description: String, // Описание события
    pub category: Option<String>, // Категория события
    pub date_start: NaiveDateTime, // Дата и время начала события
    pub date_end: NaiveDateTime, // Дата и время окончания события
    pub location: Option<String>, // Местоположение события
    pub ticket_price: Option<f64>, // Стоимость билета
    pub currency: Option<String>, // Валюта
    pub contact_name: Option<String>, // Имя контактного лица
    pub contact_phone: Option<String>, // Телефон контактного лица
    pub contact_email: Option<String>, // Email контактного лица
    pub image: Option<String>, // Ссылка на изображение события
    pub is_published: bool, // Флаг публикации события
    pub created_at: NaiveDateTime, // Дата и время создания события
    pub updated_at: NaiveDateTime, // Дата и время последнего обновления события
}

use diesel::insert_into;
use diesel::prelude::*;

// Структура для создания нового пользовательского события
#[derive(Debug, Deserialize, Insertable)]
#[table_name = "user_events"]
pub struct NewUserEvent {
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub category: Option<String>,
    pub date_start: NaiveDateTime,
    pub date_end: NaiveDateTime,
    pub location: Option<String>,
    pub ticket_price: Option<f64>,
    pub currency: Option<String>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub image: Option<String>,
    pub is_published: bool,
}


impl UserEvent {
    // Метод для создания нового пользовательского события
    pub fn create_new_user_event(
        conn: &PgConnection,
        new_user_event: NewUserEvent,
    ) -> QueryResult<UserEvent> {
        insert_into(user_events::table)
            .values(&new_user_event)
            .get_result(conn)
    }

    // Метод для обновления информации о пользовательском событии
    pub fn update_user_event_info(
        conn: &PgConnection,
        user_event_id: i32,
        updated_user_event: NewUserEvent,
    ) -> QueryResult<UserEvent> {
        diesel::update(user_events::table.find(user_event_id))
            .set(&updated_user_event)
            .get_result(conn)
    }

    // Метод для удаления пользовательского события по идентификатору
    pub fn delete_user_event_by_id(conn: &PgConnection, user_event_id: i32) -> QueryResult<()> {
        diesel::delete(user_events::table.find(user_event_id)).execute(conn)?;
        Ok(())
    }
}



//-------------------------------------------------------------------------------------

// Структура для таблицы "event_calendar"
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct EventCalendar {
    pub id: i32, // Идентификатор даты события
    pub event_id: i32, // Идентификатор события
    pub date: NaiveDate, // Дата события
    pub time_start: Option<NaiveTime>, // Время начала события
    pub time_end: Option<NaiveTime>, // Время окончания события
}

use diesel::insert_into;
use diesel::prelude::*;

// Структура для создания новой записи в календаре событий
#[derive(Debug, Deserialize, Insertable)]
#[table_name = "event_calendar"]
pub struct NewEventCalendar {
    pub event_id: i32,
    pub date: NaiveDate,
    pub time_start: Option<NaiveTime>,
    pub time_end: Option<NaiveTime>,
}


impl EventCalendar {
    // Метод для создания новой записи в календаре событий
    pub fn create_new_event_calendar(
        conn: &PgConnection,
        new_event_calendar: NewEventCalendar,
    ) -> QueryResult<EventCalendar> {
        insert_into(event_calendar::table)
            .values(&new_event_calendar)
            .get_result(conn)
    }

    // Метод для обновления информации о записи в календаре событий
    pub fn update_event_calendar_info(
        conn: &PgConnection,
        event_calendar_id: i32,
        updated_event_calendar: NewEventCalendar,
    ) -> QueryResult<EventCalendar> {
        diesel::update(event_calendar::table.find(event_calendar_id))
            .set(&updated_event_calendar)
            .get_result(conn)
    }

    // Метод для удаления записи из календаря событий по идентификатору
    pub fn delete_event_calendar_by_id(
        conn: &PgConnection,
        event_calendar_id: i32,
    ) -> QueryResult<()> {
        diesel::delete(event_calendar::table.find(event_calendar_id)).execute(conn)?;
        Ok(())
    }

//Метод для получения всех событий, запланированных на определенную дату


pub fn get_events_scheduled_for_date(
    conn: &PgConnection,
    date: NaiveDate,
) -> QueryResult<Vec<EventCalendar>> {
    use crate::schema::event_calendar::dsl::*;
    event_calendar.filter(date.eq(date)).load(conn)
}

//Метод для получения всех событий, запланированных на определенную дату и в определенной категории

pub fn get_events_scheduled_for_date_in_category(
    conn: &PgConnection,
    date: NaiveDate,
    category: &str,
) -> QueryResult<Vec<EventCalendar>> {
    use crate::schema::event_calendar::dsl::*;
    event_calendar
        .filter(date.eq(date))
        .inner_join(events::table)
        .filter(events::category.eq(category))
        .load(conn)
}

//Метод для получения всех событий, запланированных в определенном временном интервале и в определенной категории

pub fn get_events_scheduled_in_time_interval_in_category(
    conn: &PgConnection,
    start_time: NaiveTime,
    end_time: NaiveTime,
    category: &str,
) -> QueryResult<Vec<EventCalendar>> {
    use crate::schema::event_calendar::dsl::*;
    event_calendar
        .filter(time_start.le(end_time).and(time_end.ge(start_time)))
        .inner_join(events::table)
        .filter(events::category.eq(category))
        .load(conn)
}



//-------------------------------------------------------------------------------------
use crate::schema::event_attendees;
use diesel::pg::PgConnection;
use diesel::prelude::*;
// Структура для таблицы "event_attendees"
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct EventAttendee {
    pub id: i32, // Идентификатор посетителя
    pub event_id: i32, // Идентификатор события, которое посещает
    pub user_id: i32, // Идентификатор пользователя, посетившего событие
    pub created_at: NaiveDateTime, // Дата и время посещения события
}

use diesel::insert_into;
use diesel::prelude::*;

// Структура для создания новой записи о посетителе события
#[derive(Debug, Deserialize, Insertable)]
#[table_name = "event_attendees"]
pub struct NewEventAttendee {
    pub event_id: i32,
    pub user_id: i32,
}


impl EventAttendee {
    // Метод для создания новой записи о посетителе события
    pub fn create_new_event_attendee(
        conn: &PgConnection,
        new_event_attendee: NewEventAttendee,
    ) -> QueryResult<EventAttendee> {
        insert_into(event_attendees::table)
            .values(&new_event_attendee)
            .get_result(conn)
    }

    // Метод для удаления записи о посетителе события по идентификатору
    pub fn delete_event_attendee_by_id(
        conn: &PgConnection,
        event_attendee_id: i32,
    ) -> QueryResult<()> {
        diesel::delete(event_attendees::table.find(event_attendee_id)).execute(conn)?;
        Ok(())
    }

    // Метод для получения всех записей о посетителях события по идентификатору события
    pub fn get_event_attendees_for_event(
        conn: &PgConnection,
        event_id: i32,
    ) -> QueryResult<Vec<EventAttendee>> {
        use crate::schema::event_attendees::dsl::*;
        event_attendees.filter(event_id.eq(event_id)).load(conn)
    }
}

//Метод для получения всех событий, на которые ходил определенный пользователь
pub fn get_events_attended_by_user(
    conn: &PgConnection,
    user_id: i32,
) -> QueryResult<Vec<EventAttendee>> {
    use crate::schema::event_attendees::dsl::*;
    event_attendees.filter(user_id.eq(user_id)).load(conn)
}

//Метод для получения всех пользователей, посетивших определенное событие

pub fn get_users_attending_event(
    conn: &PgConnection,
    event_id: i32,
) -> QueryResult<Vec<EventAttendee>> {
    use crate::schema::event_attendees::dsl::*;
    event_attendees.filter(event_id.eq(event_id)).load(conn)
}

//Метод для получения количества посетителей для определенного события

pub fn get_attendee_count_for_event(
    conn: &PgConnection,
    event_id: i32,
) -> QueryResult<i64> {
    use crate::schema::event_attendees::dsl::*;
    event_attendees.filter(event_id.eq(event_id)).count().get_result(conn)
}


//-------------------------------------------------------------------------------------
use crate::schema::event_subscribers;
use diesel::pg::PgConnection;
use diesel::prelude::*;
// Структура для таблицы "event_subscribers"
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct EventSubscriber {
    pub id: i32, // Идентификатор подписки
    pub event_id: i32, // Идентификатор события, на которое подписан
    pub user_id: i32, // Идентификатор пользователя, подписавшегося
    pub created_at: NaiveDateTime, // Дата и время создания подписки
}

use diesel::insert_into;
use diesel::prelude::*;

// Структура для создания новой записи подписчика события
#[derive(Debug, Deserialize, Insertable)]
#[table_name = "event_subscribers"]
pub struct NewEventSubscriber {
    pub event_id: i32,
    pub user_id: i32,
}


impl EventSubscriber {
    // Метод для создания новой записи подписчика события
    pub fn create_new_event_subscriber(
        conn: &PgConnection,
        new_event_subscriber: NewEventSubscriber,
    ) -> QueryResult<EventSubscriber> {
        insert_into(event_subscribers::table)
            .values(&new_event_subscriber)
            .get_result(conn)
    }

    // Метод для удаления записи подписчика события по идентификатору
    pub fn delete_event_subscriber_by_id(
        conn: &PgConnection,
        event_subscriber_id: i32,
    ) -> QueryResult<()> {
        diesel::delete(event_subscribers::table.find(event_subscriber_id)).execute(conn)?;
        Ok(())
    }

    // Метод для получения всех записей подписчиков для определенного события
    pub fn get_event_subscribers_for_event(
        conn: &PgConnection,
        event_id: i32,
    ) -> QueryResult<Vec<EventSubscriber>> {
        use crate::schema::event_subscribers::dsl::*;
        event_subscribers.filter(event_id.eq(event_id)).load(conn)
    }
}

//Метод для получения всех событий, на которые подписан определенный пользователь

pub fn get_events_subscribed_by_user(
    conn: &PgConnection,
    user_id: i32,
) -> QueryResult<Vec<EventSubscriber>> {
    use crate::schema::event_subscribers::dsl::*;
    event_subscribers.filter(user_id.eq(user_id)).load(conn)
}

//Метод для получения всех пользователей, подписанных на определенное событие

pub fn get_users_subscribed_to_event(
    conn: &PgConnection,
    event_id: i32,
) -> QueryResult<Vec<EventSubscriber>> {
    use crate::schema::event_subscribers::dsl::*;
    event_subscribers.filter(event_id.eq(event_id)).load(conn)
}


//Метод для получения количества подписчиков для определенного события

pub fn get_subscriber_count_for_event(
    conn: &PgConnection,
    event_id: i32,
) -> QueryResult<i64> {
    use crate::schema::event_subscribers::dsl::*;
    event_subscribers.filter(event_id.eq(event_id)).count().get_result(conn)
}


//-------------------------------------------------------------------------------------
