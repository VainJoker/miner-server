-- Add up migration script here
CREATE TABLE bw_group (
    group_id BIGINT PRIMARY KEY DEFAULT next_id(),
    uid BIGINT NOT NULL,
    name VARCHAR (50) NOT NULL,

    remark TEXT,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TRIGGER update_bw_group_updated_at
BEFORE UPDATE ON bw_group
FOR EACH ROW
EXECUTE FUNCTION update_at();

CREATE INDEX idx_bw_group_uid ON bw_group (uid);

ALTER TABLE bw_group ADD FOREIGN KEY (uid) REFERENCES bw_account(uid);
