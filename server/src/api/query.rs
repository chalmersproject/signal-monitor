use super::*;

#[derive(Debug, Clone, Copy, Default, MergedObject)]
pub struct Query(TestQuery);

impl Query {
    pub fn new() -> Self {
        Self::default()
    }
}
