extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DataStruct, DeriveInput};

#[proc_macro_derive(GetRepository)]
pub fn get_repository(struc: TokenStream) -> TokenStream {
    let input = parse_macro_input!(struc as DeriveInput);

    // Get the name of the struct or enum we are deriving the trait for
    let struct_name_raw = &input.ident;

    impl_repository(struct_name_raw, input.data)

    // Convert the generated implementation back into a token stream
}

fn impl_repository(struc_name: &syn::Ident, fields: syn::Data) -> TokenStream {
    let struct_name = struc_name.to_string().to_lowercase();

    let new_struct_name = format_ident!("{}OrmRepository", struc_name);

    let mut mysql_builder = MysqlBuilder::default();

    mysql_builder.set_table_name(struct_name.clone());

    let mut insert_fields = String::new();

    let mut insert_values_fields = String::new();

    let mut update_fields = String::new();

    let struct_fields = match fields {
        syn::Data::Struct(ref data) => {
            for (index, fieldname) in data.fields.iter().enumerate() {
                let current_value = index + 1;
                let fieldname = fieldname.ident.as_ref().unwrap().to_string();
                insert_fields.push_str(
                    format!("{},", fieldname).as_str()
                );
                insert_values_fields.push_str(format!("${},", current_value).as_str());

                update_fields.push_str(format!("{} = ${},", fieldname, current_value ).as_str())
            }
            insert_fields.pop();
            insert_values_fields.pop();
            data
        }
        syn::Data::Enum(_) => unimplemented!(),
        syn::Data::Union(_) => unimplemented!(),
    };




    // Generate the implementation for the trait
    let expanded = quote! {

    #[derive(Debug)]
    pub struct #new_struct_name {

    name : String,
    select_fields : String,
    fields : String,
    insert_values_fields : String,
    update_fields : String,

    }


    impl #new_struct_name {

    pub fn builder() -> Self {

    Self { select_fields : "".into() , fields : #insert_fields.to_string(), insert_values_fields :
    #insert_values_fields.to_string(), name : #struct_name.to_string(), update_fields : #update_fields.to_string() }
    }
    }

    impl OrmRepository for #new_struct_name {

    fn find(&self) -> String {

    if self.select_fields.is_empty() {

    return format!("SELECT {} FROM {}", self.fields, #struct_name.to_string())
    }

    format!("SELECT {} FROM {}", self.select_fields, #struct_name.to_string())


    }

    fn create(&mut self) -> String {

    format!("INSERT INTO {} ({}) VALUES ({}) RETURNING {}", #struct_name.to_string(), self.fields,
    self.insert_values_fields, self.fields)

    }


    fn delete(&self) -> String {

    format!("DELETE FROM {} WHERE id = $1 RETURNING {}", #struct_name.to_string(), self.fields )

    }

    fn update(&self) -> String {
        
        format!("UPDATE {} SET {}", self.name, self.update_fields)
        
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
