extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(GetRepository)]
pub fn get_repository(struc: TokenStream) -> TokenStream {
    let input = parse_macro_input!(struc as DeriveInput);

    // Get the name of the struct or enum we are deriving the trait for
    let struct_name_raw = &input.ident;

    let struct_name = struct_name_raw.to_string().to_lowercase();

    let new_struct_name = format_ident!("{}OrmRepository", struct_name_raw);

    let mut mysql_builder = MysqlBuilder::default();

    mysql_builder.set_table_name(struct_name.clone());

    let find_statement = mysql_builder.generate_simple_select_statement();

    let mut insert_fields = String::new();

    let mut insert_values_fields = String::new();

    let _struct_fields = match input.data {
        syn::Data::Struct(ref data) => {
            for (index, fieldname) in data.fields.iter().enumerate() {
                let current_value = index + 1;
                insert_fields.push_str(
                    format!("{},", fieldname.ident.as_ref().unwrap().to_string()).as_str(),
                );
                insert_values_fields.push_str(format!("${},", current_value).as_str());
            }
            insert_fields.pop();
            insert_values_fields.pop();
        }
        syn::Data::Enum(_) => unimplemented!(),
        syn::Data::Union(_) => unimplemented!(),
    };

    // Generate the implementation for the trait
    let expanded = quote! {

    #[derive(Debug)]
    pub struct #new_struct_name {

    select_fields : String,
    insert_fields : String,
    insert_values_fields : String,

    }


    impl #new_struct_name {

    pub fn builder() -> Self {

    Self { select_fields : "".into() , insert_fields : "".into(), insert_values_fields : "".into() }
    }
    }

    impl OrmRepository for #new_struct_name {

    fn find(&self) -> String {
    if self.select_fields.is_empty() {

    #find_statement.to_string()
    } else {

    format!("SELECT {} FROM {}", self.select_fields, #struct_name.to_string() )

    }

    }

    fn create(&mut self) -> String {

    self.insert_fields = #insert_fields.to_string();
    self.insert_values_fields = #insert_values_fields.to_string();


    format!("INSERT INTO {} ({}) VALUES ({})", #struct_name.to_string(), self.insert_fields, self.insert_values_fields)

    }




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

        };

    // Convert the generated implementation back into a token stream
    expanded.into()
}

trait SQLBuilder {
    fn set_table_name(&mut self, entity_name: String) -> &mut Self;
    fn generate_simple_select_statement(&self) -> String;
    fn generate_select_statement_with_fields(&self, field: Vec<&str>) -> String;
}

#[derive(Default)]
struct MysqlBuilder {
    table_name: String,
}

impl SQLBuilder for MysqlBuilder {
    fn set_table_name(&mut self, entity_name: String) -> &mut Self {
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
