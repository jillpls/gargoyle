use std::fs;
use rand::seq::SliceRandom;

pub struct Name {
    pub value : String
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", "\n", self.value)
    }
}

impl Clone for Name {
    fn clone(&self) -> Self {
        Name {
            value : self.value.clone()
        }
    }
}

pub enum NameType {
    Female,
    Male,
    NonBinary,
    Surname
}

pub struct Names {
    pub f_names : Vec<Name>,
    pub m_names : Vec<Name>,
    pub n_names : Vec<Name>,
    pub surnames : Vec<Name>
}

impl Names {
    pub fn new() -> Names {
        Names {
            f_names : Vec::new(),
            m_names : Vec::new(),
            n_names : Vec::new(),
            surnames : Vec::new(),
        }
    }

    pub fn from_dir(dir_path : &str) -> Names {
        let f_names = Self::from_file(dir_path, "f_names.txt");
        let m_names = Self::from_file(dir_path, "m_names.txt");
        let n_names = Self::from_file(dir_path, "n_names.txt");
        let surnames = Self::from_file(dir_path, "surnames.txt");
        Names {
            f_names,
            m_names,
            n_names,
            surnames
        }
    }

    pub fn get_rand_name(&self, name_type : NameType) -> Name {
        let names = match name_type {
            NameType::Female => &self.f_names,
            NameType::Male => &self.m_names,
            NameType::NonBinary => &self.n_names,
            NameType::Surname => &self.surnames
        };
        let n = names.choose(&mut rand::thread_rng()).unwrap();
        n.clone()
    }

    fn from_file(dir_path: &str, file_name: &str) -> Vec<Name> {
        let mut names = Vec::new();
        let raw_input = fs::read_to_string(dir_path.to_string() + "/" + file_name).expect("Something went wrong :(");
        for s in raw_input.split("\n") {
            if s.trim().len() == 0 { continue; }
            if s.trim().chars().next().unwrap() == '#' { continue; }
            names.push(Name { value: s.trim().to_string() });
        }
        names
    }
}

