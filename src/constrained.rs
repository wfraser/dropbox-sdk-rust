pub trait Validator<T> {
    fn is_valid(value: &T) -> bool;
    fn assert_valid(value: &T) {
        assert!(Self::is_valid(value), "validation failed");
    }
}

pub struct Constrained<T, V: Validator<T>>(T, ::std::marker::PhantomData<V>);

impl<T, V: Validator<T>> Constrained<T, V> {
    pub fn unchecked(value: T) -> Self {
        Constrained(value, ::std::marker::PhantomData)
    }
}

impl<T, V: Validator<T>> ::std::ops::Deref for Constrained<T, V> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

impl<T, V: Validator<T>> ::std::fmt::Debug for Constrained<T, V> where T: ::std::fmt::Debug {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'de, T, V: Validator<T>> ::serde::Deserialize<'de> for Constrained<T, V> where T: ::serde::Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: ::serde::Deserializer<'de>
    {
        T::deserialize(deserializer).map(Constrained::<T, V>::unchecked)
    }
}

impl<T, V: Validator<T>> ::serde::Serialize for Constrained<T, V> where T: ::serde::Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ::serde::Serializer
    {
        self.0.serialize(serializer)
    }
}

pub trait IntoConstrained<V: Validator<Self>>: ::std::marker::Sized {
    fn validate(self) -> Constrained<Self, V> {
        //assert!(V::is_valid(&self), "validation failed");
        V::assert_valid(&self);
        self.unchecked()
    }
    fn unchecked(self) -> Constrained<Self, V> {
        Constrained::<Self, V>::unchecked(self)
    }
}

impl<T, V: Validator<T>> IntoConstrained<V> for T {}
