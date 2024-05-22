-- Add up migration script here
CREATE TABLE bw_action (
    mac MACADDR PRIMARY KEY,
    account_id BIGINT,
    action action,
    remark TEXT,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TRIGGER update_bw_action_updated_at
BEFORE UPDATE ON bw_action
FOR EACH ROW
EXECUTE FUNCTION update_at();