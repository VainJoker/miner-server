-- Add down migration script here
DROP INDEX IF EXISTS idx_bw_group_account_id;
DROP TABLE IF EXISTS bw_group;