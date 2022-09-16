# termpause
Rust Crate that allows to do interruptions in console. Will be implemented to functional terminal customization kit.


![termpause](https://github.com/github/docs/actions/workflows/main.yml/badge.svg)

## Usage
Add this in your `Cargo.toml`:

```toml
[dependencies]
termpause = "0.1.0"
```
Then add this to your `lib.rs` or `main.rs`:

```rust
    use termpause::termpause::pause;

    
    fn main() {
        
        println!("Hello, World");
        termpause::termpause::pause(); //This function will create a pause in console and print "Press any key to continue"
    }
```

