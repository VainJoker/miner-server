-- Add up migration script here
CREATE TABLE bw_machine (
    mac MACADDR PRIMARY KEY,
    account_id BIGINT NOT NULL,
    device_type VARCHAR (50) NOT NULL,
    device_name VARCHAR (50),
    device_ip INET NOT NULL,
    group_id BIGINT,
    policy_id BIGINT,
    pool_id BIGINT,
    setting JSONB,
    hardware_version VARCHAR (50),
    software_version VARCHAR (50),

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);


CREATE TRIGGER update_bw_machine_updated_at
BEFORE UPDATE ON bw_machine
FOR EACH ROW
EXECUTE FUNCTION update_at();

CREATE INDEX idx_bw_machine_account_id ON bw_machine (account_id);

ALTER TABLE bw_machine ADD FOREIGN KEY (account_id) REFERENCES bw_account(account_id);
ALTER TABLE bw_machine ADD FOREIGN KEY (group_id) REFERENCES bw_group(group_id);
ALTER TABLE bw_machine ADD FOREIGN KEY (policy_id) REFERENCES bw_policy(policy_id);
ALTER TABLE bw_machine ADD FOREIGN KEY (pool_id) REFERENCES bw_pool(pool_id);