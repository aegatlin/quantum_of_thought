defmodule QotWeb.Router do
  use Phoenix.Router

  pipeline :api do
    plug(:accepts, ["json"])
  end

  scope "/api", QotWeb do
    pipe_through(:api)

    get("/notes", NoteController, :index)
    put("/notes/:id", NoteController, :upsert)
    delete("/notes/:id", NoteController, :delete)
  end
end
