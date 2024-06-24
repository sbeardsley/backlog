-- Add migration script here
CREATE TABLE projects (
    id TEXT NOT NULL PRIMARY KEY,
    key CHAR(3) NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_projects_updated_at
AFTER UPDATE ON projects
FOR EACH ROW
BEGIN
    UPDATE projects
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;