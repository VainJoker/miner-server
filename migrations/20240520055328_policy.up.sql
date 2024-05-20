-- Add up migration script here
CREATE TABLE bw_policy (
    policy_id BIGINT PRIMARY KEY DEFAULT next_id(),
    account_id BIGINT NOT NULL,
    name VARCHAR (255) NOT NULL,

    policy_settings JSONB NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TRIGGER update_bw_policy_updated_at
BEFORE UPDATE ON bw_policy
FOR EACH ROW
EXECUTE FUNCTION update_at();

CREATE INDEX idx_bw_policy_account_id ON bw_policy (account_id);

ALTER TABLE bw_policy ADD FOREIGN KEY (account_id) REFERENCES bw_account(account_id);
