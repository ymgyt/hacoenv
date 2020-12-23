use serde::Deserialize;

use crate::config::dependency::Dependency;
use crate::prelude::*;

const CONFIG_YAML: &[u8] = include_bytes!("../../config/config.yaml");

#[derive(Deserialize, Debug)]
pub struct Config {
    pub common: Common,

    // deserialize guard.
    hacoenv: String,
}

#[derive(Deserialize, Debug)]
pub struct Common {
    pub dependencies: Vec<Dependency>,
}

impl Config {
    pub(crate) fn load() -> Result<Config> {
        let c = serde_yaml::from_slice::<Config>(CONFIG_YAML)?;
        Ok(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_yaml() {
        let config = Config::load().unwrap();

        for dep in &config.common.dependencies {
            match dep.name() {
                "git" => {
                    assert_eq!(Some(true), dep.spec().builtin)
                }
                "docker" => {
                    assert!(dep.spec().installer.is_some())
                }
                "go-task" => {
                    assert!(dep.spec().brew.is_some())
                }
                unexpected => {
                    panic!("unexpected dependency {}, update test case", unexpected);
                }
            }
        }
    }
}
