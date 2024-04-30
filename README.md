This crate is intended to be used with any database driver since it just builds a sql string with a nicer interface, for example: 

```rust

use orm_macro::OrmRepository;
use orm_macro_derive::GetRepository;


#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Default, GetRepository)]
pub struct Books {
    pub id: i64,
    pub description: Option<String>,
    pub title: Option<String>,
}


    async fn find_all(&self) -> Result<Vec<Books>, sqlx::Error> {


        /// this would generate: SELECT * FROM books 
        let db_response = sqlx::query_as(BooksOrmRepository::builder().find().as_str())
        .fetch_all(&*self.db)
        .await?;


        Ok(db_response)
    }

```
