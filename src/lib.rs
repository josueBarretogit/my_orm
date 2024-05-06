extern crate orm_macro_derive;

///This trait contains the methods that generate sql
pub trait OrmRepository {
    /// generate: SELECT {struct_fields} from {table_name}
    fn find(&self) -> String;
    ///Used to specify which fields to select
    #[deprecated(since="1.2.0", note="Removing this unnecesary method will make find() return &str instead of String, in the future there will be better find methods")]
    fn select_fields(&mut self, fields: Vec<&str>) -> &mut Self;
    /// generate: INSERT INTO {table_name} ({struct_fields}) VALUES({$1,$2...}) RETURNING
    /// {struct_fields}
    fn create(&mut self) -> &str;
    /// generate: UPDATE {table_name} SET struct_field1 = $1 , WHERE id = $2 RETURNING {struct_fields}
    /// {struct_fields}
    fn update(&self) -> &str;
    ///generates: DELETE FROM {table_name} WHERE id = $1 RETURNING {struct_fields}
    fn delete(&self) -> &str;
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
        another_property: bool,
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
            "SELECT title,others FROM entity ",
            EntityFindDtoOrm::builder().find()
        )
    }

    #[test]
    fn find_method_build_select_sql_with_main() {
        assert_eq!(
            "SELECT id,title,description,others,another_property FROM entity ",
            EntityOrm::builder().find()
        )
    }

    #[cfg(feature = "postgres")]
    #[test]
    fn create_method_build_insert_sql_with_main_entity() {
        assert_eq!(
    "INSERT INTO entity (title,description,others,another_property) VALUES ($1,$2,$3,$4) RETURNING id,title,description,others,another_property",
    EntityOrm::builder().create()
    )
    }

    #[cfg(not(feature = "postgres"))]
    #[test]
    fn create_method_build_insert_sql_with_main_entity_mysql_bindings() {
        assert_eq!(
    "INSERT INTO entity (title,description,others,another_property) VALUES (?,?,?,?) RETURNING id,title,description,others,another_property",
    EntityOrm::builder().create()
    )
    }


    #[cfg(feature = "postgres")]
    #[test]
    fn create_method_build_insert_sql() {
        assert_eq!(
            "INSERT INTO entity (description) VALUES ($1) RETURNING id,description",
            EntityCreateDtoOrm::builder().create()
        )
    }


    #[cfg(not(feature = "postgres"))]
    #[test]
    fn create_method_build_insert_mysql_bindings() {
        assert_eq!(
            "INSERT INTO entity (description) VALUES (?) RETURNING id,description",
            EntityCreateDtoOrm::builder().create()
        )
    }


    #[cfg(feature = "postgres")]
    #[test]
    fn delete_method_build_delete_sql() {
        assert_eq!(
    "DELETE FROM entity WHERE id = $1 RETURNING id,title,description,others,another_property",
    EntityOrm::builder().delete()
    )
    }

    #[cfg(not(feature = "postgres"))]
    #[test]
    fn delete_method_build_delete_sql_mysql_bindings() {
        assert_eq!(
    "DELETE FROM entity WHERE id = ? RETURNING id,title,description,others,another_property",
    EntityOrm::builder().delete()
    )
    }


    #[cfg(feature = "postgres")]
    #[test]
    fn update_method_builds_sql() {
        assert_eq!(
    "UPDATE entity SET title = $1,description = $2 WHERE id = $3 RETURNING id,title,description",
    EntityUpdateDtoOrm::builder().update()
    )

    }

    #[cfg(not(feature = "postgres"))]
    #[test]
    fn update_method_builds_sql_mysql_bindings() {
        assert_eq!(
    "UPDATE entity SET title = ?,description = ? WHERE id = ? RETURNING id,title,description",
    EntityUpdateDtoOrm::builder().update()
    )
    }




    #[cfg(feature = "postgres")]
    #[test]
    fn update_method_builds_sql_with_main() {
        assert_eq!(
    "UPDATE entity SET title = $1,description = $2,others = $3,another_property = $4 WHERE id = $5 RETURNING id,title,description,others,another_property",
    EntityOrm::builder().update()
    )
    }

    #[cfg(not(feature = "postgres"))]
    #[test]
    fn test_update_query_with_mysql_binding() {
        assert_eq!(
    "UPDATE entity SET title = ?,description = ?,others = ?,another_property = ? WHERE id = ? RETURNING id,title,description,others,another_property",
        EntityOrm::builder().update()
    )
    }

}
