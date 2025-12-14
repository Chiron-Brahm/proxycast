use rusqlite::Connection;
use serde_json::Value;
use std::path::PathBuf;

/// 从旧的 JSON 配置迁移数据到 SQLite
pub fn migrate_from_json(
    conn: &Connection,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 检查是否已经迁移过
    let migrated: bool = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'migrated_from_json'",
            [],
            |row| row.get::<_, String>(0),
        )
        .map(|v| v == "true")
        .unwrap_or(false);

    if migrated {
        return Ok(());
    }

    // 读取旧配置文件
    let home = dirs::home_dir().ok_or("Cannot find home directory")?;
    let config_path = home.join(".proxycast").join("config.json");

    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)?;
        let config: Value = serde_json::from_str(&content)?;

        // TODO: 解析旧配置并插入到数据库
        // 这里需要根据实际的旧配置格式来实现

        // 备份旧配置
        let backup_path = home.join(".proxycast").join("config.json.backup");
        std::fs::copy(&config_path, &backup_path)?;
    }

    // 标记迁移完成
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES ('migrated_from_json', 'true')",
        [],
    )?;

    Ok(())
}
