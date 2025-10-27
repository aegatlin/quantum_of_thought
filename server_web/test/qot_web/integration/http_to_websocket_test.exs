defmodule QotWeb.Integration.HttpToWebsocketTest do
  use QotWeb.ConnCase, async: false
  import Phoenix.ChannelTest
  import QotWeb.AuthHelpers

  setup do
    # ETS cleanup
    :ets.delete_all_objects(:qot_notes)

    # Create authenticated user
    auth = create_authenticated_user()

    # Setup HTTP conn
    conn = build_conn() |> authenticate_conn(auth.access_token)

    # Setup WebSocket
    {:ok, socket} = Phoenix.ChannelTest.connect(QotWeb.UserSocket, %{"token" => auth.access_token})
    {:ok, _, ws_socket} = subscribe_and_join(socket, "notes:user:#{auth.user_id}", %{})

    {:ok, conn: conn, auth: auth, ws_socket: ws_socket}
  end

  describe "HTTP to WebSocket integration" do
    test "note created via HTTP is broadcast to WebSocket", %{conn: conn, ws_socket: _ws_socket} do
      # Create note via HTTP
      note_data = Base.encode64("integration test note")
      conn = put(conn, "/api/notes/integration-note-id", %{data: note_data})

      assert json_response(conn, 200)

      # Verify broadcast on WebSocket
      assert_push "message", %{type: "note", id: "integration-note-id", data: ^note_data}
    end
  end
end
