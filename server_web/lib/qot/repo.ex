defmodule Qot.Repo do
  use Ecto.Repo,
    otp_app: :qot,
    adapter: Ecto.Adapters.Postgres
end
