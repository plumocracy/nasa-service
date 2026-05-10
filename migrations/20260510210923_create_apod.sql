CREATE TABLE apod (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    title TEXT NOT NULL,
    explanation TEXT NOT NULL,

    date TEXT NOT NULL UNIQUE,

    media_type TEXT NOT NULL,
    service_version TEXT NOT NULL,

    url TEXT NOT NULL,
    hdurl TEXT,

    copyright TEXT,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
