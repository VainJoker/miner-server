-- Add up migration script here
CREATE TABLE bw_machine (
    mac MACADDR NOT NULL,
    uid BIGINT NOT NULL,
    device_type VARCHAR (50) NOT NULL,
    device_name VARCHAR (50) NOT NULL,
    device_ip INET NOT NULL,
    setting JSONB NOT NULL,
    hardware_version VARCHAR (50) NOT NULL,
    software_version VARCHAR (50) NOT NULL,

    group_id BIGINT,
    policy_id BIGINT,
    pool_id BIGINT,

    exist bool NOT NULL DEFAULT TRUE,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,

    PRIMARY KEY (mac, uid)
);


CREATE TRIGGER update_bw_machine_updated_at
BEFORE UPDATE ON bw_machine
FOR EACH ROW
EXECUTE FUNCTION update_at();

CREATE INDEX idx_bw_machine_uid ON bw_machine (uid);
CREATE INDEX idx_bw_machine_group_id ON bw_machine (group_id);
CREATE INDEX idx_bw_machine_policy_id ON bw_machine (policy_id);
CREATE INDEX idx_bw_machine_pool_id ON bw_machine (pool_id);

ALTER TABLE bw_machine ADD FOREIGN KEY (uid) REFERENCES bw_account(uid);
ALTER TABLE bw_machine ADD FOREIGN KEY (group_id) REFERENCES bw_group(group_id);
ALTER TABLE bw_machine ADD FOREIGN KEY (policy_id) REFERENCES bw_policy(policy_id);
ALTER TABLE bw_machine ADD FOREIGN KEY (pool_id) REFERENCES bw_pool(pool_id);
