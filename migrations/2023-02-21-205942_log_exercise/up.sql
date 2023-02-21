CREATE TABLE weighttraining(
    id SERIAL PRIMARY KEY,
    lift VARCHAR NOT NULL,
    weight INT NOT NULL,
    reps INT NOT NULL,
    RPE INT,
    notes TEXT
)