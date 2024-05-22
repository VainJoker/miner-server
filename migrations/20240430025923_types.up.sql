-- Add up migration script here
CREATE TYPE currency AS ENUM ('USD', 'EUR', 'GBP', 'CNY');
COMMENT ON TYPE currency IS '枚举类型，表示本地货币选项';

CREATE TYPE language AS ENUM ('en-US', 'zh-CN', 'fr-FR', 'es-ES');
COMMENT ON TYPE language IS '枚举类型，表示系统语言选项';

CREATE TYPE account_status AS ENUM ('active', 'inactive', 'suspended');
COMMENT ON TYPE account_status IS '枚举类型，表示账号状态';

CREATE TYPE action AS ENUM (
    'restart',
    'sendminer',
    'setgroup',
    'setpool',
    'setpowermode',
    'setled',
    'resettofactory',
    'offline',
    'upgrade',
    'delete'
);
COMMENT ON TYPE action IS '枚举类型，表示操作类型';

CREATE TYPE energy_mode AS ENUM (
    'power',
    'idle',
    'balance',
    'economize'
);
COMMENT ON TYPE action IS '枚举类型，表示电源模式';