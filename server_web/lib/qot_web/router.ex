defmodule QotWeb.Router do
  use Phoenix.Router

  pipeline :api do
    plug(:accepts, ["json"])
  end

  pipeline :authenticated do
    plug(QotWeb.Plugs.RequireAuth)
  end

  scope "/api", QotWeb do
    pipe_through(:api)

    post("/auth/magic-link", AuthController, :request_magic_link)
    get("/auth/verify", AuthController, :verify_magic_link)
    post("/auth/refresh", AuthController, :refresh)
    post("/auth/logout", AuthController, :logout)
  end

  scope "/api", QotWeb do
    pipe_through([:api, :authenticated])

    get("/notes", NoteController, :index)
    put("/notes/:id", NoteController, :upsert)
    delete("/notes/:id", NoteController, :delete)
  end
end
