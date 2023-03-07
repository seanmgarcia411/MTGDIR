use std::{path::Path, collections::HashMap};

#[macro_export]
macro_rules! create_statement {
    ($con:ident, $statement:expr, $($x:expr),*) => {{
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
        create_table("Cards", &connection);
        add_single_card("BlueInstant", &connection);
        add_single_card("RedEnchantment", &connection);
        add_single_card("WhiteSorcery", &connection);
        add_single_card("BlackCreature", &connection);
        
        connection
    }
    else {
        let connection = sqlite::open("Database\\RefDB").unwrap();

        connection
    };
    list_all_cards(&connection);
}

fn list_all_cards(connection: &sqlite::Connection) {
    let sel_con = "SELECT * FROM Cards";
    connection
    .iterate(sel_con, |pairs| {
        for &(card_name, value) in pairs.iter() {
            println!("{} = {}", card_name, value.unwrap());
        }
        true
    })
    .unwrap();
}

fn add_single_card(card_name: &str, connection: &sqlite::Connection) {
    let ins_con = format!("INSERT INTO Cards VALUES (@card_name);");
    let mut ins_statement = create_statement!(connection, ins_con, card_name);
    execute_update(ins_statement);
}

fn create_table(table_name: &str, connection: &sqlite::Connection) {
    let table_con = format!("CREATE TABLE {table_name} (card_name TEXT);");
    let mut make_table_statement = create_statement!(connection, table_con,);
    execute_update(make_table_statement);
}

/// Runs an update statement
fn execute_update(mut stmt: sqlite::Statement) -> Result<(),()> {
    if let sqlite::State::Done = stmt.next().unwrap() {
        return Ok(());
    }
    Err(())
}