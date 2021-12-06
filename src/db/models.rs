/// This `struct` represents the model for items in the database.
/// It can be created by the `new` method.
#[derive(sqlx::FromRow, Debug, PartialEq, Default)]
pub struct Item {
    /// Unique identifier in the database
    pub id: i64,
    /// Text
    pub text: String,
}

impl Item {
    /// Creates new `Item`
    #[allow(unused)]
    pub fn new<S>(text: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            id: 0,
            text: text.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::db::models::*;

    #[test]
    fn new_item() {
        let actual = Item::new("hello");
        let expected = Item {
            id: 0,
            text: "hello".to_string(),
        };

        assert_eq!(actual, expected);
    }
}
