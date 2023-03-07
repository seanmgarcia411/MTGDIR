use std::path::Path;

#[macro_export]
macro_rules! create_statement {
    ($con:ident, $statement:expr, $($x:expr)*,) => {{
        #[allow(unused_mut)]
        let mut statement = $con.prepare($statement).expect("Unable to prepare statement");
        let mut _i = 1usize;
    $(
        statement.bind((_i, $x)).unwrap();
        _i +=1;
    )*
    statement
    }};
}

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

    let selcon = create_statement!(connection, "SELECT CardName FROM Cards",);
    selcon.map;
}