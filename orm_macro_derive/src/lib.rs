extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};
use utils::*;

mod  utils;


#[allow(dead_code)]
#[proc_macro_derive(GetRepository, attributes(table_name))]
pub fn get_repository(struc: TokenStream) -> TokenStream {
    let input = parse_macro_input!(struc as DeriveInput);

    impl_repository(input)
}

fn impl_repository(struc: DeriveInput) -> TokenStream {

    let attrs = struc.attrs;

    let the_real_table_name = attrs.iter().last().unwrap();

    let the_real_table_name = extract_string_atribute(the_real_table_name.to_token_stream().to_string());

    let struct_name_raw = &struc.ident;

    let struct_name = struct_name_raw.to_string().to_lowercase();

    let new_struct_name = format_ident!("{}Orm", struct_name_raw);

    let mut insert_fields = String::new();

    let mut insert_values_fields = String::new();

    let mut update_fields = String::new();

     match struc.data {
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

            let where_clause_in_update_clause = format!(" WHERE id = ${}", update_fields.split(',').count() + 1 );

            update_fields.push_str(where_clause_in_update_clause.as_str());


        }
        _ => unimplemented!(),
    };

     quote! {

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
    #insert_values_fields.to_string(), name : #the_real_table_name.to_string(), update_fields : #update_fields.to_string() }
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

    }.into()

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
        assert_eq!("table_name_extracted", extract_string_atribute(attribute_from_struct.to_string()))
    }

}
