-- Add up migration script here
CREATE TABLE bw_policy (
    policy_id BIGINT PRIMARY KEY DEFAULT next_id(),
    uid BIGINT NOT NULL,
    name VARCHAR (50) NOT NULL,

    settings JSONB NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TRIGGER update_bw_policy_updated_at
BEFORE UPDATE ON bw_policy
FOR EACH ROW
EXECUTE FUNCTION update_at();

CREATE INDEX idx_bw_policy_uid ON bw_policy (uid);

ALTER TABLE bw_policy ADD FOREIGN KEY (uid) REFERENCES bw_account(uid);
