-- accounts: 存储每个 OTP 条目的 metadata 与加密 ciphertext
CREATE TABLE IF NOT EXISTS accounts
(
    id            TEXT PRIMARY KEY, -- UUID
    issuer        TEXT,
    label         TEXT    NOT NULL,
    type          TEXT    NOT NULL, -- 'TOTP' | 'HOTP'
    algorithm     TEXT    NOT NULL, -- 'SHA1' | 'SHA256' | 'SHA512'
    digits        INTEGER NOT NULL,
    period        INTEGER,          -- TOTP 专用（单位 秒）
    counter       INTEGER,          -- HOTP 专用
    secret_cipher BLOB    NOT NULL, -- AEAD ciphertext
    secret_nonce  BLOB    NOT NULL, -- nonce (XChaCha20-Poly1305 24 bytes)
    created_at    INTEGER NOT NULL,
    updated_at    INTEGER NOT NULL,
    icon          BLOB,             -- optional
    note          TEXT
);

-- meta: 存储一些应用级元数据，例如加密参数版本等
CREATE TABLE IF NOT EXISTS meta
(
    key   TEXT PRIMARY KEY,
    value TEXT
);