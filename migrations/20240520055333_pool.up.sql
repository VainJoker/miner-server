-- Add up migration script here
CREATE TABLE bw_pool (
    pool_id BIGINT PRIMARY KEY DEFAULT next_id(),
    account_id BIGINT NOT NULL,
    name VARCHAR (50) NOT NULL,

    settings JSONB NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TRIGGER update_bw_pool_updated_at
BEFORE UPDATE ON bw_pool
FOR EACH ROW
EXECUTE FUNCTION update_at();

CREATE INDEX idx_bw_pool_account_id ON bw_pool (account_id);

ALTER TABLE bw_pool ADD FOREIGN KEY (account_id) REFERENCES bw_account(account_id);
