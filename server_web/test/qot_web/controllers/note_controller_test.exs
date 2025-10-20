defmodule QotWeb.NoteControllerTest do
  use QotWeb.ConnCase, async: false

  setup do
    :ets.delete_all_objects(:qot_notes)
    :ok
  end

  describe "PUT /api/notes/:id" do
    test "creates a note", %{conn: conn} do
      data = Base.encode64("test note content")

      conn = put(conn, "/api/notes/test-uuid", %{data: data})

      assert %{"id" => "test-uuid"} = json_response(conn, 200)
    end
  end

  describe "GET /api/notes" do
    test "returns all notes", %{conn: conn} do
      put(conn, "/api/notes/uuid-1", %{data: Base.encode64("note 1")})
      put(conn, "/api/notes/uuid-2", %{data: Base.encode64("note 2")})

      conn = get(conn, "/api/notes")

      assert %{"notes" => notes} = json_response(conn, 200)
      assert length(notes) == 2
    end
  end

  describe "DELETE /api/notes/:id" do
    test "deletes a note", %{conn: conn} do
      put(conn, "/api/notes/test-uuid", %{data: Base.encode64("test")})

      conn = delete(conn, "/api/notes/test-uuid")

      assert %{"deleted" => true} = json_response(conn, 200)
    end
  end
end
