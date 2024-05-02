extern crate orm_macro_derive;

pub trait OrmRepository {
    fn find(&self) -> String;
    fn select_fields(&mut self, fields: Vec<&str>) -> &mut Self;
    fn create(&mut self) -> String;
    fn update(&self) -> String;
    fn delete(&self) -> String;
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

    #[derive(Default, orm_macro_derive::GetRepository)]
    struct EntityUpdateDto {
        title : String,
        description : String
    }

    #[test]
    fn find_method_return_select() {
        assert_eq!(
            "SELECT id,title,description,others FROM entity",
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
            "INSERT INTO entity (id,title,description,others) VALUES ($1,$2,$3,$4) RETURNING id,title,description,others",
            EntityOrmRepository::builder().create()
        )
    }

    #[test]
    fn delete_method_build_sql() {
        assert_eq!(
            "DELETE FROM entity WHERE id = $1 RETURNING id,title,description,others",
            EntityOrmRepository::builder().delete()
        )
    }

    #[test]
    fn update_method_builds_sql() {
        assert_eq!("UPDATE entity SET title = $1,description = $2 WHERE, id = $3", 
        EntityUpdateDtoOrmRepository::builder().update()
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
