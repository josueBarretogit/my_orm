Tired of learning super complex Orms? bored of doing sqlbuilder.select("fields").from("table") (which becomes outdated as your code evolves)? 
sometimes you just want a quick, easy to use sql statement that matches your structs definitions even if it 
changes, well this crate is for you 

# Table of contents

 1. [Installation](#Installation)
 2. [Usage](#Usage)
* [Find method](#Find)
* [Create method](#Create)
* [Update method](#Update)
* [Delete method](#Delete)
 

## Installation

put this in your cargo.toml: 
```rust 
orm_macro = version = "1.2.0"
orm_macro_derive =  version = "1.2.0" 
```

The feature flag enabled by default is "postgres" which uses postgres style bindings, for example: 
```sql
DELETE FROM table WHERE id = $1 # postgres bindings
DELETE FROM table WHERE id = ? # this bindings are used by mysql and sqlite
```
If you want to use mysql bindings then in your cargo.toml
```rust
orm_macro = { version = "1.2.0", features = ["mysql"] }
orm_macro_derive = { version = "1.2.0", features = ["mysql"] } 
```

## Usage
I will be using this structs as examples and sqlx as a database driver
```rust
///bring this to scope
use orm_macro::OrmRepository;
use orm_macro_derive::GetRepository;

//GetRepository will make a new struct with methods that 
//build sql statements using your struct fields
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


#[derive(Debug, Default, GetRepository)]
#[table_name("books")]
pub struct BooksCreateDto {
    pub title : String,
    pub description: Option<String>,
}

```
## Find 

``` rust 
async fn find_all() -> Result<Vec<Books>, sqlx::Error> {

        /// this would generate: SELECT id,description,title FROM books 
        ///since it is a string you can use it with any sql driver
        let sql =  BooksOrm::builder().find();
        let db_response = sqlx::query_as(sql.as_str())
        .fetch_all(&executor)
        .await?;

        Ok(db_response)
  }

 ```

## Create

```rust
async fn create(&self, body : BooksCreateDto) -> Result<Vec<Books>, sqlx::Error> {
        let builder =  BooksCreateDtoOrm::builder();

		/// this would generate: INSERT INTO books (title,description) VALUES($1,$2) RETURNING id,title,description
		let sql = builder.create();

        let db_response = sqlx::query_as(sql)
        .bind(body.title)
        .bind(body.description)
        .fetch_one(&executor)
        .await?;

        Ok(db_response)
 }
```

## Update
```rust
    async fn update(body : BooksUpdateDto) -> Result<Vec<Books>, sqlx::Error> {

        /// this would generate: UPDATE books SET description = $1 WHERE id = $2 RETURNING id, description
        let builder =  BooksUpdateDtoOrm::builder();
		let sql = builder.update();

        let db_response = sqlx::query_as(sql))
        .bind(body.description)
        .fetch_all(&*self.db)
        .await?;


        Ok(db_response)
    }
```
## Delete

```rust
    async fn delete(id: i64) -> Result<Vec<Books>, sqlx::Error> {
        let builder =  BooksOrm::builder();
		/// this would generate: DELETE FROM books WHERE id = $1  RETURNING id,title,description,author_name
		let sql = builder.delete();
		
        let db_response = sqlx::query_as(sql)
        .bind(id)
        .fetch_one(&*self.db)
        .await?;

        Ok(db_response)
    }
```

Please suggest features or report bugs in the issues tabs
