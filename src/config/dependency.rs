use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Dependency(HashMap<String, Spec>);

impl Dependency {
    pub fn name(&self) -> &str {
        self.0.keys().next().expect("invalid dependency")
    }

    pub fn binary_name(&self) -> &str {
        if let Some(ref name) = self.spec().binary_name {
            name
        } else {
            self.name()
        }
    }

    pub fn spec(&self) -> &Spec {
        self.0.get(self.name()).expect("invalid dependency")
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Spec {
    pub binary_name: Option<String>,
    pub builtin: Option<bool>,
    pub installer: Option<Installer>,
    pub brew: Option<Brew>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Installer {
    url: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Brew {
    name: String,
}
