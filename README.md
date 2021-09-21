# apply_also

Provides the higher order functions `apply` and `also` and variants
`apply_ref`, `also_mut`, for simple object manipulation. 
They can be used to write cleaner initialization of variables 
(see example below). `also` can also be used similarly to unix' 
`tee` command.
Inspired by Kotlin's `apply` and `also` functions.

```rust
use apply_also::{ Apply, Also };

// also:
let map = HashMap::new().also_mut(|it| {
    it.insert("hello", "world");
});
assert_eq!(map.get("hello"), Some(&"world"));

// apply:
let x = 256.apply(|it| it * 2);
assert_eq!(x, 512);

// in a function:
fn test() -> usize {
    5.also(|it| {
        println!("Returning {}!", it);
    })
}
```

## Usage

```toml
[dependencies]
apply_also = { git = "https://github.com/hilbigan/apply_also", branch = "main" }
```

## Note

Is this any good? Is this idiomatic? I don't know, but this repo needs to be 
public for me to test it.

