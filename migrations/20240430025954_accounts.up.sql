-- Add up migration script here
CREATE TABLE bw_account (
                            id serial PRIMARY KEY NOT NULL,
                            account_id UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
                            name VARCHAR (255) NOT NULL,
                            email VARCHAR (255) NOT NULL,
                            email_verified_at TIMESTAMP DEFAULT NULL,
                            password VARCHAR (255) NOT NULL,

                            local_currency currency NOT NULL DEFAULT 'USD',
                            system_lang language NOT NULL DEFAULT 'en-US',

                            registered_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_bw_account_name ON bw_account (NAME);
CREATE INDEX idx_bw_account_email ON bw_account (email);

COMMENT ON COLUMN bw_account.ID IS '主键';
COMMENT ON COLUMN bw_account.account_id IS '账户ID';
COMMENT ON COLUMN bw_account.NAME IS '用户名';
COMMENT ON COLUMN bw_account.email IS '用户邮箱';
COMMENT ON COLUMN bw_account.email_verified_at IS '邮箱验证时间';
COMMENT ON COLUMN bw_account.PASSWORD IS '用户密码';
COMMENT ON COLUMN bw_account.local_currency IS '用户本地货币设置';
COMMENT ON COLUMN bw_account.system_lang IS '用户系统语言设置';
COMMENT ON COLUMN bw_account.registered_at IS '用户注册时间';
COMMENT ON COLUMN bw_account.created_at IS '记录创建时间';
COMMENT ON COLUMN bw_account.updated_at IS '记录更新时间';