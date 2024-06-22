-- Add up migration script here
CREATE TABLE bw_account_setting (
    uid BIGINT PRIMARY KEY,
    key VARCHAR (50) NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE INDEX idx_bw_account_setting_key ON bw_account_setting (key);

ALTER TABLE bw_account_setting ADD FOREIGN KEY (uid) REFERENCES bw_account(uid);