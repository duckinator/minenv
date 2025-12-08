# min env

use an env file to store key/value pairs for configuration.

environment variables override them.

it's ~50 lines, instead of ~1,500 like some alternatives.

??? why is this a thing i needed to make ???

## usage

```rust
use minenv;

fn main() {
    let env = minenv::load("test.env").expect("expected test.env to be valid");
    println!("foo={}", env.var("foo").expect("expected $foo to be defined"));
}
```
