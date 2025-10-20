import Config

config :qot,
  storage_adapter: Qot.Storage.ETSAdapter

config :qot, QotWeb.Endpoint,
  url: [host: "localhost"],
  adapter: Bandit.PhoenixAdapter,
  render_errors: [formats: [json: QotWeb.ErrorJSON], layout: false],
  pubsub_server: Qot.PubSub

import_config "#{config_env()}.exs"
