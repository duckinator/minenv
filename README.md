# min env

use an env file to store key/value pairs for configuration.

environment variables override them.

it's ~50 lines, instead of ~1,500 like some alternatives.

??? why is this a thing i needed to make ???

## usage

```rust
use minenv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = minenv::load("test.env")?;
    println!("foo={}", env.var("foo").ok_or("$foo is not defined")?);
}
```
