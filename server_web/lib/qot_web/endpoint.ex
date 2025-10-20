defmodule QotWeb.Endpoint do
  use Phoenix.Endpoint, otp_app: :qot

  plug(Plug.Parsers,
    parsers: [:json],
    pass: ["*/*"],
    json_decoder: Jason
  )

  plug(QotWeb.Router)
end
