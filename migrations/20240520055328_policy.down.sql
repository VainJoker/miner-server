-- Add down migration script here
DROP INDEX IF EXISTS idx_bw_policy_uid;
DROP TABLE IF EXISTS bw_policy;