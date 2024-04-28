extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(GetRepository)]
pub fn get_repository(struc: TokenStream) -> TokenStream {
    let input = parse_macro_input!(struc as DeriveInput);

    // Get the name of the struct or enum we are deriving the trait for
    let struct_name_raw = &input.ident;

    let struct_name = struct_name_raw.to_string().to_lowercase();

    let mysql_builder = MysqlBuilder::default();

    let find_statement = mysql_builder
        .set_table_name(struct_name)
        .generate_simple_select_statement();

    // Generate the implementation for the trait
    let expanded = quote! {

    impl Repository for #struct_name_raw {
    fn find(&self) -> String {
    #find_statement.to_string()
    }

    fn select(self, fields : Vec<&str>) -> Self {

        self
        }

        }
        };

    // Convert the generated implementation back into a token stream
    expanded.into()
}

trait SQLBuilder {
    fn set_table_name(self, entity_name: String) -> Self;
    fn generate_simple_select_statement(&self) -> String;
    fn generate_select_statement_with_fields(&self, field: Vec<&str>) -> String;
}

#[derive(Default)]
struct MysqlBuilder {
    table_name: String,
}

impl SQLBuilder for MysqlBuilder {
    fn set_table_name(mut self, entity_name: String) -> Self {
        self.table_name = entity_name;
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
