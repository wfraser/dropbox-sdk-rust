/// Checks that a value is valid for some particular use.
pub trait Validator<T: ?Sized> {
    /// The name of the type this validator is for.
    const NAME: &'static str;

    /// Check if the value is valid.
    fn is_valid(value: &T) -> bool;

    /// Assert that the value is valid. Panics if it is not.
    fn assert_valid(value: &T) {
        assert!(Self::is_valid(value), "invalid value for {}", Self::NAME);
    }
}

/// Specialized validator for strings. Supports minimum and maximum lengths, and regexes.
pub trait StringValidator {
    /// The name of the type this validator is for.
    /// (This shadows the one in Validator, but will be used instead of it in the implementation.)
    const NAME: &'static str;

    /// A regular expression that the value must match.
    const REGEX: Option<&'static str> = None;

    /// The minimum length (inclusive) for values.
    const MIN_LENGTH: Option<usize> = None;

    /// The maximum length (inclusive) for values.
    const MAX_LENGTH: Option<usize> = None;

    fn is_valid(value: &str) -> bool {
        if let Some(min) = Self::MIN_LENGTH {
            if value.len() < min {
                error!("value {:?} doesn't meet minimum length for {}", value, <Self as StringValidator>::NAME);
                return false;
            }
        }
        if let Some(max) = Self::MAX_LENGTH {
            if value.len() > max {
                error!("value {:?} exceeds maximum length for {}", value, <Self as StringValidator>::NAME);
                return false;
            }
        }
        if let Some(pattern) = Self::REGEX {
            if !::regex::Regex::new(pattern).unwrap().is_match(value) {
                error!("value {:?} doesn't match pattern for {}", value, <Self as StringValidator>::NAME);
                return false;
            }
        }
        true
    }
}

impl<V> Validator<String> for V where V: StringValidator {
    const NAME: &'static str = "";

    fn is_valid(value: &String) -> bool {
        <Self as StringValidator>::is_valid(value.as_str())
    }

    fn assert_valid(value: &String) {
        assert!(Self::is_valid(value), "{:?} is not a valid value for {}", value, <Self as StringValidator>::NAME);
    }
}

impl<V> Validator<str> for V where V: StringValidator {
    const NAME: &'static str = "";

    fn is_valid(value: &str) -> bool {
        <Self as StringValidator>::is_valid(value)
    }

    fn assert_valid(value: &str) {
        assert!(Self::is_valid(value), "{:?} is not a valid value for {}", value, <Self as StringValidator>::NAME);
    }
}

/// A type with some validity constraints associated with it.
pub struct Constrained<T, V: Validator<T>>(T, ::std::marker::PhantomData<V>);

impl<T, V: Validator<T>> Constrained<T, V> {
    /// Make an instance of the constrained type, without checking its validity. Use when you KNOW
    /// the value is valid or want to avoid the overhead of checking.
    pub fn unchecked(value: T) -> Self {
        Constrained(value, ::std::marker::PhantomData)
    }
}

/// Make the Constrained wrapper behave like the underlying type in most contexts.
impl<T, V: Validator<T>> ::std::ops::Deref for Constrained<T, V> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

/// Forward to Debug implementation.
impl<T, V: Validator<T>> ::std::fmt::Debug for Constrained<T, V> where T: ::std::fmt::Debug {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Forward to Display implementation.
impl<T, V: Validator<T>> ::std::fmt::Display for Constrained<T, V> where T: ::std::fmt::Display {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Forward to Serde Deserialize implementation.
impl<'de, T, V: Validator<T>> ::serde::Deserialize<'de> for Constrained<T, V> where T: ::serde::Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: ::serde::Deserializer<'de>
    {
        T::deserialize(deserializer).map(Constrained::<T, V>::unchecked)
    }
}

/// Forward to Serde Serialize implementation.
impl<T, V: Validator<T>> ::serde::Serialize for Constrained<T, V> where T: ::serde::Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ::serde::Serializer
    {
        self.0.serialize(serializer)
    }
}

/// Make a value into a constrained wrapper.
pub trait IntoConstrained<V: Validator<Self>>: ::std::marker::Sized {
    /// Make a value into a constrained wrapper, asserting that it is valid. Panics if it is not.
    fn validate(self) -> Constrained<Self, V> {
        V::assert_valid(&self);
        self.unchecked()
    }

    /// Make a value into a constrained wrapper, without checking for validity. Use when you KNOW
    /// the value is valid or want to avoid the overhead of checking.
    fn unchecked(self) -> Constrained<Self, V> {
        Constrained::<Self, V>::unchecked(self)
    }

    fn try_validate(self) -> Result<Constrained<Self, V>, Self> {
        if V::is_valid(&self) {
            Ok(self.unchecked())
        } else {
            Err(self)
        }
    }
}

impl<T, V: Validator<T>> IntoConstrained<V> for T {}

// FIXME(wfraser): hacks for &str
// probably the validator constraints should all be V: Validator<AsRef<T>> instead.

impl<'a, V> Validator<&'a str> for V where V: StringValidator {
    const NAME: &'static str = "";

    fn is_valid(value: &&'a str) -> bool {
        <Self as Validator<str>>::is_valid(*value)
    }

    fn assert_valid(value: &&'a str) {
        <Self as Validator<str>>::assert_valid(*value)
    }
}

pub trait RefConstrained<T, V: Validator<T>> {
    fn to_owned(self) -> Constrained<T, V>;
}

impl<'a, V: Validator<String> + Validator<&'a str>> RefConstrained<String, V> for Constrained<&'a str, V> {
    fn to_owned(self) -> Constrained<String, V> {
        self.0.to_owned().unchecked()
    }
}
