use crate::version::Version;

pub enum Repository {
    GitHub(String),
}

pub struct Project {
    pub name: String,
}

pub struct Component {
    pub id: i32,
    pub name: String,
    pub current_version: Version,
    pub repository: Repository,
}
