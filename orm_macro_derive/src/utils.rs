/// The input will have the form: atribbute("atribbute to extract")
pub fn extract_string_atribute(input: String) -> String {
    let index_c1 = input.find('(').unwrap();
    let index_c2 = input.find(')').unwrap();

    input[(index_c1 + 1)..index_c2].replace("\"", "")
}

pub trait SqlBuilder {
    fn build_sql(self) -> String;
}

pub struct SelectStatement {
    select_fields: Vec<String>,
    from_table: String,
    where_clause: Option<WhereClause>,
}

pub struct UpdateStatement {
    set_fields: String,
    update_table_name: String,
    where_clause: WhereClause,
    returning_clause: Option<ReturningClause>,
}

impl UpdateStatement {
    pub fn new(update_table_name: &str, where_clause : WhereClause) -> Self {
        Self {
            set_fields: String::new(),
            update_table_name: update_table_name.to_owned(),
            where_clause,
            returning_clause: None,
        }
    }

    pub fn set_where(&mut self, conditions : Vec<String>) -> &mut Self {
        self.where_clause.set_conditions(conditions);
        self
    }

    pub fn set_returning_clause(&mut self, returning_clause: ReturningClause) -> &mut Self {
        self.returning_clause  = Some(returning_clause);
        self
    }

    pub fn set_fields(&mut self, fields : Vec<String>) -> &mut Self { 
        self.set_fields  = fields.iter().enumerate().map(|(index, field)| format!("{} = ${},",  field, index + 1)).collect();
        self.set_fields.pop();
        self
    }

}

impl SqlBuilder for UpdateStatement {
    fn build_sql(self) -> String {

        let returning_clause = if self.returning_clause.is_some() {
            self.returning_clause.unwrap().build_sql()
        } else {
            "".into()
        };

        format!("UPDATE {} SET {} {} {}", self.update_table_name, self.set_fields, self.where_clause.build_sql(), returning_clause)
    }
}

pub struct InsertStatement {
    table_name: String,
    insert_fields: Vec<String>,
    values: Vec<String>,
    returning_clause: Option<ReturningClause>,
}

impl InsertStatement {

    pub fn new(table_name: &str, insert_fields: &Vec<String>, values : Vec<String>) -> Self {
        Self {
            table_name: table_name.to_owned(),
            insert_fields: insert_fields.to_owned(),
            values,
            returning_clause: None,
        }
    }



    pub fn set_returning_clause(&mut self, returning_clause : ReturningClause) -> &mut Self {
        self.returning_clause = Some(returning_clause);
        self
    }
}

impl SqlBuilder for InsertStatement {
    fn build_sql(self) -> String {

        let mut fields_to_insert = String::new(); 
        let mut values_to_insert = String::new(); 

        self.insert_fields.iter().enumerate().for_each(|(index, field)| {

            fields_to_insert.push_str(format!("{},", field).as_str());
            values_to_insert.push_str(format!("${},", index + 1).as_str());

        });

        fields_to_insert.pop();
        values_to_insert.pop();

        
        let returning_clause = if self.returning_clause.is_some() {
            self.returning_clause.unwrap().build_sql()
        } else {
            "".into()
        };


        format!("INSERT INTO {} ({}) VALUES ({}) {}", self.table_name, fields_to_insert, values_to_insert, returning_clause)
    }
}

pub struct DeleteStatement {
    table_name: String,
    where_clause: WhereClause,
    returning_clause: Option<ReturningClause>,
}

pub struct WhereClause {
    conditions: Vec<String>,
}

pub struct ReturningClause {
    fields: Vec<String>,
}
impl ReturningClause {
    pub fn new(fields : &Vec<String>) -> Self {
        Self { fields : fields.to_owned() }
    }
}

impl SqlBuilder for ReturningClause {
    fn build_sql(self) -> String {
        let mut fields : String = self.fields.iter().map(|field| format!("{},", field)).collect();
        fields.pop();
        
        format!("RETURNING id,{}", fields)
    }
}


impl SelectStatement {
    pub fn set_where(&mut self, where_clause: WhereClause) -> &mut Self {
        self.where_clause = Some(where_clause);
        self
    }

    pub fn new(select_fields: &Vec<String>, from_table: &str) -> Self {
        Self {
            select_fields: select_fields.to_owned(),
            from_table: from_table.to_owned(),
            where_clause: None,
        }
    }
}

impl SqlBuilder for SelectStatement {
    fn build_sql(self) -> String {
        let mut fields: String = self
            .select_fields
            .iter()
            .map(|field| format!("{},", field))
            .collect();

        fields.pop();

        let where_clause = match self.where_clause {
            Some(where_clause_builder) => where_clause_builder.build_sql(),
            None => "".into(),
        };

        format!(
            "SELECT {} from {} {}",
            fields, self.from_table, where_clause
        )
    }
}

impl WhereClause {
    pub fn set_conditions(&mut self, conditions: Vec<String>) -> &mut Self {
        self.conditions = conditions;
        self
    }
    pub fn new() -> Self {
        Self {
            conditions: vec!["".to_string()],
        }
    }
}

impl SqlBuilder for WhereClause {
    fn build_sql(self) -> String {
        let conditions : String = self.conditions.iter().map(|cond| cond.to_string()).collect();
        format!("WHERE {}", conditions)
    }
}



impl DeleteStatement {
    pub fn new(table_name: &str, where_clause: WhereClause) -> Self {
        Self {
            table_name: table_name.to_owned(),
            where_clause,
            returning_clause: None,
        }
    }
}

impl SqlBuilder for DeleteStatement {
    fn build_sql(self) -> String {
        format!(
            "DELETE FROM {} {}",
            self.table_name,
            self.where_clause.build_sql()
        )
    }
}


