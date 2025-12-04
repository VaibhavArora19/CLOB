use std::path::Path;

use anyhow::Error;
use libmdbx::{self, Database, DatabaseOptions, Mode, TableFlags, WriteMap};

pub struct TableInfo {
    pub name: &'static str,
}

impl TableInfo {
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }
}

pub trait TableSet {
    fn tables() -> Box<dyn Iterator<Item = TableInfo>>;
}


pub struct Tables; 

impl TableSet for Tables {
    fn tables() -> Box<dyn Iterator<Item = TableInfo>> {
        let list = vec![
            TableInfo::new("orders"),
            TableInfo::new("price_levels"),
        ];

        Box::new(list.into_iter())
    }
}

pub fn connect_db<T: TableSet>() -> Result<Database<WriteMap>, Error> {
    let path = Path::new("../../storage");
    let db = libmdbx::Database::<WriteMap>::open_with_options(&path, DatabaseOptions {
        mode: Mode::default(),
        max_tables: Some(2),
        ..Default::default()
    }).map_err(|e| {
        log::error!("Failed to open database connections. Failed with error: {:?}", e);

        Error::msg("Failed to open database connection")
    })?;

    log::info!("DB Connected!");

    let rw_txn = db.begin_rw_txn().unwrap();


    for table in T::tables() {
        rw_txn.create_table(Some(table.name), TableFlags::default()).map_err(|e|  {
            log::error!("Failed to create table {:?}. Failed with error: {:?}", table.name, e);

            Error::msg("Failed to create table")
        })?;
    }
    rw_txn.commit().unwrap();

    Ok(db)
}