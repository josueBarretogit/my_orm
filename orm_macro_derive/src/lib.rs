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
    id_table: String,
    struct_name: Ident,
}

impl StructData {
    fn new(fields: Vec<String>, struct_name: Ident, table_name: String, id_table: String) -> Self {
        Self {
            fields,
            struct_name,
            table_name,
            id_table,
        }
    }
}

#[allow(dead_code)]
#[proc_macro_derive(GetRepository, attributes(table_name, id))]
pub fn get_repository(struc: TokenStream) -> TokenStream {
    let input = parse_macro_input!(struc as DeriveInput);

    let attrs = input.attrs;

    let table_name = attrs.first().map(|name| {

        match &name.meta {
            syn::Meta::List(data) => data.tokens.to_string(),
            _ => panic!("The attribute should look like this #[table_name(your_table_name)]")
        }

    } ).unwrap_or_else(|| panic!(r#"#[table_name(your_table_name)] attribute is necessary to indicate which table the methods will affect"#));

    let id_table = if attrs.iter().len() > 1 {
        attrs
            .last()
            .map(|id| match &id.meta {
                syn::Meta::List(data) => data.tokens.to_string(),
                _ => panic!("the attribute should look like this:"),
            })
            .unwrap()
    } else {
        panic!("set the table id like this: #[id(your_table_id)]");
    };

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
    impl_repository(StructData::new(
        fields,
        new_struct_name,
        table_name,
        id_table,
    ))
}

fn impl_repository(struc_data: StructData) -> TokenStream {
    let orm_struct_name = struc_data.struct_name;

    let table_name = struc_data.table_name.clone();
    let id_name = struc_data.id_table;
    let fields = struc_data.fields;

    let mut update_where_condition = String::new();

    #[cfg(feature = "postgres")]
    update_where_condition.push_str(format!("{} = ${}", id_name, fields.len() + 1).as_str());

    #[cfg(not(feature = "postgres"))]
    update_where_condition.push_str(format!("{} = ?", id_name).as_str());

    let mut update_builder = UpdateStatement::new(&struc_data.table_name, WhereClause::new());

    update_builder
        .set_fields(fields.clone())
        .set_where(vec![update_where_condition])
        .set_returning_clause(ReturningClause::new(&fields, &id_name));

    let select_builder = SelectStatement::new(&fields, &table_name);

    let mut delete_builder = DeleteStatement::new(&struc_data.table_name, WhereClause::new());

    let mut where_clause_delete = WhereClause::new();

    #[cfg(feature = "postgres")]
    where_clause_delete.set_conditions(vec![format!("{} = $1", id_name)]);

    #[cfg(not(feature = "postgres"))]
    where_clause_delete.set_conditions(vec![format!("{} = ?", id_name)]);

    delete_builder.set_where_clause(where_clause_delete);

    delete_builder.set_returning_clause(ReturningClause::new(&fields, &id_name));

    let mut insert_builder = InsertStatement::new(&struc_data.table_name, &fields, fields.clone());

    insert_builder.set_returning_clause(ReturningClause::new(&fields, &id_name));

    let select_statement = select_builder.build_sql();
    let update_statement = update_builder.build_sql();
    let delete_statement = delete_builder.build_sql();
    let insert_statement = insert_builder.build_sql();

    let mut find_by_id_builder = SelectStatement::new(&fields, &table_name);

    let mut where_find_by_id_builder = WhereClause::new();

    #[cfg(feature = "postgres")]
    where_find_by_id_builder.set_conditions(vec![format!("{} = $1", id_name)]);

    #[cfg(not(feature = "postgres"))]
    where_find_by_id_builder.set_conditions(vec![format!("{} = ?", id_name)]);

    find_by_id_builder.set_where(where_find_by_id_builder);

    let find_by_id_statement = find_by_id_builder.build_sql();

    let find_method_docs = format!("Generates this sql: {}", select_statement);
    let find_method = quote! {
        #[doc = #find_method_docs]
        fn find(&self) -> &str {

                 #select_statement

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

    let find_by_id_method_name = format_ident!("find_by_{}", id_name);

    let find_by_id_method_docs = format!("Generates the following sql: {}", find_by_id_statement);

    let find_by_id_method = quote! {
        #[doc = #find_by_id_method_docs]
        fn #find_by_id_method_name(&self) -> &str {
        #find_by_id_statement
        }
    };

    let declare_struct = quote! {

    #[derive(Debug)]
    pub struct #orm_struct_name {

    }

    };

    quote! {

    #declare_struct


    impl #orm_struct_name {

        ///Instanciates a new OrmRepository builder with the structs properties as table fields
        pub fn builder() -> Self {
            Self {}
        }

        #find_by_id_method


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
