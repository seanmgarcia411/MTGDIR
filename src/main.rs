use std::path::Path;

fn main() {
    //make the initial small test db
    let connection = 
    if !Path::new("Database\\RefDB").exists() {
        let connection = sqlite::open("Database\\RefDB").unwrap();

        let query = "
            CREATE TABLE Cards (CardName TEXT);
            INSERT INTO Cards VALUES ('BlueInstant');
            INSERT INTO Cards VALUES ('RedEnchantment');
            INSERT INTO Cards VALUES ('WhiteSorcery');
            INSERT INTO Cards VALUES ('BlackCreature');
        ";
        connection.execute(query).unwrap();

        connection
    }
    else {
        let connection = sqlite::open("Database\\RefDB").unwrap();

        connection
    };


}