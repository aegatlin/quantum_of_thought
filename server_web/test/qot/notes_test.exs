defmodule Qot.NotesTest do
  use ExUnit.Case, async: false

  alias Qot.Notes

  setup do
    :ets.delete_all_objects(:qot_notes)
    :ok
  end

  describe "put_note/2" do
    test "creates a note successfully" do
      {:ok, note} = Notes.put_note("abc-123", "test content")

      assert note.id == "abc-123"
      assert note.data == "test content"
    end

    test "updates an existing note" do
      {:ok, _} = Notes.put_note("abc-123", "original")
      {:ok, note} = Notes.put_note("abc-123", "updated")

      assert note.data == "updated"
    end
  end

  describe "list_notes/0" do
    test "returns empty list initially" do
      {:ok, notes} = Notes.list_notes()
      assert notes == []
    end

    test "returns all notes" do
      Notes.put_note("id-1", "data-1")
      Notes.put_note("id-2", "data-2")

      {:ok, notes} = Notes.list_notes()

      assert length(notes) == 2
    end
  end

  describe "delete_note/1" do
    test "deletes a note" do
      Notes.put_note("abc-123", "data")
      :ok = Notes.delete_note("abc-123")

      {:ok, notes} = Notes.list_notes()
      assert notes == []
    end
  end
end
