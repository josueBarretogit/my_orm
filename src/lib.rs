extern crate my_orm_macro_derive;

pub trait Repository {
    fn find(&self) -> String;
    fn select(&mut self, fields: Vec<&str>) -> &mut Self;
}

#[cfg(test)]
mod tests {

    use crate::Repository;

    #[derive(Default, my_orm_macro_derive::GetRepository)]
    struct Entity {
        title: String,
        description: String,
        others: Vec<u32>,
    }

    #[test]
    fn find_method_return_select() {
        let enti = Entity::default();

        assert_eq!("SELECT * FROM entity", NewRepository::default().find())
    }

    #[test]
    fn find_method_queries_specific_struct_properties() {
        let mut entity = Entity::default();
        assert_eq!(
            "SELECT title, description FROM entity",
            NewRepository::default()
                .select(vec!["title", "description"])
                .find()
        )
    }

    // #[test]
    // fn find_method_queries_with_where_clause() {
    // let entity = Entity::default();
    // assert_eq!(
    // "SELECT * FROM entity where title = something",
    // entity.find(FindOptions {
    // whereOptions: WhereOptions { title: "something" }
    // })
    // )
    // }
}
