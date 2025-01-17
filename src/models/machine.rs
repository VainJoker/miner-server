use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

use crate::{
    library::{error::InnerResult, DB},
    models::types::EnergyMode,
};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
#[sqlx(rename_all = "lowercase")]
pub struct BwMachine {
    pub mac: String,
    pub uid: i64,

    pub device_type: String,
    pub device_name: String,
    pub device_ip: String,

    pub group_id: Option<i64>,
    pub policy_id: Option<i64>,
    pub pool_id: Option<i64>,

    pub setting: Json<Setting>,

    pub hardware_version: String,
    pub software_version: String,

    pub exist: bool,

    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Coin {
    pub algorithm: String,
    pub symbol: String,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
#[sqlx(rename_all = "lowercase")]
pub struct Setting {
    pub crypto_coin: Vec<Coin>,
    pub power_modes: Vec<EnergyMode>,
    pub pool_maximal: usize,

    pub support_boot: bool,
    pub support_reset: bool,
    pub support_update: bool,
    pub support_led: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MachineStatus {
    pub mac: String,

    pub device_type: String,
    pub device_ip: String,

    pub current_rate: String,
    pub average_rate: String,
    pub history_rate: String,

    pub energy_mode: EnergyMode,
    pub dig_time: String,
    pub hard_err: String,
    pub refuse: String,

    pub device_temp: String,
    pub device_fan: String,
    pub device_status: String,
}

#[derive(Debug, Clone)]
pub struct CreateBwMachineSchema<'a> {
    pub mac: &'a str,
    pub uid: i64,

    pub device_type: &'a str,
    pub device_name: &'a str,
    pub device_ip: &'a str,

    pub setting: Json<Setting>,

    pub hardware_version: &'a str,
    pub software_version: &'a str,
}

#[derive(Debug, Clone)]
pub struct UpdateOtherBwMachineSchema<'a> {
    pub mac: &'a str,
    pub uid: i64,
}

#[derive(Debug, Clone)]
pub struct UpdateGroupSchema<'a> {
    pub mac: &'a str,
    pub uid: i64,
    pub group_id: i64,
}

#[derive(Debug, Clone)]
pub struct UpdatePolicySchema<'a> {
    pub mac: &'a str,
    pub uid: i64,
    pub policy_id: i64,
}

#[derive(Debug, Clone)]
pub struct UpdatePoolSchema<'a> {
    pub mac: &'a str,
    pub uid: i64,
    pub pool_id: i64,
}

#[derive(Debug, Clone)]
pub struct DeleteBwMachineSchema<'a> {
    pub mac: &'a str,
    pub uid: i64,
}

#[derive(Debug, Clone)]
pub struct ReadBwMachineSchema<'a> {
    pub macs: Vec<&'a str>,
    pub uid: i64,
}

impl BwMachine {
    pub async fn create_bw_machine(
        db: &DB,
        item: &CreateBwMachineSchema<'_>,
    ) -> InnerResult<Self> {
        let sql = r#"
            INSERT INTO bw_machine
                (mac, uid, device_type, device_name, device_ip, setting, hardware_version, software_version)
            VALUES
                (MACADDR($1), $2, $3, $4, INET($5), $6, $7, $8)
            ON CONFLICT (mac,uid) DO UPDATE
                SET exist = true
            RETURNING mac::VARCHAR, uid, device_type, device_name, device_ip::VARCHAR, group_id,
                policy_id, pool_id, setting, hardware_version, software_version, exist, created_at, updated_at, deleted_at
            "#;

        let map = sqlx::query_as(sql)
            .bind(item.mac)
            .bind(item.uid)
            .bind(item.device_type)
            .bind(item.device_name)
            .bind(item.device_ip)
            .bind(item.setting.clone())
            .bind(item.hardware_version)
            .bind(item.software_version);

        Ok(map.fetch_one(db).await?)
    }

    pub async fn update_other_machines(
        db: &DB,
        item: &UpdateOtherBwMachineSchema<'_>,
    ) -> InnerResult<u64> {
        let sql = r#"
            UPDATE bw_machine SET exist = false
            WHERE uid != $1 AND mac = MACADDR($2)
            "#;

        let map = sqlx::query(sql).bind(item.uid).bind(item.mac);

        Ok(map.execute(db).await?.rows_affected())
    }

    pub async fn update_group_id(
        db: &DB,
        item: &UpdateGroupSchema<'_>,
    ) -> InnerResult<Self> {
        let sql = r#"
            UPDATE bw_machine
            SET group_id = $1
            WHERE mac = MACADDR($2) AND uid = $3
            RETURNING mac::VARCHAR, uid, device_type, device_name, device_ip::VARCHAR,
            group_id, policy_id, pool_id, setting, hardware_version, software_version, exist, created_at, updated_at, deleted_at
            "#;

        let map = sqlx::query_as(sql)
            .bind(item.group_id)
            .bind(item.mac)
            .bind(item.uid);

        Ok(map.fetch_one(db).await?)
    }

    pub async fn update_policy_id(
        db: &DB,
        item: &UpdatePolicySchema<'_>,
    ) -> InnerResult<Self> {
        let sql = r#"
            UPDATE bw_machine
            SET policy_id = $1
            WHERE mac = MACADDR($2) AND uid = $3
            RETURNING mac::VARCHAR, uid, device_type, device_name, device_ip::VARCHAR, group_id, policy_id, pool_id, setting, hardware_version, software_version, exist, created_at, updated_at, deleted_at
            "#;

        let map = sqlx::query_as(sql)
            .bind(item.policy_id)
            .bind(item.mac)
            .bind(item.uid);

        Ok(map.fetch_one(db).await?)
    }

    pub async fn update_pool_id(
        db: &DB,
        item: &UpdatePoolSchema<'_>,
    ) -> InnerResult<Self> {
        let sql = r#"
            UPDATE bw_machine
            SET pool_id = $1
            WHERE mac = MACADDR($2) AND uid = $3
            RETURNING mac::VARCHAR, uid, device_type, device_name, device_ip::VARCHAR, group_id, policy_id, pool_id, setting, hardware_version, software_version, exist, created_at, updated_at, deleted_at
            "#;

        let map = sqlx::query_as(sql)
            .bind(item.pool_id)
            .bind(item.mac)
            .bind(item.uid);

        Ok(map.fetch_one(db).await?)
    }

    pub async fn delete_bw_machine(
        db: &DB,
        item: &DeleteBwMachineSchema<'_>,
    ) -> InnerResult<u64> {
        let sql = "UPDATE bw_machine SET deleted_at = NOW(), exist = false WHERE mac = MACADDR($1) AND uid = $2";
        let map = sqlx::query(sql).bind(item.mac).bind(item.uid);
        Ok(map.execute(db).await?.rows_affected())
    }

    pub async fn fetch_machines_by_uid(
        db: &DB,
        uid: i64,
    ) -> InnerResult<Vec<Self>> {
        let sql = r#"SELECT mac::VARCHAR, uid, device_type, device_name, device_ip::VARCHAR,
        group_id, policy_id, pool_id, setting, hardware_version, software_version, exist,
        created_at,updated_at,deleted_at from bw_machine
        WHERE uid = $1 AND exist = true AND deleted_at IS NULL"#;
        let map = sqlx::query_as(sql).bind(uid);
        Ok(map.fetch_all(db).await?)
    }

    pub async fn fetch_machine_by_mac(
        db: &DB,
        mac: &str,
    ) -> InnerResult<Vec<Self>> {
        let sql = r#"SELECT mac::VARCHAR, uid, device_type, device_name, device_ip::VARCHAR,
        group_id, policy_id, pool_id, setting, hardware_version, software_version, exist,
        created_at,updated_at,deleted_at from bw_machine
        WHERE mac = MACADDR($1) AND exist = true AND deleted_at IS NULL"#;
        let map = sqlx::query_as(sql).bind(mac);
        Ok(map.fetch_all(db).await?)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::*;
    const ACCOUNT_ID: i64 = 6192889942050345985;
    const MAC1: &str = "28:e2:97:3e:6f:06";
    const _MAC2: &str = "28:e2:97:3e:6f:10";
    const GROUP_ID: i64 = 6193003777960711169;
    const POLICY_ID: i64 = 6194821006046008321;
    const POOL_ID: i64 = 6194824969470350666;
    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_create_bw_machine(pool: PgPool) {
        let item = CreateBwMachineSchema {
            mac: "28:E2:97:1E:AB:91",
            uid: ACCOUNT_ID,
            device_type: "type",
            device_name: "name",
            device_ip: "192.168.0.109",
            setting: Json(Setting {
                crypto_coin: vec![Coin {
                    algorithm: "crypto".to_string(),
                    symbol: "LTC".to_string(),
                }],
                power_modes: vec![EnergyMode::Power, EnergyMode::Balance],
                pool_maximal: 3,
                support_boot: true,
                support_reset: true,
                support_update: true,
                support_led: true,
            }),
            hardware_version: "version",
            software_version: "version",
        };

        let res = BwMachine::create_bw_machine(&pool, &item).await.unwrap();
        assert_eq!(res.setting.crypto_coin, item.setting.crypto_coin);
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("machine")))]
    async fn test_update_other_machines(pool: PgPool) {
        let item = UpdateOtherBwMachineSchema {
            mac: MAC1,
            uid: ACCOUNT_ID,
        };
        let rows_affected = BwMachine::update_other_machines(&pool, &item)
            .await
            .unwrap();
        assert_eq!(rows_affected, 1)
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("machine")))]
    async fn test_update_group_id(pool: PgPool) {
        let item = UpdateGroupSchema {
            mac: MAC1,
            uid: ACCOUNT_ID,
            group_id: GROUP_ID,
        };
        let res = BwMachine::update_group_id(&pool, &item).await.unwrap();
        assert_eq!(res.group_id.unwrap(), item.group_id);
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("machine")))]
    async fn test_update_policy_id(pool: PgPool) {
        let item = UpdatePolicySchema {
            mac: MAC1,
            uid: ACCOUNT_ID,
            policy_id: POLICY_ID,
        };
        let res = BwMachine::update_policy_id(&pool, &item).await.unwrap();
        assert_eq!(res.policy_id.unwrap(), item.policy_id);
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("machine")))]
    async fn test_update_pool_id(pool: PgPool) {
        let item = UpdatePoolSchema {
            mac: MAC1,
            uid: ACCOUNT_ID,
            pool_id: POOL_ID,
        };
        let res = BwMachine::update_pool_id(&pool, &item).await.unwrap();
        assert_eq!(res.pool_id.unwrap(), item.pool_id);
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("machine")))]
    async fn test_delete_bw_machine(pool: PgPool) {
        let item = DeleteBwMachineSchema {
            mac: MAC1,
            uid: ACCOUNT_ID,
        };
        let res = BwMachine::delete_bw_machine(&pool, &item).await.unwrap();
        assert_eq!(res, 1);
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("machine")))]
    async fn test_fetch_bw_machine_by_mac(pool: PgPool) {
        let res = BwMachine::fetch_machine_by_mac(&pool, MAC1).await.unwrap();
        assert_eq!(res.len(), 1);
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("machine")))]
    async fn test_fetch_machine_by_uid(pool: PgPool) {
        let res = BwMachine::fetch_machines_by_uid(&pool, ACCOUNT_ID)
            .await
            .unwrap();
        assert_eq!(res.len(), 2);
    }
}
