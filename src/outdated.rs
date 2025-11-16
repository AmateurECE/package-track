use core::fmt;
use std::collections::HashSet;

use crate::{
    component::{Component, Project},
    db::Database,
    version::Version,
};

pub struct OutdatedComponent {
    pub name: String,
    pub latest_version: Version,
    pub current_version: Version,
    pub projects: Vec<Project>,
}

pub trait FetchVersions {
    type Error;
    fn fetch_versions(&mut self, component: &Component) -> Result<Vec<Version>, Self::Error>;
}

impl<R, E> FetchVersions for R
where
    R: FnMut(&Component) -> Result<Vec<Version>, E>,
{
    type Error = E;
    fn fetch_versions(&mut self, component: &Component) -> Result<Vec<Version>, Self::Error> {
        self(component)
    }
}

#[derive(thiserror::Error)]
pub enum Error<E> {
    #[error("repository: {0}")]
    Repository(E),
    #[error("database: {0}")]
    Db(#[from] sqlx::Error),
}

pub async fn find_outdated<R>(
    db: Database,
    mut repository: R,
) -> Result<Vec<OutdatedComponent>, Error<<R as FetchVersions>::Error>>
where
    R: FetchVersions,
    <R as FetchVersions>::Error: fmt::Display,
{
    let mut outdated = Vec::new();
    for component in db.components().await? {
        match is_outdated(&db, &mut repository, component).await {
            Ok(Some(component)) => outdated.push(component),
            Ok(None) => {}
            Err(e) => eprintln!("{}", e),
        }
    }
    Ok(outdated)
}

async fn is_outdated<R>(
    db: &Database,
    repository: &mut R,
    component: Component,
) -> Result<Option<OutdatedComponent>, Error<<R as FetchVersions>::Error>>
where
    R: FetchVersions,
{
    let current_version = &component.current_version;

    let remote_versions = repository
        .fetch_versions(&component)
        .map_err(Error::Repository)?;

    let known_versions = db.known_versions(&component).await?.collect::<HashSet<_>>();
    let unknown_versions = remote_versions
        .iter()
        .filter(|v| !known_versions.contains(*v));

    for version in unknown_versions.clone() {
        db.add_known_version(&component, &version.to_string())
            .await?;
    }

    if let Some(latest_version) = unknown_versions.max()
        && latest_version > current_version
    {
        let projects = db.projects_with(component.id).await?.collect::<Vec<_>>();
        Ok(Some(OutdatedComponent {
            current_version: current_version.clone(),
            latest_version: latest_version.clone(),
            name: component.name,
            projects,
        }))
    } else {
        Ok(None)
    }
}
