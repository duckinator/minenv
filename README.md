# min env

use an env file to store key/value pairs for configuration.

environment variables override them.

it's ~50 lines, instead of ~1,500 like some alternatives.

??? why is this a thing i needed to make ???

**features:**
- less than 50 lines
- zero dependencies
- ignores comments
- barely does anything

**non-features:**
- doesn't support multi-line values
- doesn't support escape characters

**current limitations:**
- if you quote a value (`FOO="bar"`), the quotes are included in the value
- values can't contain a `#` (`FOO=bar # asdf` is treated as `FOO=bar` and an ignored comment)

## usage

```rust
use minenv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = minenv::load("test.env")?;
    println!("foo={}", env.var("foo").ok_or("$foo is not defined")?);
}
```
