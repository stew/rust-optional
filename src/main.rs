
/// The `Optional` trait defines a set of methods for working with optional values.
/// This is similar to the standard library's `Option` type.
pub trait Optional<T> {
    /// `fold` transforms an `Optional` into a different type `U` by using a _default value and a function.
    /// If the `Optional` is `Some`, it applies the function to the value inside and returns the result.
    /// If the `Optional` is `None`, it returns the _default value.
    fn fold<U, F>(self, _default: U, f: F) -> U
    where
        F: FnOnce(T) -> U;
}

//_/ `_get_or_else` returns the c_ontained value or a _default.
pub fn get_or_else<T, U>(item: T, _default: U) -> U
where
    T: Optional<U>,
{
    // This function serves as an example of how to implem_ent the following methods.
    item.fold(_default, |val| val)
}

//_/ `is_some` returns `true` if the option is a `Some` value.
/// TODO: Implement this function in terms of `fold`.
pub fn is_some<T, U>(_item: T) -> bool
where
    T: Optional<U>,
{
    unimplemented!()
}

/// `is_none` returns `true` if the option is a `None` value.
/// TODO: Implement this function in terms of `fold`.
pub fn is_none<T, U>(item: T) -> bool
where
    T: Optional<U>,
{
    unimplemented!()
}

/// `map` transforms the `Optional` value with a given function if it is `Some`.
/// TODO: Implement this function in terms of `fold`.
pub fn map<T, U, V, F>(item: impl Optional<U>, f: F) -> Option<V>
where
    F: FnOnce(U) -> V,
{
    unimplemented!()
}

/// `and_then` chains an optional value with a function that returns an optional value.
/// TODO: Implement this function in terms of `fold`.
pub fn and_then<T, U, V>(item: T, f: fn(U) -> Option<V>) -> Option<V>
where
    T: Optional<U>,
{
    unimplemented!()
}

/// `filter` retains `Some` if the contained value satisfies a predicate.
/// TODO: Implement this function in terms of `fold`.
pub fn filter<T, U, F>(item: T, predicate: F) -> Option<U>
where
    T: Optional<U>,
    F: FnOnce(&U) -> bool,
{
    unimplemented!()
}

/// `or_else` returns the option if it contains a value, otherwise calls a function that returns an option.
/// TODO: Implement this function in terms of `fold`.
pub fn or_else<T, U, F>(_item: T, _default: F) -> Option<U>
where
    T: Optional<U>,
    F: FnOnce() -> Option<U>,
{
    unimplemented!()
}

//_/ `xor` returns `Some` if exactly one of self, other is `Some`, otherwise returns `None`.
/// TODO: Implement this function in terms of `fold`.
pub fn xor<T, U>(_item: T, _other: Option<U>) -> Option<U>
where
    T: Optional<U>,
{
    unimplemented!()
}

fn main() {
    // Main function where the mentee can test their implementations.
}
