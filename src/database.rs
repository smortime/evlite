#[derive(Clone)]
pub struct Table {
    rows: Vec<Row>,
}

impl Table {
    pub fn new() -> Self {
        Table { rows: Vec::new() }
    }

    pub fn insert_row(&mut self, row: &Row) {
        self.rows.push(row.clone())
    }

    pub fn select_rows(&self) {
        self.rows.iter().for_each(|row| {
            println!("{:?} {:?} {:?}", row.id, row.dog_name, row.breed);
        })
    }

    pub fn row_count(&self) -> usize {
        return self.rows.len();
    }
}

#[derive(Clone)]
pub struct Row {
    pub id: i32,
    pub dog_name: String,
    pub breed: String,
}

impl Row {
    pub fn new(statement_str: &String) -> Self {
        let mut split = statement_str.split_whitespace();
        split.next();
        Row {
            id: split.next().unwrap().parse().unwrap(),
            dog_name: split.next().unwrap().to_string(),
            breed: split.next().unwrap().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::database::*;

    #[test]
    fn row_tests() {
        let row = Row::new(&"insert 1 evie jindo".to_string());
        assert_eq!(row.id, 1);
        assert_eq!(row.dog_name, "evie");
        assert_eq!(row.breed, "jindo");
    }

    #[test]
    fn table_tests() {
        let mut table = Table::new();
        assert_eq!(table.row_count(), 0);
        table.insert_row(&Row::new(&"insert 1 evie jindo".to_string()));
        assert_eq!(table.row_count(), 1);
    }
}
