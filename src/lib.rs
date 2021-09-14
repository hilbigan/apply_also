//! Provides the higher order functions `apply` and `also` and variants
//! `apply_ref`, `also_mut`, for simple object manipulation. 
//! They can be used to write cleaner initialization of variables 
//! (see example below). `also` can also be used similarly to unix' 
//! `tee` command.
//! Inspired by Kotlin's `apply` and `also` functions.
//!
//! ```
//! # use std::collections::HashMap;
//! use apply_also::{ Apply, Also };
//!
//! // also:
//! let map = HashMap::new().also_mut(|it| {
//!     it.insert("hello", "world");
//! });
//! assert_eq!(map.get("hello"), Some(&"world"));
//!
//! // apply:
//! let x = 256.apply(|it| it * 2);
//! assert_eq!(x, 512);
//! ```

use std::borrow::Borrow;

pub trait Apply<T, R> {
    /// Applies the given function to this value and
    /// returns the result. Consumes the value.
    ///
    /// Example:
    /// ```
    /// # use apply_also::{ Apply, Also };
    /// let x = 256.apply(|it| it * 2);
    /// assert_eq!(x, 512);
    /// ```
    fn apply(self, function: impl FnOnce(T) -> R) -> R;

    /// Applies the given function to a reference of this value 
    /// and returns the result. Consumes the value.
    ///
    /// Example:
    /// ```
    /// # use apply_also::{ Apply, Also };
    /// let x = vec![1, 2, 3].apply_ref(Vec::len);
    /// assert_eq!(x, 3);
    /// ```
    ///
    /// This is just a shorthand for
    /// ```
    /// # use apply_also::{ Apply, Also };
    /// let x = (&vec![1, 2, 3]).apply(Vec::len);
    /// assert_eq!(x, 3);
    /// ```
    fn apply_ref(self, function: impl FnOnce(&T) -> R) -> R;
}

pub trait Also<T> {
    /// Applies the given function to a reference of this value.
    /// Returns the original value.
    ///
    /// Example:
    /// ```
    /// # use apply_also::{ Apply, Also };
    /// let x = 3.also(|_| {
    ///     println!("hi!");
    /// });
    /// assert_eq!(x, 3);
    /// ```
    fn also(self, function: impl FnOnce(&T) -> ()) -> T;

    /// Applies the given function to a mutable reference of this value.
    /// Returns the mutated value.
    ///
    /// Example:
    /// ```
    /// # use apply_also::{ Apply, Also };
    /// let x = Vec::new().also_mut(|it| {
    ///     it.push("hello");
    ///     it.push("world");
    /// });
    /// assert_eq!(x, vec!["hello", "world"]);
    /// ```
    fn also_mut(self, function: impl FnOnce(&mut T) -> ()) -> T;
}

impl<T, R> Apply<T, R> for T {
    fn apply(self, mut function: impl FnOnce(T) -> R) -> R {
        function(self)
    }

    fn apply_ref(self, mut function: impl FnOnce(&T) -> R) -> R {
        function(&self)
    }
}

impl<T> Also<T> for T {
    fn also(self, mut function: impl FnOnce(&T) -> ()) -> T {
        function(&self);
        self
    }

    fn also_mut(mut self, mut function: impl FnOnce(&mut T) -> ()) -> T {
        function(&mut self);
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply() {
        let x = 500.apply(|it| it + 10);
        assert_eq!(x, 510);

        let y = (&vec![1, 2, 3]).apply(Vec::len);
        assert_eq!(y, 3);
    }

    #[test]
    fn apply_ref() {
        let y = vec![1, 2, 3].apply_ref(Vec::len);
        assert_eq!(y, 3);
    }

    #[test]
    fn also() {
        fn test(arg: &Vec<()>) {
            assert!(true);
        }

        let x = Vec::new().also_mut(|it| {
            it.push("hello");
            it.push("world");
        });
        assert_eq!(x, vec!["hello", "world"]);

        let x = Vec::new().also(test);
    }
}
