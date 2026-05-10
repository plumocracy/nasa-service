CREATE TABLE IF NOT EXISTS apod (
	id INTEGER PRIMARY KEY NOT NULL,
	copyright VARCHAR(255) NOT NULL,
	explanation TEXT,
	hdurl VARCHAR(1024),
	media_type VARCHAR(256),
	service_version VARCHAR(10),
	title VARCHAR(256),
	url VARCHAR(1024)
);
