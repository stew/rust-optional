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

// for testing in main
impl<T> Optional<T> for Option<T> {
    fn fold<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(T) -> U
    {
        match self {
            Some(v) => f(v),
            None => default
        }
    }
}

/// `get_or_else` returns the contained value or a _default.
pub fn get_or_else<T, U>(item: T, _default: U) -> U
where
    T: Optional<U>,
{
    // This function serves as an example of how to implem_ent the following methods.
    item.fold(_default, |val| val)
}

/// `is_some` returns `true` if the option is a `Some` value.
pub fn is_some<T, U>(item: T) -> bool
where
    T: Optional<U>,
{
    item.fold(false, |_| true)
}

/// `is_none` returns `true` if the option is a `None` value.
pub fn is_none<T, U>(item: T) -> bool
where
    T: Optional<U>,
{
    item.fold(true, |_| false)
}

/// `map` transforms the `Optional` value with a given function if it is `Some`.
pub fn map<U, V, F>(item: impl Optional<U>, f: F) -> Option<V>
where
    F: FnOnce(U) -> V,
{
    item.fold(None, |v| Some(f(v)))
}

/// `and_then` chains an optional value with a function that returns an optional value.
pub fn and_then<T, U, V>(item: T, f: fn(U) -> Option<V>) -> Option<V>
where
    T: Optional<U>,
{
    item.fold(None, |v| f(v))
}

/// `filter` retains `Some` if the contained value satisfies a predicate.
pub fn filter<T, U, F>(item: T, predicate: F) -> Option<U>
where
    T: Optional<U>,
    F: FnOnce(&U) -> bool,
{
    item.fold(None, |v| if predicate(&v) { Some(v) } else { None })
}

/// `or_else` returns the option if it contains a value, otherwise calls a function that returns an option.
pub fn or_else<T, U, F>(item: T, default: F) -> Option<U>
where
    T: Optional<U>,
    F: FnOnce() -> Option<U>,
{
    // default will be evaluated even when the value is Some which may not be desired
    item.fold(default(), |v| Some(v))
}

/// `xor` returns `Some` if exactly one of self, other is `Some`, otherwise returns `None`.
pub fn xor<T, U>(item: T, other: Option<U>) -> Option<U>
where
    T: Optional<U>,
{
    // (our is_some consumes the value)
    let other_has_value = other.is_some();
    item.fold(other, move |v| if other_has_value { None } else { Some(v) })
}

fn main() {
    println!("run the tests");
}

#[test]
fn test_xor() {
    let usd = None; // Some("$20");
    let gbp = Some("£16.10");
    assert!(is_some(xor(usd, gbp)));
    assert!(is_none(xor(gbp, gbp)));
    assert!(is_none(xor(usd, usd)));
}

#[test]
fn test_filter() {
    let lt = Some(50.0);
    let gt = Some(101.5);
    let is_big_enough = |x: &f32| x > &100.0;
    assert!(is_none(filter(lt, is_big_enough)));
    assert!(is_some(filter(gt, is_big_enough)));
}

#[test]
fn test_map() {
    let string = Some("42");
    let no = None;
    let to_str = |s: &str| s.parse::<usize>().unwrap();
    assert_eq!(Some(42), map(string, to_str));
    assert!(is_none(map(no, to_str)));
}

#[test]
fn test_chaining() {
    let user = Some(5);
    let anonymous = None;
    let get_account = |_u: u32| Some("<account>");
    let always_none = |_u: u32| Option::<&str>::None;
    let account = and_then(user, get_account);
    assert!(is_some(account));
    assert!(is_none(and_then(anonymous, get_account)));
    assert!(is_none(and_then(user, always_none)));
}

#[test]
fn test_or_else() {
    let u = Some(5);
    assert!(is_some(or_else(u, || None)));
    assert!(is_some(or_else(None, || Some(2))));
}
