

use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TableDescriptor{
    table_name: String,
    table_descriptor: Vec<TableDescriptorDetail>
}

#[derive(Debug, PartialEq, Eq)]
pub struct TableDescriptorDetail{
    field: String,
    data_type: String,
    is_null: String,
    column_key: Option<String>,
    extra: Option<String>,
}



#[derive(Debug, PartialEq, Eq)]
pub struct TableName{
    table_name: String,
}



pub fn get_all_tables() -> std::result::Result<Vec<TableName>, Box<dyn std::error::Error>>{
    let url = "mysql://root:root@192.168.1.150:3306/mis-notas-db";
    // jdbc:mariadb://192.168.1.150:3306/mis-notas-db
    //root

    let tables: Vec<TableName> = Vec::new();

    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    let tables = conn
    .query_map(
        "select t.TABLE_NAME as table_name from information_schema.TABLES t WHERE t.TABLE_SCHEMA = 'mis-notas-db'",
        |(table_name)|{
            TableName{table_name}
        },
    )?;

   

    Ok(tables)

}

pub fn describe_table() -> std::result::Result<(), Box<dyn std::error::Error>>{
    let url = "mysql://root:root@192.168.1.150:3306/mis-notas-db";


    let tables = get_all_tables().unwrap_or_else(|error|{
        panic!("error: {:?}", error);
    });
    

    let mut table_descriptor_list: Vec<TableDescriptor> = Vec::new();

    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    for table_name_iter in tables.iter() {
        println!("table name: {:?}", &table_name_iter);
        let table = &table_name_iter.table_name;

        let mut query = format!("SELECT isc.COLUMN_NAME as field, isc.DATA_TYPE as data_type, 
        isc.IS_NULLABLE as is_null, isc.column_key, isc.extra FROM information_schema.COLUMNS isc WHERE TABLE_NAME = '{table}'").to_owned();


        let selected_table_descriptor = conn
        .query_map(query
            ,
            |(field, data_type, is_null, column_key, extra)|{
                TableDescriptorDetail{field, data_type, is_null, column_key, extra}
            },
        )?;

        let table_descriptor = TableDescriptor{
            table_descriptor: selected_table_descriptor,
            table_name: table.clone()
        };

        table_descriptor_list.push(table_descriptor);

    }

    for table_descriptor in table_descriptor_list {
        println!("table desceiptor detail for {}", table_descriptor.table_name);
        for table_detail in table_descriptor.table_descriptor.iter() {
            println!("table detail {:?}",table_detail);
            
        }
        
    }





    Ok(())



}