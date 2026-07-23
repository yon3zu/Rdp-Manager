CREATE TABLE groups (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    parent_id   TEXT REFERENCES groups(id) ON DELETE CASCADE,
    sort_order  INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL
);
CREATE INDEX idx_groups_parent ON groups(parent_id);

CREATE TABLE connection_profiles (
    id                      TEXT PRIMARY KEY,
    group_id                TEXT REFERENCES groups(id) ON DELETE SET NULL,
    name                    TEXT NOT NULL,
    protocol                TEXT NOT NULL DEFAULT 'rdp',
    host                    TEXT NOT NULL,
    port                    INTEGER NOT NULL DEFAULT 3389,
    username                TEXT,
    domain                  TEXT,
    has_saved_password      INTEGER NOT NULL DEFAULT 0,
    has_saved_gateway_password INTEGER NOT NULL DEFAULT 0,
    sort_order              INTEGER NOT NULL DEFAULT 0,
    screen_mode             TEXT NOT NULL DEFAULT 'windowed',
    desktop_width           INTEGER,
    desktop_height          INTEGER,
    dynamic_resolution      INTEGER NOT NULL DEFAULT 1,
    color_depth             INTEGER NOT NULL DEFAULT 32,
    multi_monitor           INTEGER NOT NULL DEFAULT 0,
    selected_monitors       TEXT,
    admin_session           INTEGER NOT NULL DEFAULT 0,
    redirect_drives         INTEGER NOT NULL DEFAULT 0,
    redirect_printers       INTEGER NOT NULL DEFAULT 0,
    redirect_clipboard      INTEGER NOT NULL DEFAULT 1,
    audio_mode               TEXT NOT NULL DEFAULT 'local',
    mic_redirection          INTEGER NOT NULL DEFAULT 0,
    gateway_hostname         TEXT,
    gateway_port             INTEGER,
    gateway_username         TEXT,
    gateway_domain           TEXT,
    gateway_usage             TEXT NOT NULL DEFAULT 'none',
    cert_trust_behavior       TEXT NOT NULL DEFAULT 'prompt',
    connection_timeout_ms     INTEGER,
    notes                     TEXT,
    created_at                TEXT NOT NULL,
    updated_at                TEXT NOT NULL
);
CREATE INDEX idx_profiles_group ON connection_profiles(group_id);
