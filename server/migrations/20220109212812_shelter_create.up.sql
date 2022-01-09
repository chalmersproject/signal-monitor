CREATE TABLE shelter (
    id BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    name VARCHAR(255),
    about TEXT,
    phone VARCHAR(15) NOT NULL,
    email VARCHAR(255),
    website_url VARCHAR(2048)
);
