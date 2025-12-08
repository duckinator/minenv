use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub struct Env {
    data: HashMap<String, String>
}

impl Env {
    pub fn var(&self, key: &str) -> Option<String> {
        std::env::var(key).ok().or_else(|| self.data.get(key).cloned())
    }
}

pub fn load(file: &str) -> Result<Env, Box<dyn std::error::Error>> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut data: HashMap<String, String> = HashMap::new();
    for line in contents.split_terminator("\n") {
        let line =
            if let Some(idx) = line.find('#') {
                line.split_at(idx).0
            } else {
                line
            };

        if line.trim().is_empty() {
            continue;
        }

        let idx = line.find('=').ok_or_else(|| format!("env file had line with no equal sign: {}", line))?;
        let (k, v) = line.split_at(idx);
        data.insert(k.to_string(), v[1..].to_string());
    }

    Ok(Env { data })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        unsafe { std::env::set_var("foo", "bar"); }

        let env = load("test.env").unwrap();

        assert_eq!(env.var("foo").unwrap(), "bar");
        assert_eq!(env.var("first").unwrap(), "line");
        assert_eq!(env.var("second").unwrap(), "line=has=tons=of=equal=signs");
        assert_eq!(env.var("3").unwrap(), "1");
        assert_eq!(env.var("8").unwrap(), "a b c d e f g h i j k l m n o p q r s t u v w x y z");


        unsafe { std::env::set_var("first", "something else"); }
        assert_eq!(env.var("first").unwrap(), "something else");
    }
}
