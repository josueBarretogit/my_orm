pub trait OrmRepository {
    fn find(&self) -> String;
    fn select(&mut self, fields: Vec<&str>) -> &mut Self;
}

#[cfg(test)]
mod tests {

    
    use crate::OrmRepository;

    #[derive(Default,orm_macro_derive::GetRepository)]
    struct Entity {
        title: String,
        description: String,
        others: Vec<u32>,
    }

    #[test]
    fn find_method_return_select() {

        assert_eq!("SELECT * FROM entity", EntityOrmRepository::builder().find())
    }

    #[test]
    fn find_method_queries_specific_struct_properties() {
        
        assert_eq!(
            "SELECT title, description FROM entity",
            EntityOrmRepository::builder()
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
