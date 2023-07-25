use std::sync::OnceLock;

pub mod env {
    pub fn required(env_var: &str) -> String {
        std::env::var(env_var)
            .unwrap_or_else(|_| panic!("Missing required env variable: {}", env_var))
    }
}

/* Example
*
struct Env {
    pub var1: String,
    pub var2: String
}

impl Store for Env {
    fn new() -> Self {
        Env {
            var1: env::required("VAR1"),
            var2: env::required("VAR2"),
        }
    }
}

fn main() {
    let var1 = Env::vars().var1;
    let var2 = Env::vars().var2;

    // do whatever you want with those
}

*/
pub trait EnvStore: Sized + Clone {
    const STORE: OnceLock<Self> = OnceLock::new();
    fn new() -> Self;
    fn vars() -> Self {
        match Self::STORE.get() {
            Some(val) => val.clone(),
            None => {
                let env = Self::new();
                _ = Self::STORE.set(env.clone());
                env
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn env_store_ok() {
        #[derive(Clone)]
        struct MyStruct {
            pub var1: String,
            pub var2: String,
        }

        impl EnvStore for MyStruct {
            fn new() -> Self {
                MyStruct {
                    var1: "var1".to_string(),
                    var2: "var2".to_string(),
                }
            }
        }

        assert_eq!(&MyStruct::vars().var1, "var1");
        assert_eq!(&MyStruct::vars().var2, "var2");
    }
}
