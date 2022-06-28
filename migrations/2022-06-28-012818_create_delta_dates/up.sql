CREATE TABLE delta_dates (
    delta_id INTEGER NOT NULL REFERENCES deltas(id),
    date DATE NOT NULL,

    PRIMARY KEY (delta_id, date)
)