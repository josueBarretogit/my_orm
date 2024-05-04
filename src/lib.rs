extern crate orm_macro_derive;

pub trait OrmRepository {
    fn find(&self) -> String;
    fn select_fields(&mut self, fields: Vec<&str>) -> &mut Self;
    fn create(&mut self) -> String;
    fn update(&self) -> String;
    fn delete(&self) -> String;
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {

    use crate::OrmRepository;

    #[derive(Default, orm_macro_derive::GetRepository)]
    #[table_name("entity")]
    struct Entity {
        id: i64,
        title: String,
        description: String,
        others: Vec<u32>,
        another_property:  bool
    }

    #[derive(Default, orm_macro_derive::GetRepository)]
    #[table_name("entity")]
    struct EntityUpdateDto {
        title: String,
        description: String,
    }

    #[derive(Default, orm_macro_derive::GetRepository)]
    #[table_name("entity")]
    struct EntityFindDto {
        title: String,
        others: String,
    }

    #[derive(Default, orm_macro_derive::GetRepository)]
    #[table_name("entity")]
    struct EntityCreateDto {
        description: String,
    }


    #[test]
    fn find_method_build_select_sql() {
        assert_eq!(
            "SELECT title,others FROM entity",
            EntityFindDtoOrm::builder().find()
        )
    }

    #[test]
    fn find_method_queries_specific_properties() {
        assert_eq!(
            "SELECT title, description FROM entity",
            EntityOrm::builder()
                .select_fields(vec!["title", "description"])
                .find()
        )
    }

    #[test]
    fn create_method_build_insert_sql() {
        assert_eq!(
            "INSERT INTO entity (description) VALUES ($1) RETURNING description",
            EntityCreateDtoOrm::builder().create()
        )
    }

    #[test]
    fn delete_method_build_delete_sql() {
        assert_eq!(
            "DELETE FROM entity WHERE id = $1 RETURNING id,title,description,others,another_property",
            EntityOrm::builder().delete()
        )
    }

    #[test]
    fn update_method_builds_sql() {
        assert_eq!(
            "UPDATE entity SET title = $1,description = $2 WHERE id = $3",
            EntityUpdateDtoOrm::builder().update()
        )
    }
}
