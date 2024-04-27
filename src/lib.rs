extern crate my_orm_macro_derive;

pub trait Repository {
    fn find(&self) -> String;
}


pub enum FindOptions {
    SelectFields(Vec<String>)
}

#[cfg(test)]
mod tests {
    
    use crate::{FindOptions, Repository};
    

    #[derive(Default, my_orm_macro_derive::GetRepository)]
    struct Entity {
        title: String,
        description: String,
        others: Vec<u32>,
    }

    #[test]
    fn find_method_return_select() {
        let entity = Entity::default();
        assert_eq!("SELECT * FROM entity", entity.find())
    }

    #[test]
    fn find_method_queries_specific_struct_properties() {
        let entity = Entity::default();
        assert_eq!("SELECT title, description FROM entity", entity.select(FindOptions::SelectFields()).find())
    }
}
