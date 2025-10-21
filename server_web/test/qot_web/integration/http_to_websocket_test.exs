defmodule QotWeb.Integration.HttpToWebsocketTest do
  use QotWeb.ConnCase
  use QotWeb.ChannelCase

  alias QotWeb.{UserSocket, NotesChannel}

  describe "HTTP to WebSocket sync" do
    setup do
      # Connect a WebSocket client
      {:ok, _, socket} =
        UserSocket
        |> socket("user_id", %{})
        |> subscribe_and_join(NotesChannel, "notes:lobby")

      # Clear initial "notes" message
      assert_push("message", %{type: "notes"})

      %{socket: socket}
    end

    test "creating note via HTTP REST endpoint notifies WebSocket clients", %{socket: _socket} do
      # Step 1: CLI/HTTP client creates a note via REST API
      note_id = "http-created-note"
      note_content = "Hello from HTTP!"
      note_data = Base.encode64(note_content)

      conn = build_conn()
      conn = put(conn, "/api/notes/#{note_id}", %{id: note_id, data: note_data})

      # Verify HTTP response
      assert json_response(conn, 200) == %{"id" => note_id}

      # Step 2: Verify WebSocket client received the notification
      assert_push(
        "message",
        %{
          type: "note",
          id: ^note_id,
          data: ^note_data
        },
        1000
      )
    end
  end
end
