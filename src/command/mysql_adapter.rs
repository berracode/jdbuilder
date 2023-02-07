use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TableDescriptor{
    field: String,
    Type: String,
    is_null: String,
    key: Option<String>,
    extra: Option<String>,
}

pub fn describe_table() -> std::result::Result<(), Box<dyn std::error::Error>>{
    let url = "mysql://root:root@192.168.1.150:3306/mis-notas-db";
    // jdbc:mariadb://192.168.1.150:3306/mis-notas-db
    //root

    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    let selected_table_descriptor = conn
        .query_map(
            "describe grades",
            |(field, field_type, is_null, key, extra)|{
                TableDescriptor{field, field_type, is_null, key, extra}
            },
        )?;


        println!("fin {}", selected_table_descriptor.len());

        Ok(())



}