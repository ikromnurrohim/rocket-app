CREATE TABLE rustaceans (
                            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                            name VARCHAR NOT NULL,
                            email VARCHAR NOT NULL,
                            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)