-- Add down migration script here
DROP INDEX IF EXISTS idx_bw_account_name;
DROP INDEX IF EXISTS idx_bw_account_email;
DROP TABLE IF EXISTS bw_account;

