-- Multi-game scalable enums
CREATE TYPE game_type AS ENUM ('BGMI', 'VALORANT', 'CS2', 'APEX', 'FORTNITE', 'LOL', 'DOTA2', 'PUBG', 'COD');
CREATE TYPE tournament_status AS ENUM ('announced', 'registration_open', 'registration_closed', 'in_progress', 'completed', 'cancelled', 'postponed');
CREATE TYPE team_status AS ENUM ('active', 'inactive', 'disbanded', 'looking_for_players');
CREATE TYPE battle_status AS ENUM ('scheduled', 'in_progress', 'completed', 'cancelled');
CREATE TYPE approval_status AS ENUM ('pending', 'approved', 'rejected', 'not_applicable');
CREATE TYPE admin_role AS ENUM ('super_admin', 'admin', 'moderator');

-- Core Players (UNCHANGED - All existing columns preserved)
CREATE TABLE players (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    in_game_name VARCHAR(100),
    real_name VARCHAR(100),
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    verified BOOLEAN DEFAULT FALSE,
    country VARCHAR(100),
    bio TEXT DEFAULT '',
    profile_picture TEXT DEFAULT '',
    primary_game game_type,
    earnings DECIMAL(15,2) DEFAULT 0,
    in_game_role TEXT[],
    location VARCHAR(100),
    age INTEGER CHECK (age >= 13 AND age <= 99),
    languages TEXT[],
    aegis_rating INTEGER DEFAULT 0,
    tournaments_played INTEGER DEFAULT 0,
    battles_played INTEGER DEFAULT 0, 
    qualified_events BOOLEAN DEFAULT FALSE,
    qualified_event_details TEXT[],
    team_status VARCHAR(50),
    team_id UUID,
    availability VARCHAR(50),
    discord_tag VARCHAR(100) DEFAULT '',
    twitch VARCHAR(255) DEFAULT '',
    youtube VARCHAR(255) DEFAULT '',
    twitter VARCHAR(255) DEFAULT '',
    profile_visibility VARCHAR(20) DEFAULT 'public',
    card_theme VARCHAR(20) DEFAULT 'orange',
    coins BIGINT DEFAULT 0,
    last_check_in TIMESTAMPTZ,
    check_in_streak INTEGER DEFAULT 0,
    total_check_ins INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Game-specific player stats (UNCHANGED)
CREATE TABLE player_game_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    game_type game_type NOT NULL,
    rank_tier VARCHAR(50),
    battles_played INTEGER DEFAULT 0, 
    wins INTEGER DEFAULT 0,
    kills INTEGER DEFAULT 0,
    game_specific_stats JSONB DEFAULT '{}',
    last_updated TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(player_id, game_type)
);

-- Teams (UNCHANGED)
CREATE TABLE teams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_name VARCHAR(100) UNIQUE NOT NULL,
    team_tag VARCHAR(5) UNIQUE,
    logo TEXT DEFAULT 'https://placehold.co/200x200/1a1a1a/ffffff?text=TEAM',
    captain UUID, 
    primary_game game_type DEFAULT 'BGMI',
    region VARCHAR(50) DEFAULT 'India',
    country VARCHAR(100),
    bio TEXT DEFAULT '',
    established_date TIMESTAMPTZ DEFAULT NOW(),
    total_earnings DECIMAL(15,2) DEFAULT 0,
    aegis_rating INTEGER DEFAULT 0,
    organization_id UUID,
    discord VARCHAR(255) DEFAULT '',
    twitter VARCHAR(255) DEFAULT '',
    twitch VARCHAR(255) DEFAULT '',
    youtube VARCHAR(255) DEFAULT '',
    website VARCHAR(255) DEFAULT '',
    profile_visibility VARCHAR(20) DEFAULT 'public',
    status team_status DEFAULT 'active',
    looking_for_players BOOLEAN DEFAULT FALSE,
    open_roles TEXT[],
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Organizations (UNCHANGED)
CREATE TABLE organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_name VARCHAR(200) UNIQUE NOT NULL,
    owner_name VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    country VARCHAR(100) NOT NULL,
    headquarters VARCHAR(200),
    description TEXT DEFAULT '',
    logo TEXT DEFAULT '',
    established_date TIMESTAMPTZ DEFAULT NOW(),
    active_games game_type[] DEFAULT '{}',
    total_earnings DECIMAL(15,2) DEFAULT 0,
    contact_phone VARCHAR(20) DEFAULT '',
    discord VARCHAR(255) DEFAULT '',
    twitter VARCHAR(255) DEFAULT '',
    twitch VARCHAR(255) DEFAULT '',
    youtube VARCHAR(255) DEFAULT '',
    website VARCHAR(255) DEFAULT '',
    linkedin VARCHAR(255) DEFAULT '',
    profile_visibility VARCHAR(20) DEFAULT 'public',
    approval_status approval_status DEFAULT 'pending',
    approved_by UUID,
    approval_date TIMESTAMPTZ,
    rejection_reason TEXT,
    email_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Admins (UNCHANGED)
CREATE TABLE admins (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    role admin_role DEFAULT 'admin',
    permissions JSONB DEFAULT '{}',
    is_active BOOLEAN DEFAULT TRUE,
    last_login TIMESTAMPTZ,
    login_attempts INTEGER DEFAULT 0,
    lock_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 🚀 ENTERPRISE AUTH: User Sessions (Stripe/GitHub/AWS Pattern)
CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    session_token VARCHAR(255) UNIQUE NOT NULL,
    refresh_token VARCHAR(255) UNIQUE NOT NULL,
    user_type VARCHAR(20) NOT NULL CHECK (user_type IN ('player', 'admin', 'organization')),
    ip_address VARCHAR(45),
    user_agent TEXT,
    device_fingerprint VARCHAR(255),
    expires_at TIMESTAMPTZ NOT NULL,
    revoked BOOLEAN DEFAULT FALSE,
    revoked_at TIMESTAMPTZ,
    revoked_reason VARCHAR(100),
    last_activity TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 🛡️ ENTERPRISE SECURITY: Audit Trail (SOC2/GDPR Compliance)
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID,
    user_type VARCHAR(20),
    session_id UUID,
    action VARCHAR(50) NOT NULL,
    resource VARCHAR(100),
    resource_id UUID,
    ip_address INET,
    user_agent TEXT,
    success BOOLEAN NOT NULL,
    failure_reason VARCHAR(255),
    request_id VARCHAR(255),
    details JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 🔒 ENTERPRISE SECURITY: Rate Limiting (DDoS Protection)
CREATE TABLE rate_limits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    identifier VARCHAR(255) NOT NULL,
    identifier_type VARCHAR(20) NOT NULL CHECK (identifier_type IN ('ip', 'user_id', 'email')),
    action VARCHAR(50) NOT NULL,
    attempts INTEGER DEFAULT 1,
    window_start TIMESTAMPTZ DEFAULT NOW(),
    blocked_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(identifier, identifier_type, action)
);

-- 🔐 ENTERPRISE SECURITY: API Keys (Service-to-Service Auth)
CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key_id VARCHAR(50) UNIQUE NOT NULL,
    key_hash VARCHAR(255) NOT NULL,
    name VARCHAR(100) NOT NULL,
    owner_id UUID NOT NULL,
    owner_type VARCHAR(20) NOT NULL CHECK (owner_type IN ('admin', 'organization')),
    scopes TEXT[] DEFAULT '{}',
    rate_limit_per_hour INTEGER DEFAULT 1000,
    expires_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tournaments (UNCHANGED - All existing columns preserved)
CREATE TABLE tournaments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament_name VARCHAR(150) UNIQUE NOT NULL,
    short_name VARCHAR(50),
    slug VARCHAR(200) UNIQUE,
    game_title VARCHAR(50) DEFAULT 'BGMI',
    tier VARCHAR(20) DEFAULT 'Community',
    region VARCHAR(50) DEFAULT 'India',
    sub_region VARCHAR(100),
    organizer JSONB DEFAULT '{}',
    sponsors JSONB DEFAULT '[]',
    announcement_date TIMESTAMPTZ,
    is_open_for_all BOOLEAN DEFAULT FALSE,
    registration_start_date TIMESTAMPTZ,
    registration_end_date TIMESTAMPTZ,
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ NOT NULL,
    status tournament_status DEFAULT 'announced',
    format VARCHAR(100),
    format_details TEXT,
    slots JSONB DEFAULT '{}',
    participating_teams JSONB DEFAULT '[]',
    phases JSONB DEFAULT '[]',
    final_standings JSONB DEFAULT '[]',
    prize_pool JSONB DEFAULT '{}',
    statistics JSONB DEFAULT '{}',
    awards JSONB DEFAULT '[]',
    media JSONB DEFAULT '{}',
    stream_links JSONB DEFAULT '[]',
    social_media JSONB DEFAULT '{}',
    description TEXT,
    ruleset_document TEXT,
    website_link TEXT,
    game_settings JSONB DEFAULT '{}',
    visibility VARCHAR(20) DEFAULT 'public',
    featured BOOLEAN DEFAULT FALSE,
    verified BOOLEAN DEFAULT FALSE,
    parent_series UUID,
    qualifies_for JSONB DEFAULT '[]',
    tags TEXT[],
    notes TEXT,
    external_ids JSONB DEFAULT '{}',
    approval_status approval_status DEFAULT 'not_applicable',
    submitted_by UUID,
    submitted_at TIMESTAMPTZ,
    approved_by UUID,
    approved_at TIMESTAMPTZ,
    rejected_by UUID,
    rejected_at TIMESTAMPTZ,
    rejection_reason TEXT,
    pending_invitations JSONB DEFAULT '[]',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tournament Teams (UNCHANGED)
CREATE TABLE tournament_teams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament_id UUID REFERENCES tournaments(id) ON DELETE CASCADE,
    team_id UUID REFERENCES teams(id) ON DELETE CASCADE,
    qualified_through VARCHAR(50),
    current_stage VARCHAR(100),
    total_tournament_points INTEGER DEFAULT 0,
    total_tournament_kills INTEGER DEFAULT 0,
    final_placement INTEGER,
    prize_amount DECIMAL(15,2),
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(tournament_id, team_id)
);

-- Battles (UNCHANGED)
CREATE TABLE battles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    battle_number INTEGER NOT NULL,
    tournament UUID REFERENCES tournaments(id) ON DELETE CASCADE,
    tournament_phase VARCHAR(200),
    scheduled_start_time TIMESTAMPTZ NOT NULL,
    status battle_status DEFAULT 'scheduled',
    map VARCHAR(50),
    participating_groups TEXT[],
    participating_teams JSONB DEFAULT '[]',
    battle_stats JSONB DEFAULT '{}',
    stream_urls JSONB DEFAULT '[]',
    room_credentials JSONB DEFAULT '{}',
    points_system JSONB DEFAULT '{}',
    tags TEXT[],
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tournament Team Invites (UNCHANGED)
CREATE TABLE tournament_team_invites (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament UUID REFERENCES tournaments(id) ON DELETE CASCADE,
    team UUID REFERENCES teams(id) ON DELETE CASCADE,
    phase VARCHAR(200) NOT NULL,
    organizer UUID REFERENCES organizations(id) ON DELETE CASCADE,
    status VARCHAR(20) DEFAULT 'pending',
    message TEXT,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Rewards (UNCHANGED)
CREATE TABLE rewards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reward_name VARCHAR(100) NOT NULL,
    reward_type VARCHAR(50) NOT NULL,
    description TEXT,
    value DECIMAL(10,2),
    currency VARCHAR(10),
    requirements JSONB DEFAULT '{}',
    availability_start TIMESTAMPTZ,
    availability_end TIMESTAMPTZ,
    max_claims INTEGER,
    current_claims INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Transactions (UNCHANGED)
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    transaction_type VARCHAR(50) NOT NULL,
    amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(10) DEFAULT 'USD',
    description TEXT,
    reference_id VARCHAR(255),
    status VARCHAR(20) DEFAULT 'pending',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 🚀 ENTERPRISE PERFORMANCE: Optimized Indexes
CREATE INDEX idx_players_email ON players(email);
CREATE INDEX idx_players_username ON players(username);
CREATE INDEX idx_players_verified ON players(verified);
CREATE INDEX idx_players_team_id ON players(team_id);

CREATE INDEX idx_organizations_email ON organizations(email);
CREATE INDEX idx_organizations_approval_status ON organizations(approval_status);

CREATE INDEX idx_admins_email ON admins(email);
CREATE INDEX idx_admins_is_active ON admins(is_active);

-- Enterprise Auth Indexes (Critical for performance)
CREATE INDEX idx_user_sessions_active ON user_sessions(user_id, user_type) WHERE NOT revoked AND expires_at > NOW();
CREATE INDEX idx_user_sessions_token ON user_sessions(session_token) WHERE NOT revoked;
CREATE INDEX idx_user_sessions_refresh ON user_sessions(refresh_token) WHERE NOT revoked;
CREATE INDEX idx_user_sessions_cleanup ON user_sessions(expires_at) WHERE NOT revoked;
CREATE INDEX idx_user_sessions_activity ON user_sessions(last_activity DESC);

CREATE INDEX idx_audit_logs_user ON audit_logs(user_id, user_type, created_at DESC);
CREATE INDEX idx_audit_logs_action ON audit_logs(action, created_at DESC);
CREATE INDEX idx_audit_logs_session ON audit_logs(session_id);

CREATE INDEX idx_rate_limits_active ON rate_limits(identifier, identifier_type, action) WHERE blocked_until > NOW();
CREATE INDEX idx_rate_limits_cleanup ON rate_limits(window_start) WHERE blocked_until IS NULL OR blocked_until < NOW();

CREATE INDEX idx_api_keys_lookup ON api_keys(key_id) WHERE is_active;
CREATE INDEX idx_api_keys_owner ON api_keys(owner_id, owner_type) WHERE is_active;

CREATE INDEX idx_tournaments_status ON tournaments(status);
CREATE INDEX idx_tournaments_game_title ON tournaments(game_title);
CREATE INDEX idx_tournaments_start_date ON tournaments(start_date);

CREATE INDEX idx_tournament_teams_tournament_id ON tournament_teams(tournament_id);
CREATE INDEX idx_tournament_teams_team_id ON tournament_teams(team_id);

CREATE INDEX idx_battles_tournament ON battles(tournament);
CREATE INDEX idx_battles_status ON battles(status);

CREATE INDEX idx_transactions_player_id ON transactions(player_id);
CREATE INDEX idx_transactions_status ON transactions(status);

-- 🔗 ENTERPRISE CONSTRAINTS: Data Integrity
ALTER TABLE teams ADD CONSTRAINT fk_teams_captain FOREIGN KEY (captain) REFERENCES players(id);
ALTER TABLE teams ADD CONSTRAINT fk_teams_organization FOREIGN KEY (organization_id) REFERENCES organizations(id);
ALTER TABLE players ADD CONSTRAINT fk_players_team FOREIGN KEY (team_id) REFERENCES teams(id);
ALTER TABLE organizations ADD CONSTRAINT fk_organizations_approved_by FOREIGN KEY (approved_by) REFERENCES admins(id);

-- 🤖 ENTERPRISE AUTOMATION: Auto-cleanup Functions
CREATE OR REPLACE FUNCTION cleanup_expired_sessions()
RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM user_sessions WHERE expires_at < NOW() - INTERVAL '30 days';
    DELETE FROM rate_limits WHERE window_start < NOW() - INTERVAL '24 hours' AND (blocked_until IS NULL OR blocked_until < NOW());
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_cleanup_auth_data
    AFTER INSERT ON user_sessions
    EXECUTE FUNCTION cleanup_expired_sessions();

-- 📊 ENTERPRISE MONITORING: Performance Views
CREATE VIEW active_sessions AS
SELECT 
    user_id, user_type, COUNT(*) as session_count,
    MAX(last_activity) as last_seen,
    MIN(created_at) as first_session
FROM user_sessions 
WHERE NOT revoked AND expires_at > NOW()
GROUP BY user_id, user_type;

CREATE VIEW security_metrics AS
SELECT 
    DATE(created_at) as date,
    action,
    COUNT(*) as total_attempts,
    COUNT(*) FILTER (WHERE success) as successful,
    COUNT(*) FILTER (WHERE NOT success) as failed
FROM audit_logs 
WHERE created_at > NOW() - INTERVAL '30 days'
GROUP BY DATE(created_at), action
ORDER BY date DESC, action;
