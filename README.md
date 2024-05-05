Tired of learning super complex Orms? bored of doing sqlbuilder.select("fields").from("table")? 
sometimes you just want a quick, easy to use sql statement that matches your structs definitions even if it changes, well this crate is for you 

Currently this only supports postgres

```rust

use orm_macro::OrmRepository;
use orm_macro_derive::GetRepository;

//GetRepository will make a new struct with functions mapped to the struct
//The new struct will be named struct_nameOrm
#[derive(Debug, Default, GetRepository)]
#[table_name("books")]
pub struct Books {
    pub id: i64,
    pub description: Option<String>,
    pub title: Option<String>,
    pub author_name : String,
}

// works really well with Dto's
#[derive(Debug, Default, GetRepository)]
#[table_name("books")]
pub struct BooksUpdateDto {
    pub description: Option<String>,
}


// works really well with Dto's
#[derive(Debug, Default, GetRepository)]
#[table_name("books")]
pub struct BooksCreateDto {
    pub title : String,
    pub description: Option<String>,
}

pub struct BookRepository {}

impl BookRepository {
    async fn update(&self, description : String) -> Result<Vec<Books>, sqlx::Error> {

        /// this would generate: UPDATE books SET description = $1 WHERE id = $2 RETURNING id, description
        let sql =  BooksUpdateDtoOrm::builder().update();

        let db_response = sqlx::query_as(sql.as_str())
        .bind(description)
        .fetch_all(&*self.db)
        .await?;


        Ok(db_response)
    }


     async fn create(&self, data : ) -> Result<Vec<Books>, sqlx::Error> {

        /// this would generate: INSERT INTO books (title,description) VALUES($1,$2) RETURNING id,title,description
        let sql =  BooksCreateDtoOrm::builder().create();

        let db_response = sqlx::query_as(sql.as_str())
        .bind(description)
        .fetch_all(&*self.db)
        .await?;


        Ok(db_response)
    }


    async fn delete(&self, data : ) -> Result<Vec<Books>, sqlx::Error> {

        /// this would generate: DELETE FROM books WHERE id = $1  RETURNING id,title,description,author_name
        let sql =  BooksOrm::builder().delete();

        let db_response = sqlx::query_as(sql.as_str())
        .bind(description)
        .fetch_all(&*self.db)
        .await?;


        Ok(db_response)
    }

    async fn find_all(&self) -> Result<Vec<Books>, sqlx::Error> {

        /// this would generate: SELECT id,description,title FROM books 
        ///since it is a string you can use it with any sql driver
        let sql =  BooksOrm::builder().find();

        let db_response = sqlx::query_as(sql.as_str())
        .fetch_all(&*self.db)
        .await?;


        Ok(db_response)
    }

}
    



```
