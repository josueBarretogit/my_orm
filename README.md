Tired of learning super complex Orms? bored of doing sqlbuilder.select("fields").from("table") (which becomes outdated as your code evolves)? 
sometimes you just want a quick, easy to use sql statement that matches your structs definitions even if it 
changes, well this crate is for you 



# Disclaimer

I am new to rust and at the time I didn´t know how versioning works, so although the version says it's in 1.*.* this crate is still
subject to changes

# Table of contents

 1. [Installation](#Installation)
 2. [Usage](#Usage)
* [Find method](#Find)
* [Find by id](#Find-by-id)
* [Create method](#Create)
* [Update method](#Update)
* [Delete method](#Delete)
 

## Installation

put this in your cargo.toml: 
```rust 
orm_macro = "1.3.0"
orm_macro_derive = { version = "1.3.0", features = ["postgres"] }  
```

The feature flag  "postgres"  uses postgres style bindings, for example: 
```sql
DELETE FROM table WHERE id = $1 # postgres bindings
DELETE FROM table WHERE id = ? # this bindings are used by mysql and sqlite
```
If you want to use mysql bindings then in your cargo.toml
```rust
orm_macro =  "1.3.0"
orm_macro_derive = { version = "1.3.0", features = ["mysql"] } 
```

## Usage
I will be using this structs as examples and sqlx as a database driver
```rust
///bring this to scope
use orm_macro::OrmRepository;
use orm_macro_derive::GetRepository;

//GetRepository will make a new struct with methods that 
//build sql statements using your struct fields
//The new struct will be named {yourStructName}Orm
#[derive(Debug, Default, GetRepository)]
#[table_name(books)]
#[id(id_books)] // Set the id of your table, this will be used in RETURNING and where clauses 
pub struct Books {
    pub id_books: i64,
    pub description: Option<String>,
    pub title: Option<String>,
    pub author_name : String,
}

// works really well with Dto's
#[derive(Debug, Default, GetRepository)]
#[table_name(books)]
#[id(id_books)] // Set the id of your table, this will be used en the RETURNING clauses 
pub struct BooksUpdateDto {
    pub description: Option<String>,
}


#[derive(Debug, Default, GetRepository)]
#[table_name(books)]
#[id(id_books)] // Set the id of your table, this will be used en the RETURNING clauses 
pub struct BooksCreateDto {
    pub title : String,
    pub description: Option<String>,
}

```
## Find 

``` rust 
async fn find_all() -> Result<Vec<Books>, sqlx::Error> {

    
        let builder = BooksOrm::builder();
    

        /// this would generate: SELECT id_books,description,title FROM books 
        ///since it is a string you can use it with any sql driver
        let sql =  builder.find();

        let db_response = sqlx::query_as(sql)
        .fetch_all(&executor)
        .await?;

        Ok(db_response)
  }

 ```


## Find by id

``` rust 
async fn find_by_id() -> Result<Vec<Books>, sqlx::Error> {

    
        let builder = BooksOrm::builder();
    

        ///this generates: SELECT id_books,description,title,author_name FROM books WHERE id_books = $1
        ///This method will be named: find_by_{your_table_id}
        let sql =  builder.find_by_id_books();

        let db_response = sqlx::query_as(sql)
        .fetch_all(&executor)
        .await?;

        Ok(db_response)
  }

 ```



## Create

```rust
async fn create(&self, body : BooksCreateDto) -> Result<Vec<Books>, sqlx::Error> {
        let builder =  BooksCreateDtoOrm::builder();

		let sql = builder.create();

		/// this would generate: INSERT INTO books (title,description) VALUES($1,$2) RETURNING id_books,title,description
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

        let builder =  BooksUpdateDtoOrm::builder();

        /// this would generate: UPDATE books SET description = $1 WHERE id_books = $2 RETURNING id_books, description
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

		/// this would generate: DELETE FROM books WHERE id_books = $1  RETURNING id_books,title,description,author_name
		let sql = builder.delete();
		
        let db_response = sqlx::query_as(sql)
        .bind(id)
        .fetch_one(&*self.db)
        .await?;

        Ok(db_response)
    }
```

Please suggest features or report bugs in the issues tabs
