import Config

if config_env() == :prod do
  database_url =
    System.get_env("DATABASE_URL") ||
      raise """
      environment variable DATABASE_URL is missing.
      For example: ecto://USER:PASS@HOST/DATABASE
      """

  config :qot, Qot.Repo,
    url: database_url,
    pool_size: String.to_integer(System.get_env("POOL_SIZE") || "10"),
    ssl: true

  jwt_secret_key =
    System.get_env("JWT_SECRET_KEY") ||
      raise """
      environment variable JWT_SECRET_KEY is missing.
      Generate a secure random string for production.
      """

  config :joken, default_signer: jwt_secret_key

  resend_api_key =
    System.get_env("RESEND_API_KEY") ||
      raise """
      environment variable RESEND_API_KEY is missing.
      Get your API key from https://resend.com
      """

  config :qot, Qot.Accounts.Mailer, api_key: resend_api_key

  # Frontend URL for magic links
  frontend_url =
    System.get_env("FRONTEND_URL") ||
      raise """
      environment variable FRONTEND_URL is missing.
      Example: https://app.example.com
      """

  config :qot, :frontend_url, frontend_url

  secret_key_base =
    System.get_env("SECRET_KEY_BASE") ||
      raise """
      environment variable SECRET_KEY_BASE is missing.
      You can generate one by calling: mix phx.gen.secret
      """

  host = System.get_env("PHX_HOST") || "example.com"
  port = String.to_integer(System.get_env("PORT") || "4000")

  config :qot, QotWeb.Endpoint,
    url: [host: host, port: 443, scheme: "https"],
    http: [
      ip: {0, 0, 0, 0, 0, 0, 0, 0},
      port: port
    ],
    secret_key_base: secret_key_base
end
