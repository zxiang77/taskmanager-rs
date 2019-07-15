use chrono::{Date, Utc, NaiveDate, DateTime};
use crate::task::TaskState::OPEN;
use crate::db_connection;
use std::collections::hash_map;
use tokio_postgres::types::ToSql;
use std::fmt::Error;
use std::string::String;
use postgres::{Client, NoTls};

pub struct Task {
    id: Option<u64>,
    name: Option<String>,
    desc: Option<String>,
    state: TaskState,
    due_date: Option<DateTime<Utc>>,
}

const ST: &'static str = "";

// Ref: https://www.enodev.fr/posts/rusticity-convert-an-integer-to-an-enum.html
const TABLE_NAME: &'static str = "TASK";

pub enum TaskState {
    OPEN = 0, WIP = 1, FINISHED = 2, SUSPENDED = 3
}

impl TaskState {
    pub fn from_u64(i: u64) -> Option<TaskState> {
        match i {
            0 => Some(TaskState::OPEN),
            1 => Some(TaskState::WIP),
            2 => Some(TaskState::FINISHED),
            3 => Some(TaskState::SUSPENDED),
            _ => None,
        }
    }
}

impl Task {
    pub fn new(id: Option<u64>,
           name: Option<String>,
           desc: Option<String>,
           state: TaskState,
           due_date: Option<DateTime<Utc>>
    ) -> Self {
        Task {
            id,
            name,
            desc,
            due_date,
            state,
        }
    }

    // todo, complete this
    fn build_condition(&self) -> &str {
        ""
    }

    pub fn query(&self, opt: &[&dyn ToSql]) -> Vec<Self> { // could be a collection of self
        let mut q = "SELECT ID, NAME, DESC, STATE, DUE_DATE FROM $1".to_owned();//String::from_str(, &[&TABLE_NAME]);

        q.push_str(self.build_condition());
        q.push(';');

        let mut client = get_client();
        let mut ret = Vec::new();

        for row in &client.query(&q[..], opt).unwrap() {
            let id: Option<String> = row.get(0);
            let id  = Some(id.unwrap().parse::<u64>().unwrap());
            let name = row.get(1);
            let desc = row.get(2);
            let state_str: Option<String> = row.get(3);
            let state:TaskState = TaskState::from_u64(state_str.unwrap().parse::<u64>().unwrap()).unwrap();
            let dt_opt:Option<String> = row.get(4);
            let datetime = DateTime::parse_from_rfc3339(&dt_opt.unwrap()).unwrap();

            // let no_timezone = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?;

            ret.push(
                Task::new(id, name, desc, state, Some(datetime.with_timezone(&Utc)))
            );
        }
        ret
    }

    fn is_overdue(&self) -> bool {
        let now = Utc::now().timestamp();
        let due = match self.due_date.as_ref() {
            Some(&datetime) => datetime.timestamp(),
            None => return false // treat no due date as not overdue
        };
        due > now
    }

    fn create(&self, opt: &[&impl ToSql]) -> Result<Self, DataError> { //
        //todo: passing in the connection string from env var
        let q = "INSERT INTO $1 (NAME, DESC, DUE_DATE, STATE) VALUES ($2, $3, $4, $5);".to_owned();
        // persist it to local cache first and get a copy
        // abstract db
        let mut client = get_client();
        let name = match self.name.as_ref() {
            Some(nm) => nm,
            None => "",
        };

        let desc = match self.name.as_ref() {
            Some(dsc) => dsc,
            None => ""
        };

        client.execute(&q[..], &[&TABLE_NAME, ]); // todo complete this
        Ok((&self).query(&[]).remove(0))
    }
}

#[derive(Debug)]
pub struct DataError;
pub struct NotFoundError;
use std::error;
use std::fmt;
use chrono::format::Numeric::Day;
use crate::db_connection::get_client;

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), Error> {
        write!(f, "Some field is missing.")
    }
}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), Error> {
        write!(f, "Some field is missing.")
    }
}



impl error::Error for DataError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

// if it's just CRUD, then do we care about the internal mutability?
// either extract raw date to create or just