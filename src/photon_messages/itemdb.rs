use curl::easy::Easy;
use std::mem;
use std::str;
use std::sync::Mutex;

include!(concat!(env!("OUT_DIR"), "/itemdb.rs"));

type Database = HashMap<u32, String>;

enum ItemDB {
    Uninitialized,
    Initialized(Database),
}

lazy_static! {
    static ref ITEM_DB: Mutex<ItemDB> = Mutex::new(ItemDB::Uninitialized);
}

fn get_compiled_database() -> Database {
    ITEMDB
        .iter()
        .map(|(k, v)| (*k, String::from(*v)))
        .collect::<Database>()
}

fn try_get_database_from_site() -> Option<Database> {
    let mut easy = Easy::new();
    let mut content = String::new();
    easy.url(&crate::config::get_config().itemdb_url).ok()?;
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                content.extend(str::from_utf8(&data));
                Ok(data.len())
            })
            .ok()?;
        transfer.perform().ok()?;
    }

    let entries = content
        .split_terminator("\n")
        .enumerate()
        .map(|(index, line)| {
            let mut entries = line.split_terminator(":");
            entries.next();
            let item_name = String::from(entries.next().unwrap()).replace(" ", "");
            (index as u32, item_name)
        })
        .collect();

    Some(entries)
}

pub fn get_db() -> Database {
    let mut item_db = ITEM_DB.lock().unwrap();
    match &*item_db {
        ItemDB::Uninitialized => {
            let db = {
                if let Some(db) = try_get_database_from_site() {
                    db
                } else {
                    get_compiled_database()
                }
            };
            let _ = mem::replace(&mut *item_db, ItemDB::Initialized(db.clone()));
            db
        }
        ItemDB::Initialized(db) => db.clone(),
    }
}
