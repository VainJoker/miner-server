-- Add down migration script here
DROP INDEX IF EXISTS idx_bw_pool_uid;
DROP TABLE IF EXISTS bw_pool;