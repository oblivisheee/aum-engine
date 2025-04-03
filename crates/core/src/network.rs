use std::{
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};
pub trait Network:
    Copy + Clone + Debug + Display + FromStr + Send + Sync + 'static + Eq + Ord + Sized + Hash
{
    const NAME: &'static str;
}
