-- Master Wallet Table
CREATE TABLE IF NOT EXISTS master_wallet (
    id INTEGER PRIMARY KEY DEFAULT 1,
    balance REAL NOT NULL DEFAULT 0.0,
    total_fees_collected REAL NOT NULL DEFAULT 0.0,
    total_payouts REAL NOT NULL DEFAULT 0.0,
    created_at TEXT DEFAULT (datetime('now')),
    last_updated TEXT DEFAULT (datetime('now')),
    CHECK (id = 1)
);

-- Fee Transactions Table
CREATE TABLE IF NOT EXISTS fee_transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    block_height INTEGER NOT NULL,
    miner_address TEXT NOT NULL,
    block_reward REAL NOT NULL,
    fee_amount REAL NOT NULL,
    miner_payout REAL NOT NULL,
    collected_at TEXT DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_fee_block_height ON fee_transactions(block_height);
CREATE INDEX IF NOT EXISTS idx_fee_miner ON fee_transactions(miner_address);

-- Users Table
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    totp_secret TEXT NOT NULL,
    is_admin INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT DEFAULT (datetime('now')),
    last_login TEXT,
    wallet_address TEXT
);

-- Wallets Table
CREATE TABLE IF NOT EXISTS wallets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    address TEXT UNIQUE NOT NULL,
    user_id INTEGER,
    balance REAL NOT NULL DEFAULT 0.0,
    pending_balance REAL NOT NULL DEFAULT 0.0,
    total_mined REAL NOT NULL DEFAULT 0.0,
    total_paid REAL NOT NULL DEFAULT 0.0,
    created_at TEXT DEFAULT (datetime('now')),
    last_activity TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

-- Transactions Table
CREATE TABLE IF NOT EXISTS transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tx_hash TEXT UNIQUE NOT NULL,
    from_address TEXT,
    to_address TEXT NOT NULL,
    amount REAL NOT NULL,
    fee REAL DEFAULT 0.0,
    status TEXT NOT NULL DEFAULT 'pending',
    block_height INTEGER,
    confirmations INTEGER DEFAULT 0,
    created_at TEXT DEFAULT (datetime('now')),
    confirmed_at TEXT
);
CREATE INDEX IF NOT EXISTS idx_tx_from ON transactions(from_address);
CREATE INDEX IF NOT EXISTS idx_tx_to ON transactions(to_address);
CREATE INDEX IF NOT EXISTS idx_tx_status ON transactions(status);

-- Mining Workers Table
CREATE TABLE IF NOT EXISTS mining_workers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    worker_id TEXT UNIQUE NOT NULL,
    miner_address TEXT NOT NULL,
    hashrate REAL DEFAULT 0.0,
    shares_submitted INTEGER DEFAULT 0,
    shares_accepted INTEGER DEFAULT 0,
    shares_rejected INTEGER DEFAULT 0,
    last_share_time TEXT,
    status TEXT DEFAULT 'offline',
    connected_at TEXT DEFAULT (datetime('now')),
    last_seen TEXT DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_worker_miner ON mining_workers(miner_address);
CREATE INDEX IF NOT EXISTS idx_worker_status ON mining_workers(status);

-- Blocks Table
CREATE TABLE IF NOT EXISTS blocks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    height INTEGER UNIQUE NOT NULL,
    hash TEXT UNIQUE NOT NULL,
    prev_hash TEXT NOT NULL,
    miner_address TEXT NOT NULL,
    reward REAL NOT NULL,
    difficulty REAL NOT NULL,
    nonce INTEGER NOT NULL,
    timestamp TEXT NOT NULL,
    status TEXT DEFAULT 'orphaned',
    confirmations INTEGER DEFAULT 0,
    found_at TEXT DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_block_height ON blocks(height);
CREATE INDEX IF NOT EXISTS idx_block_miner ON blocks(miner_address);

-- Pool Statistics Table
CREATE TABLE IF NOT EXISTS pool_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    total_hashrate REAL NOT NULL DEFAULT 0.0,
    active_workers INTEGER NOT NULL DEFAULT 0,
    blocks_found INTEGER NOT NULL DEFAULT 0,
    total_paid REAL NOT NULL DEFAULT 0.0,
    pool_fee_percent REAL NOT NULL DEFAULT 2.0,
    min_payout REAL NOT NULL DEFAULT 1.0,
    recorded_at TEXT DEFAULT (datetime('now'))
);

-- Payouts Table
CREATE TABLE IF NOT EXISTS payouts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    wallet_address TEXT NOT NULL,
    amount REAL NOT NULL,
    tx_hash TEXT,
    status TEXT DEFAULT 'pending',
    created_at TEXT DEFAULT (datetime('now')),
    processed_at TEXT
);
CREATE INDEX IF NOT EXISTS idx_payout_wallet ON payouts(wallet_address);

-- Shares Table
CREATE TABLE IF NOT EXISTS shares (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    worker_id TEXT NOT NULL,
    miner_address TEXT NOT NULL,
    difficulty REAL NOT NULL,
    valid INTEGER NOT NULL DEFAULT 1,
    submitted_at TEXT DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_share_worker ON shares(worker_id);
CREATE INDEX IF NOT EXISTS idx_share_miner ON shares(miner_address);

-- Initialize master wallet
INSERT OR IGNORE INTO master_wallet (id, balance, total_fees_collected, total_payouts)
VALUES (1, 0.0, 0.0, 0.0);

-- Insert initial pool statistics
INSERT INTO pool_stats (total_hashrate, active_workers, blocks_found, total_paid, pool_fee_percent, min_payout)
VALUES (0.0, 0, 0, 0.0, 2.0, 1.0);
