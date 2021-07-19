//! Translates photon packets into game messages

//! messages is generated based on assets/messages.json
//! itemdb is generated based on assets/item_ids.txt
//! See build.rs
pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}
pub mod itemdb;

use std::convert::From;

pub use messages::into_game_message;
pub use messages::Message;

/// Player inventory items
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Items {
    pub weapon: Option<String>,
    pub offhand: Option<String>,
    pub helmet: Option<String>,
    pub armor: Option<String>,
    pub boots: Option<String>,
    pub bag: Option<String>,
    pub cape: Option<String>,
    pub mount: Option<String>,
    pub potion: Option<String>,
    pub food: Option<String>,
}

impl From<std::vec::Vec<u32>> for Items {
    fn from(item_array: std::vec::Vec<u32>) -> Self {
        macro_rules! extract {
            ($id:expr) => {
                item_array
                    .get($id)
                    .iter()
                    .filter(|id| **id != &0u32)
                    .map(|code| itemdb::get_db().get(code).map(|s| s.to_string()))
                    .filter_map(|o| o)
                    .next()
            };
        }

        Self {
            weapon: extract!(0),
            offhand: extract!(1),
            helmet: extract!(2),
            armor: extract!(3),
            boots: extract!(4),
            bag: extract!(5),
            cape: extract!(6),
            mount: extract!(7),
            potion: extract!(8),
            food: extract!(9),
        }
    }
}

#[test]
fn test_itemdb_generation() {
    assert_eq!(
        itemdb::get_db().get(&0),
        Some(&String::from("UNIQUE_HIDEOUT"))
    );
}

#[test]
fn test_itemdb_second_access() {
    assert_eq!(
        itemdb::get_db().get(&0),
        Some(&String::from("UNIQUE_HIDEOUT"))
    );
    assert_eq!(
        itemdb::get_db().get(&1),
        Some(&String::from("T1_FARM_CARROT_SEED"))
    );
}
