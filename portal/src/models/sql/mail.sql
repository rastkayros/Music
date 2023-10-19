




--______________________________________________________________________________file__________CITY_________________rs________________________


-- Создание таблицы "cities" для хранения информации о городах
CREATE TABLE IF NOT EXISTS cities (
    id SERIAL PRIMARY KEY,             -- Идентификатор города
    name VARCHAR(255) NOT NULL,        -- Название города
    population INT,                    -- Численность населения (может отсутствовать)
    area_sq_km NUMERIC,                -- Площадь в квадратных километрах (может отсутствовать)
    mayor VARCHAR(255),                -- Имя мэра (может отсутствовать)
    founding_date DATE                 -- Дата основания (может отсутствовать)
);

-- Создание таблицы "districts" для хранения информации о районах
CREATE TABLE IF NOT EXISTS districts (
    id SERIAL PRIMARY KEY,             -- Идентификатор района
    city_id INT NOT NULL,              -- Идентификатор города
    name VARCHAR(255) NOT NULL,        -- Название района
    population INT NOT NULL,           -- Численность населения в районе
    area_sq_km NUMERIC,                -- Площадь района в квадратных километрах (может отсутствовать)
);

-- Создание таблицы "streets" для хранения информации о улицах
CREATE TABLE IF NOT EXISTS streets (
    id SERIAL PRIMARY KEY,             -- Идентификатор улицы
    district_id INT NOT NULL,          -- Идентификатор района
    name VARCHAR(255) NOT NULL,        -- Название улицы
    length_km NUMERIC,                -- Длина улицы в километрах (может отсутствовать)
    surface_type VARCHAR(255),        -- Тип покрытия улицы (может отсутствовать)
    speed_limit INT                    -- Ограничение скорости на улице (может отсутствовать)
);

-- Создание таблицы "organizations" для хранения информации организациях
CREATE TABLE IF NOT EXISTS organizations (
    id SERIAL PRIMARY KEY,             -- Идентификатор организации
    street_id INT NOT NULL,            -- Идентификатор улицы
    name VARCHAR(255) NOT NULL,        -- Название организации
    description TEXT,                  -- Описание организации (может отсутствовать)
    contact_info VARCHAR(255),         -- Контактная информация (может отсутствовать)
    operating_hours VARCHAR(255),      -- Часы работы (может отсутствовать)
    website VARCHAR(255),              -- Веб-сайт (может отсутствовать)
    email VARCHAR(255),                -- Электронная почта (может отсутствовать)
    phone VARCHAR(255),                -- Телефон (может отсутствовать)
    category VARCHAR(255),             -- Категория организации (может отсутствовать)
    rating NUMERIC                     -- Рейтинг организации (может отсутствовать)
);

-- Создание таблицы "goods" для хранения информации о товарах и услугах
CREATE TABLE IF NOT EXISTS goods (
    id SERIAL PRIMARY KEY,             -- Идентификатор товара или услуги
    street_id INT NOT NULL,            -- Идентификатор улицы
    name VARCHAR(255) NOT NULL,        -- Название товара или услуги
    description TEXT                   -- Описание товара (может отсутствовать)
);


-- Создание таблицы "user_points" для хранения информации о точках, отмеченных пользователями
CREATE TABLE IF NOT EXISTS user_points (
    id SERIAL PRIMARY KEY,              -- Идентификатор точки отмеченной пользователем
    street_id INT NOT NULL,             -- Идентификатор улицы
    name VARCHAR(255) NOT NULL,         -- Название точки
    description TEXT,                   -- Описание точки (может отсутствовать)
    category VARCHAR(255),              -- Категория точки (может отсутствовать)
    rating NUMERIC,                    -- Рейтинг точки (может отсутствовать)
    photo BYTEA,                        -- Фотография точки в виде BLOB (может отсутствовать)
    latitude DOUBLE PRECISION NOT NULL, -- Широта точки
    longitude DOUBLE PRECISION NOT NULL,-- Долгота точки
    created_at TIMESTAMP NOT NULL,      -- Дата и время создания точки
    user_id INT NOT NULL,               -- Идентификатор пользователя, создавшего точку
    external_link VARCHAR(255),         -- Ссылка на внешний ресурс (может отсутствовать)
    average_rating FLOAT,               -- Средний рейтинг точки (может отсутствовать)
    reviews TEXT[]                      -- Отзывы о точке (массив, может быть пустым)
);

-- Создание таблицы "wifi_charging" для хранения информации о бесплатных Wi-Fi точках и зарядках
CREATE TABLE IF NOT EXISTS wifi_charging (
    id SERIAL PRIMARY KEY,              -- Идентификатор точки
    street_id INT NOT NULL,             -- Идентификатор улицы
    name VARCHAR(255) NOT NULL,         -- Название точки
    description TEXT,                   -- Описание точки (может отсутствовать)
    services VARCHAR(255),              -- Доступные услуги (может отсутствовать)
    location POINT,                     -- Координаты точки на карте (может отсутствовать)
    rating NUMERIC,                    -- Рейтинг точки (может отсутствовать)
);

-- Добавление индексов для улучшения производительности запросов
CREATE INDEX IF NOT EXISTS idx_user_points_street_id ON user_points (street_id);
CREATE INDEX IF NOT EXISTS idx_user_points_user_id ON user_points (user_id);
CREATE INDEX IF NOT EXISTS idx_wifi_charging_street_id ON wifi_charging (street_id);

--_____________________________________________________________________________file_________________________AFISHA___________________________________
-- Создание таблицы "event_themes" для хранения тематик событий
CREATE TABLE IF NOT EXISTS event_themes (
    id SERIAL PRIMARY KEY,        -- Идентификатор тематики
    name VARCHAR(255) NOT NULL,   -- Название тематики
    parent_id INT                 -- Идентификатор родительской тематики
);

-- Создание таблицы "events" для хранения информации о событиях
CREATE TABLE IF NOT EXISTS events (
    id SERIAL PRIMARY KEY,                 -- Идентификатор события
    title VARCHAR(255) NOT NULL,           -- Заголовок события
    description TEXT NOT NULL,             -- Описание события
    category VARCHAR(255),                 -- Категория события (необязательное)
    date_start TIMESTAMP NOT NULL,         -- Дата и время начала события
    date_end TIMESTAMP NOT NULL,           -- Дата и время окончания события
    location VARCHAR(255),                 -- Местоположение события (необязательное)
    ticket_price NUMERIC,                 -- Стоимость билета (необязательное)
    currency VARCHAR(255),                -- Валюта (необязательное)
    contact_name VARCHAR(255),            -- Имя контактного лица (необязательное)
    contact_phone VARCHAR(255),           -- Телефон контактного лица (необязательное)
    contact_email VARCHAR(255),           -- Email контактного лица (необязательное)
    image VARCHAR(255),                   -- Ссылка на изображение события (необязательное)
    is_published BOOLEAN NOT NULL,        -- Флаг публикации события
    created_at TIMESTAMP NOT NULL,        -- Дата и время создания события
    updated_at TIMESTAMP NOT NULL         -- Дата и время последнего обновления события
);

-- Создание таблицы "user_events" для хранения информации о событиях пользователя
CREATE TABLE IF NOT EXISTS user_events (
    id SERIAL PRIMARY KEY,                 -- Идентификатор события пользователя
    user_id INT NOT NULL,                  -- Идентификатор пользователя, создавшего событие
    title VARCHAR(255) NOT NULL,           -- Заголовок события
    description TEXT NOT NULL,             -- Описание события
    category VARCHAR(255),                 -- Категория события (необязательное)
    date_start TIMESTAMP NOT NULL,         -- Дата и время начала события
    date_end TIMESTAMP NOT NULL,           -- Дата и время окончания события
    location VARCHAR(255),                 -- Местоположение события (необязательное)
    ticket_price NUMERIC,                 -- Стоимость билета (необязательное)
    currency VARCHAR(255),                -- Валюта (необязательное)
    contact_name VARCHAR(255),            -- Имя контактного лица (необязательное)
    contact_phone VARCHAR(255),           -- Телефон контактного лица (необязательное)
    contact_email VARCHAR(255),           -- Email контактного лица (необязательное)
    image VARCHAR(255),                   -- Ссылка на изображение события (необязательное)
    is_published BOOLEAN NOT NULL,        -- Флаг публикации события
    created_at TIMESTAMP NOT NULL,        -- Дата и время создания события
    updated_at TIMESTAMP NOT NULL         -- Дата и время последнего обновления события
);

-- Создание таблицы "event_calendar" для хранения дат событий
CREATE TABLE IF NOT EXISTS event_calendar (
    id SERIAL PRIMARY KEY,                 -- Идентификатор даты события
    event_id INT NOT NULL,                -- Идентификатор события
    date DATE NOT NULL,                   -- Дата события
    time_start TIME,                      -- Время начала события (необязательное)
    time_end TIME                         -- Время окончания события (необязательное)
);

-- Создание таблицы "event_attendees" для хранения информации о посетителях событий
CREATE TABLE IF NOT EXISTS event_attendees (
    id SERIAL PRIMARY KEY,                 -- Идентификатор посетителя
    event_id INT NOT NULL,                -- Идентификатор события, которое посещает
    user_id INT NOT NULL,                 -- Идентификатор пользователя, посетившего событие
    created_at TIMESTAMP NOT NULL         -- Дата и время посещения события
);

-- Создание таблицы "event_subscribers" для хранения информации о подписчиках событий
CREATE TABLE IF NOT EXISTS event_subscribers (
    id SERIAL PRIMARY KEY,                 -- Идентификатор подписки
    event_id INT NOT NULL,                -- Идентификатор события, на которое подписан
    user_id INT NOT NULL,                 -- Идентификатор пользователя, подписавшегося
    created_at TIMESTAMP NOT NULL         -- Дата и время создания подписки
);

-- Создание таблицы "advertisements" для хранения объявлений---------------------------------------file______Advertisement________rs
    id SERIAL PRIMARY KEY,              -- Идентификатор объявления
    user_id INT NOT NULL,               -- Идентификатор пользователя, разместившего объявление
    community_id INT,                   -- Идентификатор сообщества, если применимо
    title VARCHAR(255) NOT NULL,        -- Заголовок объявления
    description TEXT,                   -- Описание объявления (необязательное)
    category VARCHAR(255),              -- Категория объявления (необязательное)
    price NUMERIC,                      -- Цена, если применимо
    currency VARCHAR(255),              -- Валюта, если есть
    location VARCHAR(255),              -- Местоположение (необязательное)
    contact_name VARCHAR(255),          -- Имя контактного лица (необязательное)
    contact_phone VARCHAR(255),         -- Телефон контактного лица (необязательное)
    contact_email VARCHAR(255),         -- Email контактного лица (необязательное)
    images TEXT[],                      -- Ссылки на изображения объявления, массив
    video VARCHAR(255),                 -- Ссылка на видео объявления, если есть
    is_active BOOLEAN NOT NULL,         -- Флаг активности объявления
    created_at TIMESTAMP NOT NULL       -- Дата и время создания
);

-- Создание индекса на поле "user_id" для быстрого поиска объявлений пользователя
CREATE INDEX IF NOT EXISTS idx_user_id ON advertisements (user_id);

-- Создание индекса на поле "is_active" для быстрого поиска активных объявлений
CREATE INDEX IF NOT EXISTS idx_is_active ON advertisements (is_active);

--________________________________________________________________________-File __TAGS_____________________________________________
-- Создание таблицы "file_tags" для хранения тегов файлов
CREATE TABLE IF NOT EXISTS file_tags (
    id SERIAL PRIMARY KEY,      -- Уникальный идентификатор тега
    file_id INT NOT NULL,       -- Идентификатор файла, к которому привязан тег
    tag_id INT NOT NULL         -- Идентификатор собственно тега
);

-- Создание таблицы "files" для хранения информации о файлах
CREATE TABLE IF NOT EXISTS files (
    id SERIAL PRIMARY KEY,          -- Уникальный идентификатор файла
    user_id INT NOT NULL,           -- Идентификатор пользователя, загрузившего файл
    community_id INT,               -- Идентификатор сообщества, если применимо
    title VARCHAR(255) NOT NULL,    -- Заголовок файла
    description TEXT,               -- Описание файла (необязательное)
    file_type VARCHAR(255) NOT NULL,-- Тип файла (например, "изображение", "документ")
    file_url TEXT,                  -- URL файла (необязательное)
    is_private BOOLEAN NOT NULL,    -- Флаг приватности файла
    is_approved BOOLEAN,            -- Флаг одобрения файла администратором (необязательное)
    is_deleted BOOLEAN NOT NULL,    -- Флаг удаления файла
    uploaded_at TIMESTAMP NOT NULL  -- Дата и время загрузки файла
);

-- Создание таблицы "file_comments" для хранения комментариев к файлам
CREATE TABLE IF NOT EXISTS file_comments (
    id SERIAL PRIMARY KEY,       -- Уникальный идентификатор комментария
    file_id INT NOT NULL,        -- Идентификатор файла, к которому относится комментарий
    user_id INT NOT NULL,        -- Идентификатор пользователя, оставившего комментарий
    comment_text TEXT,           -- Текст комментария (необязательное)
    is_deleted BOOLEAN NOT NULL, -- Флаг удаления комментария
    created_at TIMESTAMP NOT NULL-- Дата и время создания комментария
);

-- Создание таблицы "file_likes" для хранения лайков к файлам
CREATE TABLE IF NOT EXISTS file_likes (
    id SERIAL PRIMARY KEY,      -- Уникальный идентификатор лайка
    file_id INT NOT NULL,       -- Идентификатор файла, к которому относится лайк
    user_id INT NOT NULL,       -- Идентификатор пользователя, поставившего лайк
    created_at TIMESTAMP NOT NULL-- Дата и время создания лайка
);

-- Создание таблицы "file_downloads" для хранения записей о скачиваниях файлов
CREATE TABLE IF NOT EXISTS file_downloads (
    id SERIAL PRIMARY KEY,      -- Уникальный идентификатор записи о скачивании
    file_id INT NOT NULL,       -- Идентификатор файла, который был скачан
    user_id INT NOT NULL,       -- Идентификатор пользователя, скачавшего файл
    downloaded_at TIMESTAMP NOT NULL-- Дата и время скачивания файла
);

-- Создание таблицы "file_report" для хранения отчетов о файлах
CREATE TABLE IF NOT EXISTS file_report (
    id SERIAL PRIMARY KEY,      -- Уникальный идентификатор отчета
    file_id INT NOT NULL,       -- Идентификатор файла, к которому относится отчет
    user_id INT NOT NULL,       -- Идентификатор пользователя, создавшего отчет
    reason VARCHAR(255) NOT NULL, -- Причина создания отчета
    description TEXT,           -- Описание отчета (необязательное)
    status VARCHAR(255),        -- Статус отчета (необязательное)
    created_at TIMESTAMP NOT NULL-- Дата и время создания отчета
);
--____________________________________________________________________________File_____FILES_______rs__________________________
-- Создание таблицы "files" для хранения информации о файлах
CREATE TABLE IF NOT EXISTS files (
    id SERIAL PRIMARY KEY,               -- Идентификатор файла
    user_id INT NOT NULL,                -- Идентификатор пользователя, загрузившего файл
    community_id INT,                    -- Идентификатор сообщества (может отсутствовать)
    title VARCHAR(255) NOT NULL,         -- Заголовок файла
    description TEXT,                    -- Описание файла (может отсутствовать)
    file_type VARCHAR(255) NOT NULL,     -- Тип файла (например, "image", "document" и т. д.)
    file_url VARCHAR(255),               -- URL файла (может отсутствовать)
    is_private BOOLEAN NOT NULL,         -- Флаг приватности файла (true - приватный, false - публичный)
    is_approved BOOLEAN,                 -- Флаг одобрения файла (может отсутствовать)
    is_deleted BOOLEAN NOT NULL,         -- Флаг удаления файла (true - удален, false - не удален)
    uploaded_at TIMESTAMP NOT NULL,      -- Дата и время загрузки файла
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (community_id) REFERENCES communities (id),
    CHECK (file_type <> ''),
    CHECK (is_private IN (true, false))
);

-- Создание таблицы "file_tags" для хранения связей между файлами и метками
CREATE TABLE IF NOT EXISTS file_tags (
    id SERIAL PRIMARY KEY,          -- Идентификатор связи
    file_id INT NOT NULL,           -- Идентификатор файла
    tag_id INT NOT NULL,            -- Идентификатор метки
    FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
);

-- Создание таблицы "file_comments" для хранения комментариев к файлам
CREATE TABLE IF NOT EXISTS file_comments (
    id SERIAL PRIMARY KEY,          -- Идентификатор комментария
    file_id INT NOT NULL,           -- Идентификатор файла, к которому оставлен комментарий
    user_id INT NOT NULL,           -- Идентификатор пользователя, оставившего комментарий
    comment_text TEXT,              -- Текст комментария (может отсутствовать)
    is_deleted BOOLEAN NOT NULL,    -- Флаг удаления комментария (true - удален, false - не удален)
    created_at TIMESTAMP NOT NULL,  -- Дата и время создания комментария
    FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

-- Создание таблицы "file_likes" для хранения отметок "Нравится" к файлам
CREATE TABLE IF NOT EXISTS file_likes (
    id SERIAL PRIMARY KEY,          -- Идентификатор отметки "Нравится"
    file_id INT NOT NULL,           -- Идентификатор файла, к которому оставлена отметка "Нравится"
    user_id INT NOT NULL,           -- Идентификатор пользователя, поставившего отметку "Нравится"
    created_at TIMESTAMP NOT NULL,  -- Дата и время создания отметки "Нравится"
    FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

-- Создание таблицы "file_downloads" для хранения информации о скачиваниях файлов
CREATE TABLE IF NOT EXISTS file_downloads (
    id SERIAL PRIMARY KEY,          -- Идентификатор скачивания файла
    file_id INT NOT NULL,           -- Идентификатор файла, который был скачан
    user_id INT NOT NULL,           -- Идентификатор пользователя, скачавшего файл
    downloaded_at TIMESTAMP NOT NULL,  -- Дата и время скачивания файла
    FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

-- Создание таблицы "file_report" для хранения отчетов о файлах
CREATE TABLE IF NOT EXISTS file_report (
    id SERIAL PRIMARY KEY,          -- Идентификатор отчета
    file_id INT NOT NULL,           -- Идентификатор файла, на который составлен отчет
    user_id INT NOT NULL,           -- Идентификатор пользователя, создавшего отчет
    reason VARCHAR(255) NOT NULL,   -- Причина отчета
    description TEXT,               -- Описание отчета (может отсутствовать)
    status VARCHAR(255),            -- Статус отчета (может отсутствовать)
    created_at TIMESTAMP NOT NULL,  -- Дата и время создания отчета
    FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    CHECK (reason <> '')
);
--_______________________________________________________________________________file____FORUM______rs____________


-- Создание таблицы "forums" для хранения информации о форумах
CREATE TABLE IF NOT EXISTS forums (
    id SERIAL PRIMARY KEY,               -- Идентификатор форума
    name VARCHAR(255) NOT NULL,          -- Название форума
    description TEXT,                    -- Описание форума (может быть пустым)
    category VARCHAR(255),               -- Категория форума (может быть пустой)
    is_private BOOLEAN NOT NULL,         -- Флаг приватности форума (true - приватный, false - публичный)
    created_at TIMESTAMP NOT NULL,       -- Дата и время создания форума
    updated_at TIMESTAMP NOT NULL,       -- Дата и время последнего обновления форума
    CHECK (name <> ''),
    CHECK (is_private IN (true, false))
);

-- Создание таблицы "forum_topics" для хранения информации о темах форума
CREATE TABLE IF NOT EXISTS forum_topics (
    id SERIAL PRIMARY KEY,          -- Идентификатор темы
    forum_id INT NOT NULL,          -- Идентификатор форума, к которому принадлежит тема
    title VARCHAR(255) NOT NULL,    -- Заголовок темы
    description TEXT,               -- Описание темы (может быть пустым)
    user_id INT NOT NULL,           -- Идентификатор пользователя, создавшего тему
    created_at TIMESTAMP NOT NULL,  -- Дата и время создания темы
    updated_at TIMESTAMP NOT NULL,  -- Дата и время последнего обновления темы
    FOREIGN KEY (forum_id) REFERENCES forums (id) ON DELETE CASCADE,
    CHECK (title <> '')
);

-- Создание таблицы "forum_posts" для хранения сообщений на форуме
CREATE TABLE IF NOT EXISTS forum_posts (
    id SERIAL PRIMARY KEY,          -- Идентификатор сообщения
    topic_id INT NOT NULL,          -- Идентификатор темы, к которой принадлежит сообщение
    user_id INT NOT NULL,           -- Идентификатор пользователя, написавшего сообщение
    content TEXT NOT NULL,          -- Содержание сообщения
    created_at TIMESTAMP NOT NULL,  -- Дата и время создания сообщения
    updated_at TIMESTAMP NOT NULL,  -- Дата и время последнего обновления сообщения
    FOREIGN KEY (topic_id) REFERENCES forum_topics (id) ON DELETE CASCADE
);

-- Создание таблицы "forum_likes" для хранения лайков к сообщениям на форуме
CREATE TABLE IF NOT EXISTS forum_likes (
    id SERIAL PRIMARY KEY,          -- Идентификатор лайка
    post_id INT NOT NULL,           -- Идентификатор сообщения, к которому поставлен лайк
    user_id INT NOT NULL,           -- Идентификатор пользователя, поставившего лайк
    created_at TIMESTAMP NOT NULL,  -- Дата и время поставки лайка
    FOREIGN KEY (post_id) REFERENCES forum_posts (id) ON DELETE CASCADE
);

-- Создание таблицы "forum_subscriptions" для описания подписок на темы форума
CREATE TABLE IF NOT EXISTS forum_subscriptions (
    id SERIAL PRIMARY KEY,          -- Идентификатор подписки
    user_id INT NOT NULL,           -- Идентификатор пользователя, подписавшегося
    topic_id INT NOT NULL,          -- Идентификатор темы, на которую подписан
    created_at TIMESTAMP NOT NULL,  -- Дата и время подписки
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (topic_id) REFERENCES forum_topics (id) ON DELETE CASCADE
);

-- Создание таблицы "forum_reports" для хранения отчетов о сообщениях на форуме
CREATE TABLE IF NOT EXISTS forum_reports (
    id SERIAL PRIMARY KEY,          -- Идентификатор отчета
    post_id INT NOT NULL,           -- Идентификатор сообщения, на которое составлен отчет
    user_id INT NOT NULL,           -- Идентификатор пользователя, создавшего отчет
    reason VARCHAR(255) NOT NULL,   -- Причина отчета
    description TEXT,               -- Описание отчета (может быть пустым)
    status VARCHAR(255),            -- Статус отчета (может быть пустым)
    created_at TIMESTAMP NOT NULL,  -- Дата и время создания отчета
    FOREIGN KEY (post_id) REFERENCES forum_posts (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    CHECK (reason <> '')
);

--______________________________________file__REQUESTS_____rs

-- Создание таблицы "requests" для хранения информации о запросах
CREATE TABLE IF NOT EXISTS requests (
    id SERIAL PRIMARY KEY,                -- Идентификатор запроса
    user_id INT NOT NULL,                 -- Идентификатор пользователя, создавшего запрос
    service_id INT NOT NULL,              -- Идентификатор услуги, связанной с запросом
    request_date TIMESTAMP NOT NULL,      -- Дата и время создания запроса
    description TEXT,                     -- Описание запроса (может быть пустым)
    status VARCHAR(255) NOT NULL,         -- Статус запроса
    assigned_to INT,                      -- Идентификатор пользователя, назначенного на запрос (может быть пустым)
    priority SMALLINT NOT NULL,           -- Приоритет запроса
    due_date DATE,                        -- Срок выполнения запроса (может быть пустым)
    location POINT,                       -- Местоположение запроса (может быть пустым)
    contact_name VARCHAR(255),            -- Имя контактного лица (может быть пустым)
    contact_phone VARCHAR(255),           -- Телефон контактного лица (может быть пустым)
    contact_email VARCHAR(255),           -- Email контактного лица (может быть пустым)
    comments TEXT,                        -- Комментарии к запросу (может быть пустым)
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (service_id) REFERENCES services (id) ON DELETE CASCADE,
    FOREIGN KEY (assigned_to) REFERENCES users (id) ON DELETE SET NULL,
    CHECK (status <> ''),
    CHECK (priority >= 0)
);
--_______________________________________________file__SHEDULES___RS___



-- Создание таблицы "transport_vehicles" для хранения информации о транспортных средствах
CREATE TABLE IF NOT EXISTS transport_vehicles (
    id SERIAL PRIMARY KEY,              -- Идентификатор транспортного средства
    vehicle_name VARCHAR(255) NOT NULL, -- Название транспортного средства
    vehicle_description TEXT NOT NULL,  -- Описание транспортного средства
    carrier_id INT NOT NULL,            -- Идентификатор перевозчика (поставщика)
    model VARCHAR(255) NOT NULL,        -- Модель транспортного средства
    route_number VARCHAR(255),          -- Номер маршрута (может быть пустым)
    FOREIGN KEY (carrier_id) REFERENCES carrier_information (id) ON DELETE CASCADE
);

-- Создание таблицы "schedules" для хранения информации о расписаниях
CREATE TABLE IF NOT EXISTS schedules (
    id SERIAL PRIMARY KEY,              -- Идентификатор расписания
    vehicle_id INT NOT NULL,            -- Идентификатор транспортного средства, к которому относится расписание
    departure_date DATE NOT NULL,       -- Дата отправления
    arrival_date DATE NOT NULL,         -- Дата прибытия
    departure_location VARCHAR(255) NOT NULL, -- Место отправления
    arrival_location VARCHAR(255) NOT NULL,   -- Место прибытия
    intermediate_stops TEXT,            -- Промежуточные остановки (могут быть пустыми)
    departure_time TIME NOT NULL,       -- Время отправления
    arrival_time TIME NOT NULL,         -- Время прибытия
    FOREIGN KEY (vehicle_id) REFERENCES transport_vehicles (id) ON DELETE CASCADE
);

-- Создание таблицы "price_and_tickets" для хранения информации о ценах и билетах
CREATE TABLE IF NOT EXISTS price_and_tickets (
    id SERIAL PRIMARY KEY,              -- Идентификатор информации о ценах и билетах
    schedule_id INT NOT NULL,           -- Идентификатор расписания, связанного с ценами и билетами
    ticket_price NUMERIC(10, 2) NOT NULL,  -- Цена билета
    ticket_availability BOOLEAN NOT NULL,  -- Доступность билета (true/false)
    FOREIGN KEY (schedule_id) REFERENCES schedules (id) ON DELETE CASCADE
);

-- Создание таблицы "carrier_information" для хранения информации о перевозчиках
CREATE TABLE IF NOT EXISTS carrier_information (
    id SERIAL PRIMARY KEY,              -- Идентификатор перевозчика
    vehicle_id INT NOT NULL,            -- Идентификатор транспортного средства, связанного с перевозчиком
    carrier_name VARCHAR(255) NOT NULL, -- Название перевозчика
    carrier_contact_information TEXT NOT NULL, -- Контактная информация перевозчика
    FOREIGN KEY (vehicle_id) REFERENCES transport_vehicles (id) ON DELETE CASCADE
);
--_________________________________________file_________SUBSCRIPTIONS____RS_______

-- Создание таблицы "subscriptions" для хранения подписок
CREATE TABLE IF NOT EXISTS subscriptions (
    id SERIAL PRIMARY KEY,                  -- Уникальный идентификатор подписки
    user_id INT NOT NULL,                   -- Идентификатор пользователя
    object_id INT NOT NULL,                 -- Идентификатор объекта
    object_type VARCHAR(255) NOT NULL,      -- Тип объекта
    status VARCHAR(255) NOT NULL,           -- Статус подписки
    created_at TIMESTAMP NOT NULL,          -- Дата и время создания подписки
    name VARCHAR(255) NOT NULL,             -- Название
    types SMALLINT NOT NULL,                -- Типы
    link VARCHAR(255) NOT NULL,             -- Ссылка
    image VARCHAR(255) NOT NULL,            -- Изображение
    category_id INT NOT NULL,               -- Идентификатор категории
    lists SMALLINT NOT NULL,                -- Списки
    members INT NOT NULL,                   -- Количество участников
    description TEXT,                       -- Описание (необязательное)
);
--________________________________________________________________________________
































