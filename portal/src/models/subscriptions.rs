use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::insert_into;

#[derive(Debug, Queryable)]
pub struct Subscription {
    pub id: i32,
    pub user_id: i32,
    pub object_id: i32,
    pub object_type: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub types: i16,
    pub link: String,
    pub image: String,
    pub category_id: i32,
    pub lists: i16,
    pub members: i32,
    pub description: Option<String>,
}

//Создание подписки (Create)

impl Subscription {
    pub fn create_subscription(
        conn: &PgConnection,
        user_id: i32,
        object_id: i32,
        object_type: &str,
        status: &str,
        name: &str,
        types: i16,
        link: &str,
        image: &str,
        category_id: i32,
        lists: i32,
        members: i16,
        description: &str,
    ) -> Result<Subscription, diesel::result::Error> {
        let new_subscription = NewSubscription {
            user_id,
            object_id,
            object_type,
            status,
            name,
            types,
            link,
            image,
            category_id,
            lists,
            members,
            description,
        };

        let subscription = insert_into(subscriptions::table)
            .values(&new_subscription)
            .get_result(conn)?;

        Ok(subscription)
    }
}


//Чтение подписки (Read)

impl Subscription {
    pub fn get_subscription_by_id(
        conn: &PgConnection,
        subscription_id: i32,
    ) -> Result<Option<Subscription>, diesel::result::Error> {
        let subscription = subscriptions::table
            .filter(subscriptions::id.eq(subscription_id))
            .first(conn)
            .optional()?;

        Ok(subscription)
    }

 
}

//Обновление подписки (Update)

impl Subscription {
    pub fn update_subscription(
        conn: &PgConnection,
        subscription_id: i32,
        new_status: &str,
    ) -> Result<usize, diesel::result::Error> {
        let updated_rows = diesel::update(subscriptions::table)
            .filter(subscriptions::id.eq(subscription_id))
            .set(subscriptions::status.eq(new_status))
            .execute(conn)?;

        Ok(updated_rows)
    }
}


//Удаление подписки (Delete)

impl Subscription {
    pub fn delete(&self, connection: &PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(subscriptions::table.find(self.id)).execute(connection)
    }
}

//Получение всех подписок пользователя

impl Subscription {
    pub fn get_subscriptions_by_user(user_id: i32, connection: &PgConnection) -> Result<Vec<Subscription>, diesel::result::Error> {
        subscriptions::table.filter(subscriptions::user_id.eq(user_id)).load(connection)
    }
}


//Поиск подписок по имени или описанию

impl Subscription {
    pub fn search_subscriptions(keyword: &str, connection: &PgConnection) -> Result<Vec<Subscription>, diesel::result::Error> {
        subscriptions::table.filter(
            subscriptions::name.ilike(format!("%{}%", keyword)).or(
                subscriptions::description.ilike(format!("%{}%", keyword)),
            ),
        ).load(connection)
    }
}

//Фильтрация подписок по статусу

impl Subscription {
    pub fn filter_subscriptions_by_status(status: &str, connection: &PgConnection) -> Result<Vec<Subscription>, diesel::result::Error> {
        subscriptions::table.filter(subscriptions::status.eq(status)).load(connection)
    }
}


//Подсчет количества подписок определенного типа

impl Subscription {
    pub fn count_subscriptions_by_type(object_type: &str, connection: &PgConnection) -> Result<i64, diesel::result::Error> {
        subscriptions::table.filter(subscriptions::object_type.eq(object_type)).count().get_result(connection)
    }
}

//Подсчет активных подписок пользователя

impl Subscription {
    pub fn count_active_subscriptions_by_user(user_id: i32, connection: &PgConnection) -> Result<i64, diesel::result::Error> {
        subscriptions::table.filter(
            subscriptions::user_id.eq(user_id)
            .and(subscriptions::status.eq("active")) // Подставь нужный статус
        ).count().get_result(connection)
    }
}


//Получение популярных объектов с наибольшим количеством подписчиков

impl Subscription {
    pub fn get_popular_objects(limit: i64, connection: &PgConnection) -> Result<Vec<(String, i64)>, diesel::result::Error> {
        subscriptions::table
            .select((subscriptions::name, diesel::dsl::count_star()))
            .group_by(subscriptions::name)
            .order(diesel::dsl::count_star().desc())
            .limit(limit)
            .load(connection)
    }
}
