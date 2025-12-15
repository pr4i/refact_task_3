-- =====================================================
-- Basic schema
-- =====================================================

CREATE TABLE IF NOT EXISTS iss_fetch_log (
    id BIGSERIAL PRIMARY KEY,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    source_url TEXT NOT NULL,
    payload JSONB NOT NULL
);

CREATE TABLE IF NOT EXISTS telemetry_legacy (
    id BIGSERIAL PRIMARY KEY,
    recorded_at TIMESTAMPTZ NOT NULL,
    voltage NUMERIC(6,2) NOT NULL,
    temp NUMERIC(6,2) NOT NULL,
    source_file TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cms_pages (
    id BIGSERIAL PRIMARY KEY,
    slug TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS osdr_items (
  id BIGSERIAL PRIMARY KEY,
  dataset_id TEXT UNIQUE,
  title TEXT,
  status TEXT,
  updated_at TIMESTAMPTZ,
  inserted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  raw JSONB NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_osdr_items_inserted_at ON osdr_items(inserted_at DESC);


-- =====================================================
-- Cache tables for rust_iss schedulers
-- =====================================================
-- Используются для актуального состояния (last / cache),
-- из них читают REST-эндпойнты (/iss/last и др.)
-- =====================================================

CREATE TABLE IF NOT EXISTS space_cache (
    id BIGSERIAL PRIMARY KEY,
    source TEXT NOT NULL UNIQUE,
    key TEXT,
    payload JSONB NOT NULL,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_space_cache_fetched_at
    ON space_cache (fetched_at DESC);


CREATE INDEX IF NOT EXISTS idx_space_cache_source
    ON space_cache (source);


-- =====================================================
-- Seed data for CMS (XSS practice)
-- =====================================================

INSERT INTO cms_pages(slug, title, body)
VALUES
('welcome', 'Добро пожаловать', '<h3>Демо контент</h3><p>Этот текст хранится в БД</p>'),
('unsafe', 'Небезопасный пример', '<script>console.log("XSS training")</script>
<p>Если вы видите всплывашку значит защита не работает</p>')
ON CONFLICT DO NOTHING;
