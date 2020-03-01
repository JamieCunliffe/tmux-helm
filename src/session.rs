#[derive(Debug, Clone)]
pub struct Session {
    pub name: String,
    pub new: bool,
}

impl Session {
    pub fn new(name: String, new: bool) -> Session {
        Session {
            name: name,
            new: new
        }
    }
}

impl std::fmt::Display for Session {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if self.new {
            write!(fmt, "Create new session: {}", self.name)
        } else {
            write!(fmt, "{}", self.name)
        }
    }
}
