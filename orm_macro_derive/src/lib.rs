extern crate proc_macro;

use core::panic;

use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Attribute, DeriveInput, Ident};
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
    let input = parse_macro_input!(struc as DeriveInput);

    let attrs = input.attrs;

    let table_name = attrs.first().map(|name| {

        match &name.meta {
            syn::Meta::List(data) => data.tokens.to_string(),
            _ => panic!("The attribute should look like this #[table_name(your_table_name)]")
            
        }

    } ).unwrap_or_else(|| panic!(r#"#[table_name(your_table_name)] attribute is necessary to indicate which table the methods will affect"#));

    let struct_name_raw = &input.ident;

    let new_struct_name = format_ident!("{}Orm", struct_name_raw);

    let fields: Vec<String> = match input.data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => fields
                .named
                .iter()
                .map(|fiel| fiel.ident.as_ref().unwrap().to_string())
                .collect(),
            _ => panic!("This macro only works on structs with named fields"),
        },
        _ => panic!("This macro only works on structs"),
    };

    //This part is only concerned about having the structs's properties / data
    impl_repository(StructData::new(fields, new_struct_name, table_name))
}

fn impl_repository(struc_data: StructData) -> TokenStream {
    let orm_struct_name = struc_data.struct_name;

    let table_name = struc_data.table_name.clone();

    //for update sql statement we dont want the id to appear
    let fields_ignoring_id: Vec<String> = struc_data
        .fields
        .iter()
        .filter(|field| *field != "id")
        .map(|field| field.to_string())
        .collect();

    let mut update_where_condition = String::new();

    #[cfg(feature = "postgres")]
    update_where_condition.push_str(format!("id = ${}", fields_ignoring_id.len() + 1).as_str());

    #[cfg(not(feature = "postgres"))]
    update_where_condition.push_str("id = ?");

    let mut update_builder = UpdateStatement::new(&struc_data.table_name, WhereClause::new());

    update_builder
        .set_fields(fields_ignoring_id.clone())
        .set_where(vec![update_where_condition])
        .set_returning_clause(ReturningClause::new(&fields_ignoring_id));

    let select_builder = SelectStatement::new(&struc_data.fields, &struc_data.table_name);

    let mut delete_builder = DeleteStatement::new(&struc_data.table_name, WhereClause::new());

    delete_builder.set_returning_clause(ReturningClause::new(&fields_ignoring_id));

    let mut insert_builder = InsertStatement::new(
        &struc_data.table_name,
        &fields_ignoring_id,
        fields_ignoring_id.clone(),
    );

    insert_builder.set_returning_clause(ReturningClause::new(&fields_ignoring_id));

    let select_statement = select_builder.build_sql();
    let update_statement = update_builder.build_sql();
    let delete_statement = delete_builder.build_sql();
    let insert_statement = insert_builder.build_sql();

    let find_method_docs = format!("Generates this sql: {}", select_statement);
    let find_method = quote! {
        #[doc = #find_method_docs]
        fn find(&self) -> String {

            if self.select_fields.is_empty() {
                return #select_statement.into()

            }


            format!("SELECT {} FROM {}", self.select_fields, #table_name.to_string())

        }
    };

    let create_method_docs = format!("Generates this sql: {}", insert_statement);
    let create_method = quote! {

        #[doc = #create_method_docs]
        fn create(&self) -> &str {

            #insert_statement

        }

    };

    let update_method_docs = format!("Generates this sql: {}", update_statement);
    let update_method = quote! {
        #[doc = #update_method_docs]
        fn update(&self) -> &str {
            #update_statement
        }
    };

    let delete_method_docs = format!("Generates the following sql: {}", delete_statement);
    let delete_method = quote! {

        #[doc = #delete_method_docs]
        fn delete(&self) -> &str {


        #delete_statement

        }
    };

    let declare_struct = quote! {

    #[derive(Debug)]
    pub struct #orm_struct_name {
        select_fields : String,

    }

    };

    quote! {

    #declare_struct


    impl #orm_struct_name {

        ///Instanciates a new OrmRepository builder with the structs properties as table fields
        pub fn builder() -> Self {
            Self { select_fields : "".into() }
        }

    }

    impl OrmRepository for #orm_struct_name {

        #find_method

        #create_method

        #delete_method

        #update_method


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
