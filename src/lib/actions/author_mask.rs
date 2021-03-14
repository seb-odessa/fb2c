use diesel::sql_types::Text;
use serde::Serialize;

pub trait NvcMethods {
    fn get_length_by_name(&self, name: &str) -> usize;
    fn get_where_like_clause(&self) -> String;
    fn get_where_explicit_clause(&self) -> String;
}

#[derive(QueryableByName, Debug, Clone, Serialize)]
pub struct AuthorMask {
    #[sql_type = "Text"] pub first_name: String,
    #[sql_type = "Text"] pub middle_name: String,
    #[sql_type = "Text"] pub last_name: String,
}
impl AuthorMask {
    fn decode(mask: String) -> String {
        if &mask == "-" {
            String::new()
        } else {
            mask
        }
    }

    pub fn encode(val: String) -> String {
        if val.is_empty() {
            String::from("-")
        } else {
            val
        }
    }

    pub fn get_encoded_by_name(&self, name: &str) -> String {
        match name {
            "first_name" => Self::encode(self.first_name.clone()),
            "middle_name" => Self::encode(self.middle_name.clone()),
            "last_name" => Self::encode(self.last_name.clone()),
            _ => String::new()
        }
    }

    pub fn get_encoded_by_name_previous(&self, name: &str) -> String {
        let mut name = self.get_encoded_by_name(name);
        if "-" != &name{
            name.pop();
        }
        return name;
    }

    pub fn get_uri(&self) -> String {
        format!("{}/{}/{}",
            Self::encode(self.first_name.clone()),
            Self::encode(self.middle_name.clone()),
            Self::encode(self.last_name.clone()))
    }

    pub fn new(first_name: String, middle_name: String, last_name: String) -> Self {
        Self {
            first_name: Self::decode(first_name),
            middle_name: Self::decode(middle_name),
            last_name: Self::decode(last_name),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.first_name.is_empty() &&
        self.middle_name.is_empty() &&
        self.last_name.is_empty()
    }

    pub fn get_full_name(&self) -> String {
        format!("{} {} {}", self.last_name, self.first_name, self.middle_name)
    }
}

impl NvcMethods for AuthorMask {
    fn get_length_by_name(&self, name: &str) -> usize {
        match name {
            "first_name" => self.first_name.chars().count(),
            "middle_name" => self.middle_name.chars().count(),
            "last_name" => self.last_name.chars().count(),
            _ => 0
        }
    }
    fn get_where_like_clause(&self) -> String {
        let mut clauses = Vec::new();
        if !self.first_name.is_empty()
        {
            clauses.push(format!("first_name LIKE '{}%'", self.first_name));
        }
        if !self.middle_name.is_empty() {
            if !clauses.is_empty() {
                clauses.push("AND".to_owned());
            }
            clauses.push(format!("middle_name LIKE '{}%'", self.middle_name));
        }
        if !self.last_name.is_empty() {
            if !clauses.is_empty() {
                clauses.push("AND".to_owned());
            }
            clauses.push(format!("last_name LIKE '{}%'", self.last_name));
        }

        return if clauses.is_empty() {
           String::new()
        } else {
            "WHERE ".to_owned() + &clauses.join(" ")
        }
    }

    fn get_where_explicit_clause(&self) -> String {
        let mut clauses = Vec::new();
        if !self.first_name.is_empty()
        {
            clauses.push(format!("first_name = '{}'", self.first_name));
        }
        if !self.middle_name.is_empty() {
            if !clauses.is_empty() {
                clauses.push("AND".to_owned());
            }
            clauses.push(format!("middle_name = '{}'", self.middle_name));
        }
        if !self.last_name.is_empty() {
            if !clauses.is_empty() {
                clauses.push("AND".to_owned());
            }
            clauses.push(format!("last_name = '{}'", self.last_name));
        }

        return if clauses.is_empty() {
           String::new()
        } else {
            "WHERE ".to_owned() + &clauses.join(" ")
        }
    }
}