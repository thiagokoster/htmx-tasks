-- Add migration script here
CREATE TABLE IF NOT EXISTS task
(
    id          INTEGER PRIMARY KEY NOT NULL,
    title       TEXT                NOT NULL,
    created_on  DATETIME DEFAULT (datetime('now', 'localtime')),
    done        BOOLEAN             NOT NULL DEFAULT 0
)
