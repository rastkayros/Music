//подключить библиотеку Diesel и определить структуры NewTransportVehicle, NewSchedule, NewPriceAndTicket, NewCarrierInformation, UpdateTransportVehicle, UpdateSchedule, 
//UpdatePriceAndTicket и UpdateCarrierInformation,

use chrono::{NaiveDate, NaiveTime};
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

// Определение структуры для транспортного средства---------------------------------------------------------------------------
#[derive(Debug, Queryable, Identifiable, Associations)]
pub struct TransportVehicle {
    pub id: i32,
    pub vehicle_name: String,
    pub vehicle_description: String,
    pub carrier_id: i32,
    pub model: String,
    pub route_number: Option<String>,
}

// Определение структуры для расписания---------------------------------------------------------------------------------------
#[derive(Debug, Queryable, Identifiable, Associations)]
pub struct Schedule {
    pub id: i32,
    pub vehicle_id: i32,
    pub departure_date: NaiveDate,
    pub arrival_date: NaiveDate,
    pub departure_location: String,
    pub arrival_location: String,
    pub intermediate_stops: Option<String>,
    pub departure_time: NaiveTime,
    pub arrival_time: NaiveTime,
}

// Определение структуры для цен и билетов-------------------------------------------------------------------------------------
#[derive(Debug, Queryable, Identifiable, Associations)]
pub struct PriceAndTicket {
    pub id: i32,
    pub schedule_id: i32,
    pub ticket_price: f64,
    pub ticket_availability: bool,
}

// Определение структуры для информации о перевозчике--------------------------------------------------------------------------
#[derive(Debug, Queryable, Identifiable, Associations)]
pub struct CarrierInformation {
    pub id: i32,
    pub vehicle_id: i32,
    pub carrier_name: String,
    pub carrier_contact_information: String,
}

// Реализация методов для структуры TransportVehicle------------------------------------------------------------------------------

// Определение структуры для создания нового транспортного средства
#[derive(Debug, Insertable)]
#[table_name = "transport_vehicles"]
pub struct NewTransportVehicle {
    pub vehicle_name: String,
    pub vehicle_description: String,
    pub carrier_id: i32,
    pub model: String,
    pub route_number: Option<String>,
}

impl NewTransportVehicle {
    // Конструктор для создания нового транспортного средства
    pub fn new(
        vehicle_name: String,
        vehicle_description: String,
        carrier_id: i32,
        model: String,
        route_number: Option<String>,
    ) -> Self {
        NewTransportVehicle {
            vehicle_name,
            vehicle_description,
            carrier_id,
            model,
            route_number,
        }
    }
    // Создание нового транспортного средства
    pub fn create(new_vehicle: &NewTransportVehicle, connection: &PgConnection) -> Result<Self, diesel::result::Error> {
        use crate::schema::transport_vehicles;

        diesel::insert_into(transport_vehicles::table)
            .values(new_vehicle)
            .get_result(connection)
    }
}

impl TransportVehicle {
    // Поиск транспортного средства по ID
    pub fn find_by_id(id: i32, connection: &PgConnection) -> Result<Option<Self>, diesel::result::Error> {
        use crate::schema::transport_vehicles::dsl::*;

        transport_vehicles.filter(id.eq(id)).first(connection).optional()
    }

    // Поиск всех транспортных средств
    pub fn find_all(connection: &PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::transport_vehicles::dsl::*;

        transport_vehicles.load(connection)
    }

    // Поиск транспортных средств по названию
    pub fn find_by_name(name: &str, connection: &PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::transport_vehicles::dsl::*;

        transport_vehicles.filter(vehicle_name.eq(name)).load(connection)
    }

        // Обновление информации о транспортном средстве
    pub fn update(id: i32, updated_vehicle: &UpdateTransportVehicle, connection: &PgConnection) -> Result<Self, diesel::result::Error> {
        use crate::schema::transport_vehicles::dsl::*;

        diesel::update(transport_vehicles.filter(id.eq(id)))
            .set(updated_vehicle)
            .get_result(connection)
    }

    // Удаление транспортного средства по ID
    pub fn delete(id: i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use crate::schema::transport_vehicles::dsl::*;

        diesel::delete(transport_vehicles.filter(id.eq(id))).execute(connection)?;

        Ok(())
    }
}
use chrono::{NaiveDate, NaiveTime};

// Определение структуры для создания нового расписания
#[derive(Debug, Insertable)]
#[table_name = "schedules"]
pub struct NewSchedule {
    pub vehicle_id: i32,
    pub departure_date: NaiveDate,
    pub arrival_date: NaiveDate,
    pub departure_location: String,
    pub arrival_location: String,
    pub intermediate_stops: Option<String>,
    pub departure_time: NaiveTime,
    pub arrival_time: NaiveTime,
}

impl NewSchedule {
    // Конструктор для создания нового расписания
    pub fn new(
        vehicle_id: i32,
        departure_date: NaiveDate,
        arrival_date: NaiveDate,
        departure_location: String,
        arrival_location: String,
        intermediate_stops: Option<String>,
        departure_time: NaiveTime,
        arrival_time: NaiveTime,
    ) -> Self {
        NewSchedule {
            vehicle_id,
            departure_date,
            arrival_date,
            departure_location,
            arrival_location,
            intermediate_stops,
            departure_time,
            arrival_time,
        }
    }
    // Создание нового расписания
    pub fn create(new_schedule: &NewSchedule, connection: &PgConnection) -> Result<Self, diesel::result::Error> {
        use crate::schema::schedules;

        diesel::insert_into(schedules::table)
            .values(new_schedule)
            .get_result(connection)
    }
    
}

// Реализация методов для структуры Schedule----------------------------------------------------------------------------------------------------

impl Schedule {
    // Поиск расписания по ID
    pub fn find_by_id(id: i32, connection: &PgConnection) -> Result<Option<Self>, diesel::result::Error> {
        use crate::schema::schedules::dsl::*;

        schedules.filter(id.eq(id)).first(connection).optional()
    }

    // Поиск всех расписаний для определенного транспортного средства
    pub fn find_by_vehicle_id(vehicle_id: i32, connection: &PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::schedules::dsl::*;

        schedules.filter(vehicle_id.eq(vehicle_id)).load(connection)
    }

    // Поиск расписаний по дате отправления
    pub fn find_by_departure_date(date: NaiveDate, connection: &PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::schedules::dsl::*;

        schedules.filter(departure_date.eq(date)).load(connection)
    }

    // Поиск расписаний по местам отправления и прибытия
    pub fn find_by_departure_and_arrival(
        departure_location: &str,
        arrival_location: &str,
        connection: &PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::schedules::dsl::*;

        schedules
            .filter(departure_location.eq(departure_location).and(arrival_location.eq(arrival_location)))
            .load(connection)
    }

    

    // Обновление информации о расписании
    pub fn update(id: i32, updated_schedule: &UpdateSchedule, connection: &PgConnection) -> Result<Self, diesel::result::Error> {
        use crate::schema::schedules::dsl::*;

        diesel::update(schedules.filter(id.eq(id)))
            .set(updated_schedule)
            .get_result(connection)
    }

    // Удаление расписания по ID
    pub fn delete(id: i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use crate::schema::schedules::dsl::*;

        diesel::delete(schedules.filter(id.eq(id))).execute(connection)?;

        Ok(())
    }
}

// Реализация методов для структуры PriceAndTicket
// Определение структуры для создания новой информации о ценах и билетах
#[derive(Debug, Insertable)]
#[table_name = "price_and_tickets"]
pub struct NewPriceAndTicket {
    pub schedule_id: i32,
    pub ticket_price: f64,
    pub ticket_availability: bool,
}

impl NewPriceAndTicket {
    // Конструктор для создания новой информации о ценах и билетах
    pub fn new(schedule_id: i32, ticket_price: f64, ticket_availability: bool) -> Self {
        NewPriceAndTicket {
            schedule_id,
            ticket_price,
            ticket_availability,
        }
    }
    // Создание новой информации о цене и билетах
    pub fn create(new_price_and_ticket: &NewPriceAndTicket, connection: &PgConnection) -> Result<Self, diesel::result::Error> {
        use crate::schema::price_and_tickets;

        diesel::insert_into(price_and_tickets::table)
            .values(new_price_and_ticket)
            .get_result(connection)
    }
}

impl PriceAndTicket {
    // Поиск информации о цене и билетах по ID расписания
    pub fn find_by_schedule_id(schedule_id: i32, connection: &PgConnection) -> Result<Option<Self>, diesel::result::Error> {
        use crate::schema::price_and_tickets::dsl::*;

        price_and_tickets.filter(schedule_id.eq(schedule_id)).first(connection).optional()
    }

    // Поиск всех доступных билетов
    pub fn find_available_tickets(connection: &PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::price_and_tickets::dsl::*;

        price_and_tickets.filter(ticket_availability.eq(true)).load(connection)
    }

    // Поиск билетов по цене
    pub fn find_by_ticket_price(price: f64, connection: &PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::price_and_tickets::dsl::*;

        price_and_tickets.filter(ticket_price.eq(price)).load(connection)
    }

    

    // Обновление информации о цене и билетах
    pub fn update(id: i32, updated_price_and_ticket: &UpdatePriceAndTicket, connection: &PgConnection) -> Result<Self, diesel::result::Error> {
        use crate::schema::price_and_tickets::dsl::*;

        diesel::update(price_and_tickets.filter(id.eq(id)))
            .set(updated_price_and_ticket)
            .get_result(connection)
    }

    // Удаление информации о цене и билетах по ID
    pub fn delete(id: i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use crate::schema::price_and_tickets::dsl::*;

        diesel::delete(price_and_tickets.filter(id.eq(id))).execute(connection)?;

        Ok(())
    }
}

// Реализация методов для структуры CarrierInformation
// Определение структуры для создания новой информации о перевозчике
#[derive(Debug, Insertable)]
#[table_name = "carrier_information"]
pub struct NewCarrierInformation {
    pub vehicle_id: i32,
    pub carrier_name: String,
    pub carrier_contact_information: String,
}

impl NewCarrierInformation {
    // Конструктор для создания новой информации о перевозчике
    pub fn new(vehicle_id: i32, carrier_name: String, carrier_contact_information: String) -> Self {
        NewCarrierInformation {
            vehicle_id,
            carrier_name,
            carrier_contact_information,
        }
    }
    // Создание новой информации о перевозчике
    pub fn create(new_carrier_info: &NewCarrierInformation, connection: &PgConnection) -> Result<Self, diesel::result::Error> {
        use crate::schema::carrier_information;

        diesel::insert_into(carrier_information::table)
            .values(new_carrier_info)
            .get_result(connection)
    }
}

impl CarrierInformation {
    // Поиск информации о перевозчике по ID транспортного средства
    pub fn find_by_vehicle_id(vehicle_id: i32, connection: &PgConnection) -> Result<Option<Self>, diesel::result::Error> {
        use crate::schema::carrier_information::dsl::*;

        carrier_information.filter(vehicle_id.eq(vehicle_id)).first(connection).optional()
    }

    // Поиск информации о перевозчике по имени
    pub fn find_by_carrier_name(name: &str, connection: &PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::carrier_information::dsl::*;

        carrier_information.filter(carrier_name.eq(name)).load(connection)
    }

    

    // Обновление информации о перевозчике
    pub fn update(id: i32, updated_carrier_info: &UpdateCarrierInformation, connection: &PgConnection) -> Result<Self, diesel::result::Error> {
        use crate::schema::carrier_information::dsl::*;

        diesel::update(carrier_information.filter(id.eq(id)))
            .set(updated_carrier_info)
            .get_result(connection)
    }

    // Удаление информации о перевозчике по ID
    pub fn delete(id: i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use crate::schema::carrier_information::dsl::*;

        diesel::delete(carrier_information.filter(id.eq(id))).execute(connection)?;

        Ok(())
    }
}
