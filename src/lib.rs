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
        let line = line.splitn(2, '#').next().unwrap_or("").trim();
        if line.is_empty() {
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
        let env = load("test.env").unwrap();

        assert_eq!(env.var("foo").unwrap(), "bar");
        assert_eq!(env.var("beep").unwrap(), "boop with=equal=signs");

        unsafe { std::env::set_var("foo", "something else"); }
        assert_eq!(env.var("foo").unwrap(), "something else");
    }
}
