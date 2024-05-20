-- Add down migration script here
DROP INDEX IF EXISTS idx_bw_template_account_id;
DROP TABLE IF EXISTS bw_template;