defmodule Qot.NotesTest do
  use ExUnit.Case, async: false

  alias Qot.Notes

  @user_id "test-user-id"

  setup do
    :ets.delete_all_objects(:qot_notes)
    :ok
  end

  describe "set/3" do
    test "creates a note successfully" do
      {:ok, note} = Notes.set(@user_id, "abc-123", "test content")

      assert note.id == "abc-123"
      assert note.data == "test content"
      assert note.user_id == @user_id
    end

    test "updates an existing note" do
      {:ok, _} = Notes.set(@user_id, "abc-123", "original")
      {:ok, note} = Notes.set(@user_id, "abc-123", "updated")

      assert note.data == "updated"
    end
  end

  describe "list/1" do
    test "returns empty list initially" do
      {:ok, notes} = Notes.list(@user_id)
      assert notes == []
    end

    test "returns all notes for a user" do
      Notes.set(@user_id, "id-1", "data-1")
      Notes.set(@user_id, "id-2", "data-2")
      # Different user's note should not appear
      Notes.set("other-user", "id-3", "data-3")

      {:ok, notes} = Notes.list(@user_id)

      assert length(notes) == 2
    end
  end

  describe "delete/2" do
    test "deletes a note" do
      Notes.set(@user_id, "abc-123", "data")
      :ok = Notes.delete(@user_id, "abc-123")

      {:ok, notes} = Notes.list(@user_id)
      assert notes == []
    end
  end
end
