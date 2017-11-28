extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use std::thread;

use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;

fn main() {
    let manager = ConnectionManager::<SqliteConnection>::new("db.sqlite");
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    for _ in 0..10i32 {
        let pool = pool.clone();
        thread::spawn(move || {
            let connection = pool.get();

            assert!(connection.is_ok());
        });
    }
}
