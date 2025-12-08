use minenv;

fn main() {
    let env = minenv::load("test.env").expect("expected test.env to be valid");
    println!("foo={}", env.var("foo").expect("expected $foo to be defined"));
}
