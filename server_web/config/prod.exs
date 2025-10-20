import Config

config :qot, :cors_origins, []

config :qot, QotWeb.Endpoint, cache_static_manifest: "priv/static/cache_manifest.json"

config :logger, level: :info
