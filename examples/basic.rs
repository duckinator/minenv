use minenv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = minenv::load("test.env")?;
    println!("foo={}", env.var("foo").ok_or("expected $foo to be defined")?);
    Ok(())
}
