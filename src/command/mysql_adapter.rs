use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TableDescriptor{
    field: String,
    data_type: String,
    is_null: String,
    column_key: Option<String>,
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
            "SELECT isc.COLUMN_NAME as field, isc.DATA_TYPE as data_type, isc.IS_NULLABLE as is_null, isc.column_key, isc.extra  
            FROM information_schema.COLUMNS isc WHERE TABLE_NAME ='grades'",
            |(field, data_type, is_null, column_key, extra)|{
                TableDescriptor{field, data_type, is_null, column_key, extra}
            },
        )?;


        println!("fin {:?}", selected_table_descriptor[0]);

        Ok(())



}