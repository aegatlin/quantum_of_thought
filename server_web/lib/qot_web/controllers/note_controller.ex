defmodule QotWeb.NoteController do
  use QotWeb, :controller
  alias Qot.Notes

  def upsert(conn, %{"id" => id, "data" => data}) do
    user_id = conn.assigns.user_id

    with {:ok, binary_data} <- Base.decode64(data),
         {:ok, _note} <- Notes.set(user_id, id, binary_data) do
      json(conn, %{id: id})
    else
      :error ->
        conn
        |> put_status(:bad_request)
        |> json(%{error: "Invalid base64 data"})

      {:error, reason} ->
        conn
        |> put_status(:internal_server_error)
        |> json(%{error: inspect(reason)})
    end
  end

  def index(conn, _params) do
    user_id = conn.assigns.user_id
    {:ok, notes} = Notes.list(user_id)

    notes_json =
      Enum.map(notes, fn note ->
        %{
          id: note.id,
          data: Base.encode64(note.data)
        }
      end)

    json(conn, %{notes: notes_json})
  end

  def delete(conn, %{"id" => id}) do
    user_id = conn.assigns.user_id
    :ok = Notes.delete(user_id, id)
    json(conn, %{deleted: true})
  end
end
