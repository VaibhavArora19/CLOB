use std::{path::Path, sync::Arc};

use anyhow::Error;
use libmdbx::{self, Database, DatabaseOptions, Mode, Table, TableFlags, WriteFlags, WriteMap};


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

pub fn connect_db<T: TableSet>() -> Result<Arc<Database<WriteMap>>, Error> {
    let path = Path::new("./storage");

    let db = Arc::new(libmdbx::Database::<WriteMap>::open_with_options(&path, DatabaseOptions {
        mode: Mode::default(),
        max_tables: Some(3),
        ..Default::default()
    }).map_err(|e| {
        log::error!("Failed to open database connections. Failed with error: {:?}", e);

        Error::msg("Failed to open database connection")
    })?);

    log::info!("DB Connected!");

    let rw_txn = db.begin_rw_txn().unwrap();


    for table in T::tables() {
        rw_txn.create_table(Some(table.name), TableFlags::DUP_SORT).map_err(|e|  {
            log::error!("Failed to create table {:?}. Failed with error: {:?}", table.name, e);

            Error::msg("Failed to create table")
        })?;
    }
    rw_txn.commit().unwrap();

    Ok(db)
}

pub async fn insert(db: Arc<Database<WriteMap>>, id: u64, price: u64, order: String) {
    let rw_txn = db.begin_rw_txn().unwrap();

    let order_table: Table = rw_txn.open_table(Some("orders")).unwrap();
    let price_table = rw_txn.open_table(Some("price_levels")).unwrap();

    let id = &id.to_le_bytes();
    let price_id = &price.to_le_bytes();

    if let Err(err) = rw_txn.put(&order_table, id, &order.as_str(), WriteFlags::default()) {
        log::error!("Failed to put data into orders table. Failed with error: {:?}", err);
    };

    
    if let Err(err) = rw_txn.put(&price_table, price_id, id, WriteFlags::default()) {
        log::error!("Failed to put data into price levels table. Failed with error: {:?}", err);
    };

    rw_txn.commit().unwrap();

    get_one(db.clone()).await;
    
}

pub async fn get_all(db: Arc<Database<WriteMap>>) {
    let ro_txn = db.begin_ro_txn().unwrap();
    let price_levels_table = ro_txn.open_table(Some("price_levels")).unwrap();
    let orders_table = ro_txn.open_table(Some("orders")).unwrap();

    let cursor = ro_txn.cursor(&orders_table).unwrap();

    for item in cursor {
        let (key, value) = item.unwrap();
        log::info!("key value: {:?}{:?}", key, value);
    }

}

pub async fn get_one(db: Arc<Database<WriteMap>>) {
    let ro_txn = db.begin_ro_txn().unwrap();
    let orders_table = ro_txn.open_table(Some("orders")).unwrap();

    let id: u64 = 10;
    let key = id.to_le_bytes();

    let order: Option<Vec<u8>> = ro_txn.get(&orders_table, &key).unwrap();

    log::info!("Order is: {:?}", order);
}