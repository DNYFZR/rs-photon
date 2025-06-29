// Backend Database Operations
use rusqlite::Connection;
use serde_json;

const DB: &str = "%USERPROFILE%/.photon/app.db"; 

fn connect(database: &str, table:&str) -> Connection {
    let create_table: &str = &format!("CREATE TABLE IF NOT EXISTS {} (name TEXT PRIMARY KEY, data JSON)", table);

    // Open the database, falling back to in-memory if path is not usable
    let conn = match Connection::open(database) {
        Ok(conn) => conn,
        Err(_) => Connection::open_in_memory().expect("failed to open in-memory DB"),
    };

     // Create the tables if it does not exist
     conn.execute(create_table, []).expect("failed to create table");
     
    return conn;
}

#[tauri::command]
pub fn query(database: &str, table:&str, sql: &str) -> Vec<String> {
    //  Query the DB table
    let conn = connect(database, table);
    
    let mut stmt = conn.prepare(sql)
        .expect("database rejected the query ststement");
    
    // Need to add better row extraction here - get all ???
    let rows = stmt.query_map([], |row| row.get::<usize, String>(0))
        .expect("query error");

    let mut data:Vec<String> = Vec::new();
    
    for (idx, item) in rows.enumerate() {
        match item {
            Ok(res) => data.push(res),
            Err(e) => println!("error : no data at index {idx} ( {e} )"),
        }
    }

    println!("{:#?}", data);

    return data;   
}

#[tauri::command]
pub fn update(table:&str, name:&str, data:Vec<String>) -> String {
    let conn = connect(DB, table);

    // Insert or update the data in the api_data table
    match serde_json::to_string(&data) {
        Ok(res) => {
            conn.execute(&format!("INSERT OR REPLACE INTO {} (name, data) VALUES (?1, ?2)", table), (name, res),
                ).expect(&format!("failed to update {} with key {}", table, name ));
            return String::from("ok");
        },

        Err(e) => {
            println!("error : {e}");
            return String::from("error : {e}");
        },
     }
}

#[tauri::command]
pub fn delete(table:&str, name:&str) -> String {
    let conn = connect(DB, table);
    let query = conn.execute(&format!("DELETE FROM {} WHERE name = ?1", table), (name, ));
    
    match query {
        Ok(_) => return String::from("ok"),
        Err(e) => {
            println!("error : {e}");
            return String::from("error : {e}");
        },
     }
}

// #[cfg(test)]
// mod db_testing {
//     use std::fs::File;
//     use std::io::Read;
//     use super::{DB, Repository, connect, query, update, delete};
    
//     const TEST_TABLE:&str = "testing";

//     #[test]
//     fn test_connect() {
//         let conn = connect(DB, TEST_TABLE);
//         let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name;")
//             .expect("failed to prepare execution statement");
//         let rows = stmt.query_map([], |row| row.get::<usize, String>(0))
//             .expect("No results from query");

//         let mut names = Vec::new();
        
//         for name_result in rows {
//             names.push(name_result);
//         }

//         println!("{:#?}", names);  
//     }

//     #[test]
//     fn test_query() {
//         let sql = "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name;";
//         let test = query(DB, TEST_TABLE, sql);

//         assert_eq!(vec!["api_data", "testing"], test);
//     }

//     #[test]
//     fn test_update() {
//         let test_name = "rust";
        
//         let mut test_data = String::new();
//         File::open("testing/rust.json")
//             .expect("could not access test data")
//             .read_to_string(&mut test_data)
//             .expect("could not parse test data");
        
//         let test_data:Vec<Repository> = serde_json::from_str(&test_data).expect("failed to parse to JSON"); 
            
//         // Update DB
//         match update(TEST_TABLE, &test_name, test_data) {
//             Ok(_) => println!("DB table {TEST_TABLE} updated at key {test_name}"),
//             Err(e) => println!("Error : {e}"),
//         }
//     } 

//     #[test]
//     fn test_delete() {
//         let table = "delete_test";

//         // Bring in test dataset
//         let mut test_data = String::new();
//         File::open("testing/rust.json")
//             .expect("could not access test data")
//             .read_to_string(&mut test_data)
//             .expect("could not parse test data");
        
//         // Add test table for delete
//         let test_data:Vec<Repository> = serde_json::from_str(&test_data).expect("failed to parse to JSON"); 
//         update("testing", table, test_data)
//             .expect("failed to update delete_test table");

//         // Test delete method
//         let test = delete("testing", table)
//             .expect("failed to execute delete test");
        
//         assert_eq!(test, ());
//     }

// }
