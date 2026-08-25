use std::{collections::HashMap, error::Error, fs, ops::Deref};

pub struct Env(HashMap<String, String>);

impl Env {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut map = HashMap::new();

        for (k, v) in std::env::vars() {
            map.insert(k, v);
        }

        let rows = fs::read(".env")?;

        let mut key = String::new();
        let mut value = String::new();
        let mut is_value: bool = false;

        for r in rows {
            let r = r as char;

            match r {
                '=' => is_value = true,
                '\n' => {
                    is_value = false;
                    map.insert(key.to_owned(), value.to_owned());

                    value.clear();
                    key.clear();
                }
                _ => {
                    if is_value {
                        value.push(r);
                    } else {
                        key.push(r);
                    }
                }
            }
        }

        Ok(Self(map))
    }
}

impl Deref for Env {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::tools::write_dot_env;
//
//     #[test]
//     fn le() {
//         write_dot_env();
//     }
// }
