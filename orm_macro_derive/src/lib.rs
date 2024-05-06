extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Ident};
use utils::*;

mod utils;

#[derive(Debug)]
struct StructData {
    fields: Vec<String>,
    table_name: String,
    struct_name: Ident,
}

impl StructData {
    fn new(fields: Vec<String>, struct_name: Ident, table_name: String) -> Self {
        Self {
            fields,
            struct_name,
            table_name,
        }
    }
}

#[allow(dead_code)]
#[proc_macro_derive(GetRepository, attributes(table_name))]
pub fn get_repository(struc: TokenStream) -> TokenStream {
    // this part is only concerned about extracting the data from the struct
    let input = parse_macro_input!(struc as DeriveInput);

    let attrs = input.attrs;

    let table_name = attrs.iter().last().unwrap();

    let table_name = extract_string_atribute(table_name.to_token_stream().to_string());

    let struct_name_raw = &input.ident;

    let new_struct_name = format_ident!("{}Orm", struct_name_raw);

    let fields: Vec<String> = match input.data {
        syn::Data::Struct(ref data) => data
            .fields
            .iter()
            .map(|field| field.ident.as_ref().unwrap().to_string())
            .collect(),
        _ => unimplemented!(),
    };

    //This part is only concerned about having the structs's properties / data
    impl_repository(StructData::new(fields, new_struct_name, table_name))
}

fn impl_repository(struc_data: StructData) -> TokenStream {
    let orm_struct_name = struc_data.struct_name;

    //for update sql statement we dont want the id to appear
    let fields_ignoring_id : Vec<String> = struc_data.fields.iter().filter(|field| *field != "id").map(|field| field.to_string()).collect();

    
    let mut update_where_condition  = String::new();
    update_where_condition.push_str(format!("id = ${}", fields_ignoring_id.len() + 1).as_str());


    let mut update_builder = UpdateStatement::new(&struc_data.table_name, WhereClause::new());

    update_builder.set_fields(fields_ignoring_id.clone())
        .set_where(vec![update_where_condition])
        .set_returning_clause(ReturningClause::new(&fields_ignoring_id));

    let mut select_builder = SelectStatement::new(&struc_data.fields, &struc_data.table_name);



    let mut delete_builder = DeleteStatement::new(&struc_data.table_name, WhereClause::new());

    let mut insert_builder = InsertStatement::new(&struc_data.table_name, &fields_ignoring_id, fields_ignoring_id.clone());

    insert_builder.set_returning_clause(ReturningClause::new(&fields_ignoring_id));

    let select_statement = select_builder.build_sql();
    let update_statement = update_builder.build_sql();
    let delete_statement = delete_builder.build_sql();
    let insert_statement = insert_builder.build_sql();


    let find_method = quote! {
        /// Generates a SELECT struct_properties FROM table_name sql clause
        fn find(&self) -> &str {
            #select_statement
        }
    };

    let create_method = quote! {

        /// Generates a INSERT INTO table_name (properties) VALUES (placeholders) RETURNIN properties sql
        /// clause
        fn create(&mut self) -> &str {

            #insert_statement

        }

    };


    let update_method = quote! {
        /// generates a UPDATE table_name SET property1 = $, ... WHERE id = $ sql clause
        fn update(&self) -> &str {
            #update_statement
        }
    };

    let delete_method = quote! { 

        ///Generates a DELETE FROM table_name WHERE id = ${} RETURNIN properties sql clause
        fn delete(&self) -> &str {


        #delete_statement

        }
    };

    quote! {

    #[derive(Debug)]
    pub struct #orm_struct_name {}


    impl #orm_struct_name {

        ///Instanciates a new OrmRepository builder with the structs properties as table fields
        pub fn builder() -> Self {
            Self {}
        }

    }

    impl OrmRepository for #orm_struct_name {

        #find_method

        #create_method

        #delete_method

        #update_method


    }

    }
    .into()
}

#[cfg(test)]
mod tests {
    use crate::utils::*;

    #[test]
    fn extracts_table_name() {
        let attribute_from_struct = r#"table_name("table_name_extracted")"#;
        assert_eq!(
            "table_name_extracted",
            extract_string_atribute(attribute_from_struct.to_string())
        )
    }
}
