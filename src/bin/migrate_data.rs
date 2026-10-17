use rusqlite::Connection;
fn main()->Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to the new database...");
    // Connection must be mutable to create a transaction
    let mut connection = Connection::open("server_db/server.db")?;

    // 1. Attach must happen outside (before) the transaction
    println!("Attaching old database...");
    connection.execute("ATTACH DATABASE 'out/alpr_events.db' AS old_db;", [])?;

    // 2. Begin the transaction
    println!("Starting transaction...");
    let tx = connection.transaction()?;

    println!("Inserting location...");
    let insert_location = concat!("INSERT OR IGNORE INTO location(id, name, description) ",
        "VALUES('TCEV', 'Town and Country Executive Village', 'LEGACY-TESTING')",
    );
    tx.execute(insert_location, [])?;

    println!("Inserting sublocation...");
    let insert_sublocation = concat!("INSERT OR IGNORE INTO sublocation(id, name, description, location_id) ",
        "VALUES('Balimbing', 'Balimbing St.', 'LEGACY-TESTING', 'TCEV')",
    );
    tx.execute(insert_sublocation, [])?;

    println!("Inserting cameras...");
    let insert_camera_front = concat!("INSERT OR IGNORE INTO camera(id, name, description, sublocation_id, orientation) ",
        "VALUES('InSight LPR_Front', 'InSight LPR_Front', 'Testing Camera', 'Balimbing', 'Inbound')",
    );
    tx.execute(insert_camera_front, [])?;

    let insert_camera_rear= concat!("INSERT OR IGNORE INTO camera(id, name, description, sublocation_id, orientation) ",
    "VALUES('InSight LPR_Rear', 'InSight LPR_Rear', 'Testing Camera', 'Balimbing', 'Outbound')",
    );
    tx.execute(insert_camera_rear, [])?;


    println!("Copying ALPR events...");
    tx.execute(
        "INSERT OR IGNORE INTO alpr_events (id, camera_id, date, plate, image, plate_image)
         SELECT id, camid, date, plate, image, plate_image FROM old_db.alpr_events;",
        [],
    )?;

    // 3. Commit the transaction
    tx.commit()?;

    println!("Detaching old database...");
    connection.execute("DETACH DATABASE old_db;", [])?;

    println!("Data migration completed successfully!");


    Ok(())
}