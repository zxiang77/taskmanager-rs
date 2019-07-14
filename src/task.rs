use chrono::{Date, Utc, NaiveDate, DateTime};
use crate::task::TaskState::OPEN;
use postgres::{Client};
use crate::db_connection;
use std::collections::hash_map;
pub struct Task {
    id: Option<u64>,
    name: Option<String>,
    desc: Option<String>,
    state: TaskState,
    due_date: Option<DateTime<Utc>>,
    modified: bool,
}

static ST: String = String::from_str("");

static TABLE_NAME: String = String::from_str("TASK");

#[derive(FromPrimitive)]
pub enum TaskState {
    OPEN = 0, WIP = 1, FINISHED = 2, SUSPENDED = 3
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
            modified: false,
        }
    }

    // todo
    fn build_condition(&self) -> &str {
        ""
    }

    pub fn query(&self, opt: &[&dyn ToSql]) -> Vec<Self> { // could be a collection of self
        let mut q = String::from_str("SELECT ID, NAME, DESC, STATE, DUE_DATE FROM $1", &[&TABLE_NAME]);

        q.push_str(self.build_condition());
        q.push(';');

        let mut client = Client::new();
        let res = client.query(q, opt)?; //  assuming getting all the rows
        let mut ret = Vec::new();

        for row in res {
            let id: Option<u64> = row.get(0);
            let name = row.get(1);
            let desc = row.get(2);
            let state:TaskState = num::FromPrimitive::from_u32(row.get(3));
            let dt_opt:Option<String> = row.get(4);
            let datetime = Utc::parse_from_rfc3339(dt_opt);

            ret.push(Task::new(
                id, name, desc, state, Some(datetime)
            ));
        }
        ret
    }

    fn is_overdue(&self) -> bool {
        let now = Utc::now().timestamp();
        let due = match self.due_date.as_ref() {
            Some(&datetime) => datetime.timestamp(),
            None => return false // regard no due date as not overdue
        };
        due > now
    }

    fn create(self) -> Self { //
        //todo: passing in the connection string from env var

        // persist it to local cache first and get a copy
        // abstract db

    }
}



// if it's just CRUD, then do we care about the internal mutability?
// either extract raw date to create or just