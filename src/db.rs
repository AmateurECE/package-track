use std::collections::HashSet;

use sqlx::PgPool as DbPool;

use crate::{
    component::{Component, Project, Repository},
    version::Version,
};

pub struct Database {
    pool: DbPool,
}

impl Database {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn components(&self) -> Result<impl Iterator<Item = Component>, sqlx::Error> {
        Ok(
            sqlx::query_as::<_, (i32, String, String, String)>("SELECT * FROM components")
                .fetch_all(&self.pool)
                .await?
                .into_iter()
                .map(|(id, name, version, url)| Component {
                    id,
                    name,
                    // INVARIANT: Any version in the database is a valid version
                    current_version: version.parse().unwrap(),
                    // TODO: Support more than just GitHub
                    repository: Repository::GitHub(url),
                }),
        )
    }

    pub async fn known_versions(
        &self,
        component: &Component,
    ) -> Result<impl Iterator<Item = Version>, sqlx::Error> {
        Ok(sqlx::query_as::<_, (String,)>(
            "SELECT version FROM known_versions WHERE component_id = $1",
        )
        .bind(component.id)
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|(v,)| v.parse().unwrap()))
    }

    pub async fn add_known_version(
        &self,
        component: &Component,
        version: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO known_versions (component_id, version) VALUES ($1, $2)")
            .bind(component.id)
            .bind(version)
            .execute(&self.pool)
            .await
            .map(|_| ())
    }

    pub async fn projects_with(
        &self,
        component_id: i32,
    ) -> Result<impl Iterator<Item = Project>, sqlx::Error> {
        let project_ids = sqlx::query_as::<_, (i32,)>(
            "SELECT project_id FROM project_components WHERE component_id = $1",
        )
        .bind(component_id)
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|(id,)| id)
        .collect::<HashSet<i32>>();

        Ok(sqlx::query_as::<_, (i32, String)>("SELECT * FROM projects")
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .filter(move |(id, _)| project_ids.contains(id))
            .map(|(_, name)| Project { name }))
    }
}
