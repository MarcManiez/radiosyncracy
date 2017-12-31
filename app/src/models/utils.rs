use diesel::query_builder;
use diesel::pg::Pg;

// This trait allows objects to have convenient .delete instance methods in addition
// to their .delete class method.
pub trait Deletable<T=Self> {
    fn delete(&self) -> Result<Option<T>, String>;
}

pub fn print<T>(query: T) -> T
    where T: query_builder::QueryFragment<Pg>,
{
    println!("{}", query_builder::debug_query::<Pg, _>(&query));
    return query
}
