trait Aggregate {
    type Item;

    fn version(&self) -> u64;
    fn apply(&self, evt: &Self::Item) -> Self where Self:Sized;
}