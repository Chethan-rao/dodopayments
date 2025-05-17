-- Your SQL goes here

CREATE TABLE IF NOT EXISTS transactions (
    id SERIAL PRIMARY KEY,
    transaction_id VARCHAR(64) NOT NULL UNIQUE,
    sender_id VARCHAR(64) NOT NULL,  
    recipient_id VARCHAR(64) NOT NULL, 
    amount_in_rs BIGINT NOT NULL, 
    description TEXT, 
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    status VARCHAR(32) NOT NULL DEFAULT 'pending', 
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    FOREIGN KEY (sender_id) REFERENCES users(user_id), 
    FOREIGN KEY (recipient_id) REFERENCES users(user_id) 
);

CREATE INDEX IF NOT EXISTS transactions_sender_id_created_at_idx ON transactions (sender_id, created_at);
CREATE INDEX IF NOT EXISTS transactions_recipient_id_created_at_idx ON transactions (recipient_id, created_at);