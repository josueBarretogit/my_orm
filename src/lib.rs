extern crate orm_macro_derive;

///This trait contains the methods that generate sql
pub trait OrmRepository {
    /// generate: SELECT {struct_fields} from {table_name}
    fn find(&self) -> &str;
    /// generate: INSERT INTO {table_name} ({struct_fields}) VALUES({$1,$2...}) RETURNING
    /// {struct_fields}
    fn create(&self) -> &str;
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
    #[table_name(entity)]
    #[id(id)]
    struct Entity {
        id: i64,
        title: String,
        description: String,
        others: Vec<u32>,
        another_property: bool,
    }

    #[derive(Default, orm_macro_derive::GetRepository)]
    #[table_name(entity)]
    #[id(id)]
    struct EntityUpdateDto {
        title: String,
        description: String,
    }

    #[derive(Default, orm_macro_derive::GetRepository)]
    #[table_name(entity)]
    #[id(id)]
    struct EntityFindDto {
        title: String,
        others: String,
    }

    #[derive(Default, orm_macro_derive::GetRepository)]
    #[table_name(entity)]
    #[id(id)]
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

    #[cfg(feature = "postgres")]
    #[test]
    fn find_by_id_method_builds_sql_postgres() {
        assert_eq!(
            "SELECT title,others FROM entity WHERE id = $1",
            EntityFindDtoOrm::builder().find_by_id()
        )
    }

    #[cfg(not(feature = "postgres"))]
    #[test]
    fn find_by_id_method_builds_sql() {
        assert_eq!(
            "SELECT title,others FROM entity WHERE id = ?",
            EntityFindDtoOrm::builder().find_by_id()
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
    "UPDATE entity SET id = $1,title = $2,description = $3,others = $4,another_property = $5 WHERE id = $6 RETURNING id,title,description,others,another_property",
    EntityOrm::builder().update()
    )
    }

    #[cfg(not(feature = "postgres"))]
    #[test]
    fn test_update_query_with_mysql_binding() {
        assert_eq!(
    "UPDATE entity SET id = ?,title = ?,description = ?,others = ?,another_property = ? WHERE id = ? RETURNING id,title,description,others,another_property",
        EntityOrm::builder().update()
    )
    }
}
