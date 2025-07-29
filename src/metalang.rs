use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    StrVec(Vec<String>),
}

#[derive(Debug, Default)]
pub struct Context {
    pub variables: HashMap<String, Value>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            variables: HashMap::new(),
        }
    }

    pub fn from_lang(raw: String) -> Context {
        let mut ctx = Context::new();
        let mut raw = raw;
        while let Some(start) = raw.find("/*") {
            if let Some(end) = raw[start + 2..].find("*/") {
                let end = start + 2 + end + 2;
                raw.replace_range(start..end, "");
            } else {
                break;
            }
        }

        let lines: Vec<String> = raw.lines().map(|line| line.trim().to_string()).collect();
        let mut mount_started = false;
        let mut mount_varname: String = String::from("");
        let mut mount_dirnames: Vec<&str> = Vec::new();

        for i in &lines {
            if mount_started {
                if let Some(pos) = i.find("}") {
                    if pos != 0 {
                        mount_dirnames.push(&i[0..pos]);
                    }

                    let dirnames: Vec<String> =
                        mount_dirnames.iter().map(|&s| s.to_string()).collect();

                    let new_value = match ctx.variables.get(&mount_varname) {
                        Some(Value::Str(s)) => {
                            let mut new_vec = vec![s.clone()];
                            new_vec.extend(dirnames);
                            Value::StrVec(new_vec)
                        }
                        Some(Value::StrVec(vec)) => {
                            let mut new_vec = vec.clone();
                            new_vec.extend(dirnames);
                            Value::StrVec(new_vec)
                        }
                        None => Value::StrVec(dirnames),
                    };

                    ctx.variables.insert(mount_varname.clone(), new_value);

                    mount_started = false;
                    mount_varname.clear();
                    mount_dirnames.clear();
                } else {
                    mount_dirnames.push(&i);
                }
            }

            if i.starts_with("Use") {
                if let Some(rest) = i.strip_prefix("Use ") {
                    if let Some((var, val)) = rest.split_once('=') {
                        let varname = var.trim().to_string();
                        let value = val.trim().to_string();
                        ctx.variables.insert(varname, Value::Str(value));
                    }
                }
            } else if i.starts_with("Begin") {
                if let Some(rest) = i.strip_prefix("Begin ") {
                    if let Some((block_type, var)) = rest.split_once("on") {
                        let btype = block_type.trim().to_string();
                        let varname = var.strip_suffix("{").unwrap().trim().to_string();

                        if btype == "Mount" {
                            mount_started = true;
                            mount_varname = varname.clone();
                        }
                    }
                }
            }
        }

        ctx
    }

    pub fn get_var(&self, name: &str) -> &Value {
        match self.variables.get(name) {
            Some(val) => val,
            None => panic!("Variable does not exist"),
        }
    }
}
