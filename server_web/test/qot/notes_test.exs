defmodule Qot.NotesTest do
  use ExUnit.Case, async: false

  alias Qot.Notes

  setup do
    :ets.delete_all_objects(:qot_notes)
    :ok
  end

  describe "set/2" do
    test "creates a note successfully" do
      {:ok, note} = Notes.set("abc-123", "test content")

      assert note.id == "abc-123"
      assert note.data == "test content"
    end

    test "updates an existing note" do
      {:ok, _} = Notes.set("abc-123", "original")
      {:ok, note} = Notes.set("abc-123", "updated")

      assert note.data == "updated"
    end
  end

  describe "list/0" do
    test "returns empty list initially" do
      {:ok, notes} = Notes.list()
      assert notes == []
    end

    test "returns all notes" do
      Notes.set("id-1", "data-1")
      Notes.set("id-2", "data-2")

      {:ok, notes} = Notes.list()

      assert length(notes) == 2
    end
  end

  describe "delete/1" do
    test "deletes a note" do
      Notes.set("abc-123", "data")
      :ok = Notes.delete("abc-123")

      {:ok, notes} = Notes.list()
      assert notes == []
    end
  end
end
