defmodule QotWeb.UserSocket do
  use Phoenix.Socket

  # Channel routes
  channel("notes:*", QotWeb.NotesChannel)

  @impl true
  def connect(_params, socket, _connect_info) do
    # No authentication for now
    {:ok, socket}
  end

  @impl true
  def id(_socket), do: nil
end
