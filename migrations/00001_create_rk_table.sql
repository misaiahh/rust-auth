CREATE TABLE
    IF NOT EXISTS RegistrationKeys (
        id SERIAL PRIMARY KEY,
        registration_key TEXT,
        owned BOOLEAN DEFAULT FALSE,
        owned_by INTEGER
    );