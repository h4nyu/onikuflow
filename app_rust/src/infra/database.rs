use crate::domain::entities::{Point, Trace};
use crate::domain::usecase::{HavePointQuery, HaveTraceQuery};
use crate::domain::{PointRepository, Repository, TraceRepository, Transition};
use chrono::prelude::{DateTime, Utc};
use csv::Writer;
use postgres::{types::ToSql, Client, NoTls, Row};
use rayon::prelude::*;
use serde::Serialize;
use std::error::Error;
use std::io::Write;
use uuid::Uuid;
use std::sync::{Mutex, Arc};
pub struct Postgresql(Arc<Mutex<Client>>);
pub struct QueryValue<'a>(&'static str, &'a dyn ToSql);

pub trait SQLable {
    fn table_name() -> &'static str;
    fn select() -> &'static str;
    fn copy_in() -> &'static str;
    fn from_row(row: &Row) -> Self;
}

impl Postgresql {
    pub fn new() -> Postgresql {
        Postgresql(
            Arc::new(
                Mutex::new(
                    Client::connect("host=db user=mlboard password=mlboard", NoTls).unwrap()
                )
            )
        )
    }
}

impl SQLable for Point {
    fn table_name() -> &'static str {
        "points"
    }
    fn select() -> &'static str {
        "SELECT * FROM points"
    }
    fn copy_in() -> &'static str {
        "COPY points FROM STDIN CSV HEADER"
    }
    fn from_row(row: &Row) -> Self {
        Self {
            value: row.get("value"),
            trace_id: row.get("trace_id"),
            ts: row.get("ts"),
        }
    }
}

impl SQLable for Trace {
    fn select() -> &'static str {
        "SELECT * FROM traces"
    }
    fn table_name() -> &'static str {
        "traces"
    }
    fn copy_in() -> &'static str {
        "COPY traces FROM STDIN CSV HEADER"
    }
    fn from_row(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            updated_at: row.get("updated_at"),
            created_at: row.get("created_at"),
        }
    }
}

impl<T> Repository<T> for Postgresql
where
    T: SQLable + Serialize,
{
    fn all(&self) -> Vec<T> {
        Arc::clone(&self.0).lock().unwrap()
            .query::<str>(T::select(), &[])
            .unwrap()
            .iter()
            .map(T::from_row)
            .collect::<Vec<T>>()
    }
    fn bulk_insert(&self, rows: &[T]) -> u64 {
        let mut csv_wtr = Writer::from_writer(vec![]);
        rows.iter().for_each(|x| {
            csv_wtr.serialize(x).unwrap();
        });
        let data = String::from_utf8(csv_wtr.into_inner().unwrap()).unwrap();

        let conn = Arc::clone(&self.0);
        let mut conn = conn.lock().unwrap();
        let mut copy_in_writer = conn.copy_in::<str>(T::copy_in()).unwrap();
        copy_in_writer.write_all(data.as_bytes()).unwrap();

        return copy_in_writer.finish().unwrap();
    }

    fn get(&self, _id: Uuid) -> Option<T> {
        return None;
    }
    fn clear(&self) -> () {
        let sql = format!("TRUNCATE TABLE {}", T::table_name());
        Arc::clone(&self.0).lock().unwrap()
            .execute::<str>(&sql, &[]).unwrap();
    }
}

impl PointRepository for Postgresql {}
impl TraceRepository for Postgresql {}

impl HavePointQuery for Postgresql {
    fn point_query(&mut self) -> &mut dyn PointRepository {
        return self;
    }
}

impl HaveTraceQuery for Postgresql {
    fn trace_query(&mut self) -> &mut dyn TraceRepository {
        return self;
    }
}

impl Transition for Postgresql {
    fn with_tx<F>(&self, f: F) -> ()
    where
        F: Fn() -> Result<(), ()>
    {
        let conn = Arc::clone(&self.0);
        conn.lock().unwrap().execute("BEGIN;", &[]).unwrap();
        match f() {
            Ok(()) => {conn.lock().unwrap().execute("COMMIT;", &[]).unwrap();},
            Err(e) => {conn.lock().unwrap().execute("ROLLBACK;", &[]).unwrap();}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transtion() {
        let repo = Postgresql::new();
        let trace_id = Uuid::new_v4();
        let points = (0..10).collect::<Vec<_>>().into_iter().map(|x| {
            Point{
                value: x as f64,
                ts: Utc::now(),
                trace_id: trace_id,
            }
        }).collect::<Vec<_>>();
        let point_repo: &PointRepository = &repo;
        point_repo.clear();
        repo.with_tx(|| {
            point_repo.bulk_insert(&points);
            Err(())
        });
        assert_eq!(point_repo.all().len(), 0);
        repo.with_tx(|| {
            point_repo.bulk_insert(&points);
            Ok(())
        });
        assert_eq!(point_repo.all().len(), 10);
    }
}
