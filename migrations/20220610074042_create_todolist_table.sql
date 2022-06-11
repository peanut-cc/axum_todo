-- Add migration script here
-- create todo_list and tod_item table
CREATE TABLE todo_list(
    id SERIAL PRIMARY KEY,
    title VARCHAR(150) NOT NULL
);

CREATE TABLE todo_item(
    id SERIAL PRIMARY KEY,
    title VARCHAR(150) NOT NULL,
    checked BOOLEAN NOT NULL DEFAULT FALSE,
    list_id INTEGER NOT NULL,
    FOREIGN KEY(list_id) REFERENCES todo_list(id)
);

