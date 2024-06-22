INSERT INTO "public"."bw_account" ("uid", "name", "email", "email_verified_at", "password", "failed_attempt", "status", "last_login", "local_currency", "system_lang", "created_at", "updated_at", "deleted_at") VALUES (6192889942050345985, 'VJ', 'vainjoker@tuta.io', NULL, '$argon2id$v=19$m=19456,t=2,p=1$qx5HhB7crer58ao9Lyov6g$9QK5ZzCQTOlie/8lefC72EwCLrlp3r89bIbLR3dK2qo', 0, 'inactive', NULL, 'USD', 'en-US', '2024-05-21 09:37:51.894308', NULL, NULL);

INSERT INTO public.bw_group (group_id, uid, name, remark)
VALUES (6193003777960711169,6192889942050345985, 'Group 1', 'This is group 1');

INSERT INTO public.bw_group (group_id, uid, name, remark)
VALUES (6193003777960711170,6192889942050345985, 'Group 2', 'This is group 2');

INSERT INTO "public"."bw_policy" ("policy_id", "uid", "name", "settings", "created_at", "updated_at", "deleted_at")
VALUES (6194821006046008321, 6192889942050345985, 'test_policy', '[{"mode": "Power", "time": "2024-05-24T03:20:00"}, {"mode": "Idle", "time": "2024-05-24T03:20:00"}]', '2024-05-24 01:34:33.020328', NULL, NULL);
INSERT INTO "public"."bw_policy" ("policy_id", "uid", "name", "settings", "created_at", "updated_at", "deleted_at")
VALUES (6194821006046008322, 6192889942050345985, 'test_policy', '[{"mode": "Power", "time": "2024-05-24T03:20:00"}, {"mode": "Idle", "time": "2024-05-24T03:20:00"}]', '2024-05-24 01:34:33.020328', NULL, NULL);

INSERT INTO "public"."bw_pool" ("pool_id", "uid", "name", "settings", "created_at", "updated_at", "deleted_at")
VALUES (6194824969470350666, 6192889942050345985, 'pool_test', '[{"url": "http://example.com", "coin": "BTC", "user": "username", "suffix": true, "worker": "worker1", "password": "password123"}, {"url": "http://example.com", "coin": "BTC", "user": "username", "suffix": true, "worker": "worker2", "password": "password123"}]', '2024-05-24 01:42:25.501818', NULL, NULL);
INSERT INTO "public"."bw_pool" ("pool_id", "uid", "name", "settings", "created_at", "updated_at", "deleted_at")
VALUES (6194824969470350667, 6192889942050345985, 'pool_test', '[{"url": "http://example.com", "coin": "BTC", "user": "username", "suffix": true, "worker": "worker1", "password": "password123"}, {"url": "http://example.com", "coin": "BTC", "user": "username", "suffix": true, "worker": "worker2", "password": "password123"}]', '2024-05-24 01:42:25.501818', NULL, NULL);

INSERT INTO "public"."bw_machine" ("uid", "created_at", "deleted_at", "device_ip", "device_name", "device_type", "group_id", "hardware_version", "mac", "policy_id", "pool_id", "setting", "software_version", "updated_at")
VALUES ('6192889942050345985', '2024-06-16 09:49:07.221466', NULL, '192.168.110.97', '', 'Goldshell-MiniDOGEPro', NULL, 'GP.CI.IA', '28:e2:97:3e:6f:06', 6194821006046008322, NULL, '{"crypto_coin":[{"symbol":"LTC","algorithm":"scrypt"}],"power_modes":["Power"],"support_led":true,"pool_maximal":3,"support_boot":true,"support_reset":true,"support_update":true}', '2.2.8', NULL);
INSERT INTO "public"."bw_machine" ("uid", "created_at", "deleted_at", "device_ip", "device_name", "device_type", "group_id", "hardware_version", "mac", "policy_id", "pool_id", "setting", "software_version", "updated_at")
VALUES ('6192889942050345985', '2024-06-16 09:49:07.221466', NULL, '192.168.110.97', '', 'Goldshell-MiniDOGEPro', 6193003777960711169, 'GP.CI.IA', '28:e2:97:3e:6f:07', NULL, 6194824969470350667, '{"crypto_coin":[{"symbol":"LTC","algorithm":"scrypt"}],"power_modes":["Power"],"support_led":true,"pool_maximal":3,"support_boot":true,"support_reset":true,"support_update":true}', '2.2.8', NULL);
INSERT INTO "public"."bw_machine" ("uid", "created_at", "deleted_at", "device_ip", "device_name", "device_type", "group_id", "hardware_version", "mac", "policy_id", "pool_id", "setting", "software_version", "updated_at")
VALUES ('6192889942050345985', '2024-06-16 09:49:07.221466', NULL, '192.168.110.97', '', 'Goldshell-MiniDOGEPro', NULL, 'GP.CI.IA', '28:e2:97:3e:6f:08', NULL, 6194824969470350667, '{"crypto_coin":[{"symbol":"LTC","algorithm":"scrypt"}],"power_modes":["Power"],"support_led":true,"pool_maximal":3,"support_boot":true,"support_reset":true,"support_update":true}', '2.2.8', NULL);