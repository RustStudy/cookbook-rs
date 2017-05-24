/*
  使用lazy_static
*/

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref PRIVILEGES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("James", vec!["user", "admin"]);
        map.insert("Jim", vec!["user"]);
        map
    };
}

pub fn show_access(name: &str) -> &Vec<&'static str> {
    let access = PRIVILEGES.get(name);
    println!("{}: {:?}", name, access.unwrap());
    access.unwrap()
}


// 修改全局可变状态
lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert(fruit: &str) -> Result<(), String> {
    // acquire exclusive access
    let mut db = FRUIT.lock()
        .map_err(|_| "Failed to acquire MutexGuard")?;
    db.push(fruit.to_string());
    Ok(())
    // release exclusive access
}

pub fn run() -> Result<(), String> {
    insert("apple")?;
    insert("orange")?;
    insert("peach")?;
    {
        // acquire access
        let db = FRUIT.lock()
            .map_err(|_| "Failed to acquire MutexGuard")?;

        for (i, item) in db.iter().enumerate() {
            println!("{}: {}", i, item);
        }
        // release access
    }
    insert("grape")?;

    Ok(())
}
