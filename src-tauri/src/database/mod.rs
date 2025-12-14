pub mod dao;
pub mod migration;
pub mod schema;

use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub type DbConnection = Arc<Mutex<Connection>>;

/// 获取数据库文件路径
pub fn get_db_path() -> PathBuf {
    let home = dirs::home_dir().expect("Cannot find home directory");
    let db_dir = home.join(".proxycast");
    std::fs::create_dir_all(&db_dir).expect("Cannot create .proxycast directory");
    db_dir.join("proxycast.db")
}

/// 初始化数据库连接
pub fn init_database() -> Result<DbConnection, rusqlite::Error> {
    let db_path = get_db_path();
    let conn = Connection::open(&db_path)?;

    // 创建表结构
    schema::create_tables(&conn)?;

    Ok(Arc::new(Mutex::new(conn)))
}
