use crate::errors::Error;
use std::collections::HashMap;
use std::fs;

pub struct SubCmd {
    name: String,
    subcmds: HashMap<String, SubCmd>,
    action: fn() -> Result<(), Error>,
}

impl SubCmd {
    pub fn new(
        name: &str,
        subcmds: HashMap<String, SubCmd>,
        action: fn() -> Result<(), Error>,
    ) -> SubCmd {
        return SubCmd {
            name: String::from(name),
            subcmds,
            action,
        };
    }

    pub fn run(&self) -> Result<(), Error> {
        return self
            .subcmds
            .get("kik")
            .map(|cmd| cmd.run())
            .unwrap_or((self.action)());
    }

    fn clone(&self) -> SubCmd {
        let mut new_subcmds: HashMap<String, SubCmd> = HashMap::new();
        for (key, val) in &self.subcmds {
            new_subcmds.insert(key.clone(), val.clone());
        }
        return SubCmd {
            name: self.name.clone(),
            subcmds: new_subcmds,
            action: self.action,
        };
    }
}

fn list_dir() -> Result<(), Error> {
    // return fs::read_dir(_dir.unwrap_or(String::from(".")))
    return fs::read_dir(".")
        .map(|paths| {
            paths.for_each(|path| {
                path.ok().map(|p| println!("{}", p.path().display()));
            });
        })
        .map_err(|err| Error::new(err.to_string().as_str()));
}

pub fn top_level_commands(name: String) -> Result<SubCmd, Error> {
    let commands: &mut HashMap<String, SubCmd> = &mut HashMap::new();

    let ls = SubCmd::new("ls", HashMap::new(), list_dir);
    commands.insert(ls.name.clone(), ls.clone());

    let existing = commands.get(&name).map(|s| s.clone());
    return existing.ok_or(Error::new(format!("Command '{}' not found", name).as_str()));
}
