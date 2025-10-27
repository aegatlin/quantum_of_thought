defmodule QotWeb.NotesChannelTest do
  use QotWeb.ChannelCase, async: false

  import QotWeb.AuthHelpers

  setup do
    # ETS cleanup (not transactional)
    :ets.delete_all_objects(:qot_notes)

    # Create authenticated user
    auth = create_authenticated_user()

    # Connect to user socket with JWT
    {:ok, socket} = connect(QotWeb.UserSocket, %{"token" => auth.access_token})

    {:ok, socket: socket, auth: auth}
  end

  describe "join notes:user:USER_ID" do
    test "joins channel with valid user_id", %{socket: socket, auth: auth} do
      {:ok, _, socket} = subscribe_and_join(socket, "notes:user:#{auth.user_id}", %{})

      assert socket.assigns.user_id == auth.user_id
    end

    test "rejects join with mismatched user_id", %{socket: socket} do
      assert {:error, %{reason: "unauthorized"}} =
               subscribe_and_join(socket, "notes:user:wrong-user-id", %{})
    end
  end

  describe "message event with type=note" do
    test "broadcasts note update to channel", %{socket: socket, auth: auth} do
      {:ok, _, joined_socket} = subscribe_and_join(socket, "notes:user:#{auth.user_id}", %{})

      note_data = Base.encode64("test note content")
      push(joined_socket, "message", %{"type" => "note", "id" => "test-note-id", "data" => note_data})

      assert_push "message", %{type: "note", id: "test-note-id", data: ^note_data}
    end
  end
end
