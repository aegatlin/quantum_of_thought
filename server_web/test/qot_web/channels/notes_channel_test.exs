defmodule QotWeb.NotesChannelTest do
  use QotWeb.ChannelCase
  alias QotWeb.{UserSocket, NotesChannel}

  setup do
    {:ok, _, socket} =
      UserSocket
      |> socket("user_id", %{})
      |> subscribe_and_join(NotesChannel, "notes:lobby")

    %{socket: socket}
  end

  test "sends all notes on join", %{socket: _socket} do
    # Should receive initial notes message
    assert_push("message", %{type: "notes", notes: _notes})
  end

  test "broadcasts note from WebSocket client to other clients", %{socket: socket} do
    note_data = Base.encode64("test note data")

    push(socket, "message", %{
      type: "note",
      id: "test-id",
      data: note_data
    })

    # Should receive the note back via PubSub
    assert_push("message", %{
      type: "note",
      id: "test-id",
      data: ^note_data
    })
  end

  test "broadcasts delete from WebSocket client to other clients", %{socket: socket} do
    # First create a note
    note_data = Base.encode64("test note data")
    push(socket, "message", %{type: "note", id: "test-id", data: note_data})
    assert_push("message", %{type: "note"})

    # Then delete it
    push(socket, "message", %{type: "delete", id: "test-id"})

    # Should receive delete via PubSub
    assert_push("message", %{
      type: "delete",
      id: "test-id"
    })
  end
end
