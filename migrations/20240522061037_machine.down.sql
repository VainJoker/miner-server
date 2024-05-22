-- Add down migration script here
DROP INDEX IF EXISTS idx_bw_machine_account_id;
DROP TABLE IF EXISTS bw_machine;