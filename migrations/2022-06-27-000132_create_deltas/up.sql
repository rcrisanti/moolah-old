CREATE TABLE deltas (
    id SERIAL PRIMARY KEY,
    prediction_id INTEGER NOT NULL REFERENCES predictions(id),
    name VARCHAR NOT NULL,
    value REAL NOT NULL,
    positive_uncertainty REAL NOT NULL CHECK(positive_uncertainty >= 0),
    negative_uncertainty REAL NOT NULL CHECK(negative_uncertainty >= 0)
)