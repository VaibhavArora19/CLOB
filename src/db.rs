use std::{path::Path, sync::Arc};

use anyhow::Error;
use libmdbx::{self, Database, DatabaseOptions, Mode, Table, TableFlags, WriteFlags, WriteMap};

use crate::{
    order::{Order, Side},
    order_book::OrderBook,
    price_level::PriceLevel,
};

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
        let list = vec![TableInfo::new("orders"), TableInfo::new("price_levels")];

        Box::new(list.into_iter())
    }
}

pub fn connect_db<T: TableSet>() -> Result<Arc<Database<WriteMap>>, Error> {
    let path = Path::new("./storage");

    let db = Arc::new(
        libmdbx::Database::<WriteMap>::open_with_options(
            &path,
            DatabaseOptions {
                mode: Mode::default(),
                max_tables: Some(3),
                ..Default::default()
            },
        )
        .map_err(|e| {
            log::error!(
                "Failed to open database connections. Failed with error: {:?}",
                e
            );

            Error::msg("Failed to open database connection")
        })?,
    );

    log::info!("DB Connected!");

    let rw_txn = db.begin_rw_txn().unwrap();

    for table in T::tables() {
        rw_txn
            .create_table(Some(table.name), TableFlags::DUP_SORT)
            .map_err(|e| {
                log::error!(
                    "Failed to create table {:?}. Failed with error: {:?}",
                    table.name,
                    e
                );

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
        log::error!(
            "Failed to put data into orders table. Failed with error: {:?}",
            err
        );
    };

    if let Err(err) = rw_txn.put(&price_table, price_id, id, WriteFlags::default()) {
        log::error!(
            "Failed to put data into price levels table. Failed with error: {:?}",
            err
        );
    };

    rw_txn.commit().unwrap();
}

pub async fn build_order_book(db: Arc<Database<WriteMap>>, order_book: &mut OrderBook) {
    let ro_txn = db.begin_ro_txn().unwrap();
    let price_levels_table = ro_txn.open_table(Some("price_levels")).unwrap();

    let price_levels_cursor = ro_txn.cursor(&price_levels_table).unwrap();

    for price_level_item in price_levels_cursor {
        let (_, value) = price_level_item.unwrap();

        let value_bytes: &[u8] = value.as_ref().try_into().unwrap();

        if let Ok(order) = get_order(db.clone(), value_bytes) {
            match order.side {
                Side::Bid => {
                    let level = order_book
                        .bids
                        .levels
                        .entry(order.price)
                        .or_insert_with(|| PriceLevel::new(order.price));
                    level.add_orders(order);
                }
                Side::Ask => {
                    let level = order_book
                        .asks
                        .levels
                        .entry(order.price)
                        .or_insert_with(|| PriceLevel::new(order.price));
                    level.add_orders(order);
                }
            }
        };
    }
}

pub fn get_order(db: Arc<Database<WriteMap>>, key: &[u8]) -> Result<Order, anyhow::Error> {
    let ro_txn = db.begin_ro_txn().unwrap();
    let orders_table = ro_txn.open_table(Some("orders")).unwrap();

    let is_order: Option<Vec<u8>> = ro_txn.get(&orders_table, &key).unwrap();

    if let Some(order) = is_order {
        Ok(serde_json::from_str(String::from_utf8(order).unwrap().as_str()).unwrap())
    } else {
        Err(anyhow::Error::msg("Failed to parse order"))
    }
}
