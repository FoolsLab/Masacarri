CREATE TABLE pages (
  id UUID PRIMARY KEY,
  title VARCHAR(1024) NOT NULL,
  page_url VARCHAR(512) NOT NULL UNIQUE,
  published BOOLEAN NOT NULL
);
