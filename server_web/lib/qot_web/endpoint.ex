defmodule QotWeb.Endpoint do
  use Phoenix.Endpoint, otp_app: :qot

  plug(CORSPlug, origin: Application.compile_env(:qot, :cors_origins, []))

  plug(Plug.Parsers,
    parsers: [:json],
    pass: ["*/*"],
    json_decoder: Jason
  )

  plug(QotWeb.Router)
end
