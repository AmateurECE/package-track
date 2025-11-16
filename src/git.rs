use git2::{Direction, Remote};

use crate::{
    component::{Component, Repository},
    version::Version,
};

pub fn get_git_versions(component: &Component) -> Result<Vec<Version>, git2::Error> {
    let Repository::GitHub(url) = &component.repository;
    let mut gh = Remote::create_detached(url.as_str())?;
    gh.connect(Direction::Fetch)?;
    Ok(gh
        .list()?
        .iter()
        // Remove the refs/tags/ prefix, and optionally remove a 'v' prefix
        .filter_map(|h| {
            h.name()
                .strip_prefix("refs/tags/")
                .map(|v| v.trim_prefix("v"))
        })
        // A suffix ^ followed by an empty brace pair means the object could be a tag.
        // See: https://git-scm.com/docs/gitrevisions
        .filter(|h| !h.ends_with("^{}"))
        .filter_map(|n| n.parse().ok())
        .collect())
}
