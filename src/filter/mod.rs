use crate::domain::{Label};

#[derive(Copy, Clone)]
pub enum Operator {
    Equal,
}

pub trait Filter {
    type Item;

    fn matches(&self, op: Operator, item: Self::Item) -> bool;
}

pub struct FilterByLabel {
    label: crate::domain::Label,
}

impl Filter for FilterByLabel {
    type Item = Label;

    fn matches(&self, op: Operator, item: Label) -> bool {
        use Operator::*;
        match op {
            Equal => self.label == item,
        }
    }
}

// This will need to be storable in the DB
pub struct StoredFilter<T, F: Filter<Item=T>> {
    operator: Operator,
    filter: F,
}

impl <T, F: Filter<Item=T>> StoredFilter<T, F> {
    fn matches(&self, item: T) -> bool  {
        self.filter.matches(self.operator, item)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Label;
    use super::*;

    #[test]
    fn matches_the_same_label() {
        let foo: Label = "foo".into();
        let bar: Label = "bar".into();

        let filter = StoredFilter {
            operator: Operator::Equal,
            filter: FilterByLabel {  label: "foo".into() },
        };

        assert!(filter.matches(foo));
        assert!(!filter.matches(bar));

    }
}
