INSERT INTO "public"."bw_account" ("account_id", "name", "email", "email_verified_at", "password", "failed_attempt", "status", "last_login", "local_currency", "system_lang", "created_at", "updated_at", "deleted_at") VALUES (6192889942050345985, 'VJ', 'vainjoker@tuta.io', NULL, '$argon2id$v=19$m=19456,t=2,p=1$qx5HhB7crer58ao9Lyov6g$9QK5ZzCQTOlie/8lefC72EwCLrlp3r89bIbLR3dK2qo', 0, 'inactive', NULL, 'USD', 'en-US', '2024-05-21 09:37:51.894308', NULL, NULL);

INSERT INTO public.bw_group (group_id, account_id, name, remark)
VALUES (6193003777960711169,6192889942050345985, 'Group 1', 'This is group 1');

INSERT INTO public.bw_group (group_id, account_id, name, remark)
VALUES (6193003777960711170,6192889942050345985, 'Group 2', 'This is group 2');

INSERT INTO "public"."bw_policy" ("policy_id", "account_id", "name", "settings", "created_at", "updated_at", "deleted_at") VALUES (6194821006046008321, 6192889942050345985, 'test_policy', '[{"mode": "Power", "time": "2024-05-24T03:20:00"}, {"mode": "Idle", "time": "2024-05-24T03:20:00"}]', '2024-05-24 01:34:33.020328', NULL, NULL);
INSERT INTO "public"."bw_policy" ("policy_id", "account_id", "name", "settings", "created_at", "updated_at", "deleted_at") VALUES (6194821006046008322, 6192889942050345985, 'test_policy', '[{"mode": "Power", "time": "2024-05-24T03:20:00"}, {"mode": "Idle", "time": "2024-05-24T03:20:00"}]', '2024-05-24 01:34:33.020328', NULL, NULL);

INSERT INTO "public"."bw_pool" ("pool_id", "account_id", "name", "settings", "created_at", "updated_at", "deleted_at") VALUES (6194824969470350666, 6192889942050345985, 'pool_test', '[{"url": "http://example.com", "coin": "BTC", "user": "username", "suffix": true, "worker": "worker1", "password": "password123"}, {"url": "http://example.com", "coin": "BTC", "user": "username", "suffix": true, "worker": "worker2", "password": "password123"}]', '2024-05-24 01:42:25.501818', NULL, NULL);
INSERT INTO "public"."bw_pool" ("pool_id", "account_id", "name", "settings", "created_at", "updated_at", "deleted_at") VALUES (6194824969470350667, 6192889942050345985, 'pool_test', '[{"url": "http://example.com", "coin": "BTC", "user": "username", "suffix": true, "worker": "worker1", "password": "password123"}, {"url": "http://example.com", "coin": "BTC", "user": "username", "suffix": true, "worker": "worker2", "password": "password123"}]', '2024-05-24 01:42:25.501818', NULL, NULL);

INSERT INTO bw_machine (
    mac, account_id, device_type, device_name, device_ip, group_id, policy_id, setting, hardware_version, software_version, updated_at, deleted_at
) VALUES (
    '08:00:2B:01:02:03', 6192889942050345985, 'Type1', 'Device1', '192.168.1.1', 6193003777960711170, 6194821006046008321, '{"crypto_coin": "Bitcoin", "power_modes": ["Power", "Idle"], "pool_maximal": 3, "support_boot": true, "support_reset": true, "support_update": true, "support_led": true}', '1.0', '1.0', '2024-05-29 02:01:34', NULL
);

INSERT INTO bw_machine (
    mac, account_id, device_type, device_name, device_ip, group_id, pool_id, setting, hardware_version, software_version, updated_at, deleted_at
) VALUES (
    '08:00:2B:01:02:04', 6192889942050345985, 'Type2', 'Device2', '192.168.1.2', 6193003777960711169, 6194824969470350667, '{"crypto_coin": "Ethereum", "power_modes": ["Power", "Idle"], "pool_maximal": 4, "support_boot": false, "support_reset": false, "support_update": false, "support_led": false}', '2.0', '2.0', '2024-05-29 02:01:34', NULL
);

INSERT INTO bw_machine (
    mac, account_id, device_type, device_name, device_ip, group_id, policy_id, pool_id, setting, hardware_version, software_version, updated_at, deleted_at
) VALUES (
    '08:00:2B:01:02:05',6192889942050345985, 'Type3', 'Device3', '192.168.1.3', 6193003777960711170, 6194821006046008322, 6194824969470350667, '{"crypto_coin": "Litecoin", "power_modes": ["Power", "Idle"], "pool_maximal": 2, "support_boot": true, "support_reset": true, "support_update": true, "support_led": true}', '3.0', '3.0', '2024-05-29 02:01:34', NULL
);
