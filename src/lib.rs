extern crate orm_macro_derive;

pub trait OrmRepository {
    fn find(&self) -> String;
    fn select_fields(&mut self, fields: Vec<&str>) -> &mut Self;
    fn create(&self) -> String;
}

#[cfg(test)]
mod tests {

    use crate::OrmRepository;

    #[derive(Default, orm_macro_derive::GetRepository)]
    struct Entity {
        id: i64,
        title: String,
        description: String,
        others: Vec<u32>,
    }

    #[test]
    fn find_method_return_select() {
        assert_eq!(
            "SELECT * FROM entity",
            EntityOrmRepository::builder().find()
        )
    }

    #[test]
    fn find_method_queries_specific_struct_properties() {
        assert_eq!(
            "SELECT title, description FROM entity",
            EntityOrmRepository::builder()
                .select_fields(vec!["title", "description"])
                .find()
        )
    }

    #[test]
    fn create_method_build_sql() {
        assert_eq!(
            "INSERT INTO entity (id, title , description, others) VALUES ($1, $2, $3,
        $4)",
            EntityOrmRepository::builder().create()
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
