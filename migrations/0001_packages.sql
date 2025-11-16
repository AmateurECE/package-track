CREATE TABLE projects (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE components (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  current_version TEXT NOT NULL,
  url TEXT NOT NULL
);

CREATE TABLE project_components (
  id SERIAL PRIMARY KEY,
  component_id INT NOT NULL,
  project_id INT NOT NULL
);

CREATE TABLE known_versions (
  id SERIAL PRIMARY KEY,
  component_id INT NOT NULL,
  version TEXT NOT NULL
);
