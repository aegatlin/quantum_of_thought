defmodule QotWeb.NoteControllerTest do
  use QotWeb.ConnCase, async: false

  import QotWeb.AuthHelpers

  setup do
    # ETS cleanup (not transactional, so we need manual cleanup)
    :ets.delete_all_objects(:qot_notes)

    # Create authenticated user for tests (SQL Sandbox handles cleanup)
    auth = create_authenticated_user()
    {:ok, auth: auth}
  end

  describe "PUT /api/notes/:id (authenticated)" do
    test "creates a note with valid auth", %{conn: conn, auth: auth} do
      data = Base.encode64("test note content")

      conn =
        conn
        |> authenticate_conn(auth.access_token)
        |> put("/api/notes/test-uuid", %{data: data})

      assert %{"id" => "test-uuid"} = json_response(conn, 200)
    end

    test "rejects request without auth", %{conn: conn} do
      data = Base.encode64("test note content")
      conn = put(conn, "/api/notes/test-uuid", %{data: data})

      assert json_response(conn, 401)
    end

    test "rejects request with invalid token", %{conn: conn} do
      data = Base.encode64("test note content")

      conn =
        conn
        |> authenticate_conn("invalid.token.here")
        |> put("/api/notes/test-uuid", %{data: data})

      assert json_response(conn, 401)
    end
  end

  describe "GET /api/notes (authenticated)" do
    test "returns user's notes only", %{conn: conn, auth: auth} do
      conn_auth = authenticate_conn(conn, auth.access_token)

      # Create notes for authenticated user
      put(conn_auth, "/api/notes/uuid-1", %{data: Base.encode64("note 1")})
      put(conn_auth, "/api/notes/uuid-2", %{data: Base.encode64("note 2")})

      # Create another user and their notes
      auth2 = create_authenticated_user("user2@example.com")
      conn_auth2 = authenticate_conn(conn, auth2.access_token)
      put(conn_auth2, "/api/notes/uuid-3", %{data: Base.encode64("user2 note")})

      # User 1 should only see their notes
      conn = get(conn_auth, "/api/notes")
      assert %{"notes" => notes} = json_response(conn, 200)
      assert length(notes) == 2
      note_ids = Enum.map(notes, & &1["id"])
      assert "uuid-1" in note_ids
      assert "uuid-2" in note_ids
      refute "uuid-3" in note_ids

      # User 2 should only see their notes
      conn = get(conn_auth2, "/api/notes")
      assert %{"notes" => notes} = json_response(conn, 200)
      assert length(notes) == 1
      assert hd(notes)["id"] == "uuid-3"
    end

    test "rejects request without auth", %{conn: conn} do
      conn = get(conn, "/api/notes")

      assert json_response(conn, 401)
    end
  end

  describe "DELETE /api/notes/:id (authenticated)" do
    test "deletes user's own note", %{conn: conn, auth: auth} do
      conn_auth = authenticate_conn(conn, auth.access_token)

      put(conn_auth, "/api/notes/test-uuid", %{data: Base.encode64("test")})
      conn = delete(conn_auth, "/api/notes/test-uuid")

      assert %{"deleted" => true} = json_response(conn, 200)
    end

    test "rejects request without auth", %{conn: conn} do
      conn = delete(conn, "/api/notes/test-uuid")

      assert json_response(conn, 401)
    end
  end
end
