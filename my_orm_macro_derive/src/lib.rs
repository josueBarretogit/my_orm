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

    let find_statement = mysql_builder.generate_select_statement(struct_name);
    
    // Generate the implementation for the trait
    let expanded = quote! {

    impl Repository for #struct_name_raw {
    fn find(&self) -> String {
        #find_statement.to_string()
    }

    }
    };

    // Convert the generated implementation back into a token stream
    expanded.into()
}

trait SQLBuilder {
    fn generate_select_statement(&self, entity_name : String) -> String;
}



#[derive(Default)]
struct MysqlBuilder {}

impl SQLBuilder for MysqlBuilder {
    fn generate_select_statement(&self, entity_name : String) -> String {
        format!("SELECT * FROM {}", entity_name)
    }
    
}
