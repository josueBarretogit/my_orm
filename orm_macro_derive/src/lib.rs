extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Ident};
use utils::*;

mod utils;

#[allow(dead_code)]
#[proc_macro_derive(GetRepository, attributes(table_name))]
pub fn get_repository(struc: TokenStream) -> TokenStream {
    let input = parse_macro_input!(struc as DeriveInput);

    let attrs = input.attrs;

    let the_real_table_name = attrs.iter().last().unwrap();

    let the_real_table_name =
        extract_string_atribute(the_real_table_name.to_token_stream().to_string());

    let struct_name_raw = &input.ident;

    let new_struct_name = format_ident!("{}Orm", struct_name_raw);

    let mut insert_fields = String::new();

    let mut insert_values_fields = String::new();

    let mut update_fields = String::new();

    match input.data {
        syn::Data::Struct(ref data) => {
            for (index, fieldname) in data.fields.iter().enumerate() {
                let current_value = index + 1;
                let fieldname = fieldname.ident.as_ref().unwrap().to_string();
                insert_fields.push_str(format!("{},", fieldname).as_str());
                insert_values_fields.push_str(format!("${},", current_value).as_str());

                update_fields.push_str(format!("{} = ${},", fieldname, current_value).as_str())
            }

            insert_fields.pop();

            insert_values_fields.pop();
            update_fields.pop();
        }
        _ => unimplemented!(),
    };

    impl_repository(
        new_struct_name,
        insert_fields,
        insert_values_fields,
        the_real_table_name,
    )
}

fn impl_repository(
    orm_struct_name: Ident,
    fields: String,
    insert_values_fields: String,
    the_real_table_name: String,
) -> TokenStream {
    let mut update_set = fields
        .split(',')
        .enumerate()
        .map(|(index, value)| format!("{} = ${},", value, index + 1))
        .collect::<String>();

    update_set.pop();

    let where_clause_in_update_clause = update_set.split(',').count() + 1;

    quote! {

    #[derive(Debug)]
    pub struct #orm_struct_name {

    name : String,
    select_fields : String,
    fields : String,
    insert_values_fields : String,

    }


    impl #orm_struct_name {

    ///Instanciates a new OrmRepository builder with the structs properties as table fields
    pub fn builder() -> Self {

    Self { select_fields : "".into() , fields : #fields.to_string(), insert_values_fields :
    #insert_values_fields.to_string(), name : #the_real_table_name.to_string() }
    }
    }

    impl OrmRepository for #orm_struct_name {

    /// Generates a SELECT struct_properties FROM table_name sql clause
    fn find(&self) -> String {

    if self.select_fields.is_empty() {

    return format!("SELECT {} FROM {}", self.fields, self.name)
    }

    format!("SELECT {} FROM {}", self.select_fields, self.name)


    }

    /// Generates a INSERT INTO table_name (properties) VALUES (placeholders) RETURNIN properties sql
    /// clause
    fn create(&mut self) -> String {

    format!("INSERT INTO {} ({}) VALUES ({}) RETURNING id,{}", self.name, self.fields,
    self.insert_values_fields, self.fields)

    }


    ///Generates a DELETE FROM table_name WHERE id = ${} RETURNIN properties sql clause
    fn delete(&self) -> String {

    format!("DELETE FROM {} WHERE id = $1 RETURNING {}", self.name, self.fields )

    }


    /// generates a UPDATE table_name SET property1 = $, ... WHERE id = $ sql clause
    fn update(&self) -> String {


    format!("UPDATE {} SET {} WHERE id = ${} RETURNING id,{}", self.name, #update_set.to_string(),
    #where_clause_in_update_clause.to_string(), self.fields)

    }

    /// Used to select specific properties, but its easier to make a Dto and derive OrmRepository
    /// instead of using this
    fn select_fields(&mut self, fields : Vec<&str>) -> &mut Self {
        for field in fields {

        self.select_fields.push_str(field);

        self.select_fields.push_str(", ");

        }

        self.select_fields.pop();
        self.select_fields.pop();

        self
        }

        }

        }
    .into()
}

trait SQLBuilder {
    fn set_table_name(&mut self, entity_name: &str) -> &mut Self;
    fn generate_simple_select_statement(&self) -> String;
    fn generate_select_statement_with_fields(&self, field: Vec<&str>) -> String;
}

#[derive(Default)]
struct MysqlBuilder {
    table_name: String,
}

impl SQLBuilder for MysqlBuilder {
    fn set_table_name(&mut self, entity_name: &str) -> &mut Self {
        self.table_name = entity_name.to_string();
        self
    }

    fn generate_select_statement_with_fields(&self, fields: Vec<&str>) -> String {
        let mut select_statement = String::from("SELECT ");
        for field in fields {
            select_statement.push_str(field);
        }
        select_statement.push_str("FROM ");
        select_statement.push_str(&self.table_name);
        select_statement
    }

    fn generate_simple_select_statement(&self) -> String {
        format!("SELECT * FROM {}", self.table_name)
    }
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
