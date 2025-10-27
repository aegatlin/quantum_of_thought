defmodule QotWeb.UserSocket do
  use Phoenix.Socket

  alias Qot.Accounts.Token

  # Channel routes
  channel("notes:user:*", QotWeb.NotesChannel)

  @impl true
  def connect(%{"token" => token}, socket, _connect_info) do
    case Token.verify_jwt(token) do
      {:ok, %{"user_id" => user_id}} ->
        {:ok, assign(socket, :user_id, user_id)}

      {:error, _reason} ->
        :error
    end
  end

  def connect(_params, _socket, _connect_info) do
    # Reject connection if no token provided
    :error
  end

  @impl true
  def id(socket), do: "user:#{socket.assigns.user_id}"
end
