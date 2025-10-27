defmodule QotWeb.NotesChannel do
  use Phoenix.Channel
  alias Qot.Notes

  @impl true
  def join("notes:user:" <> user_id, _message, socket) do
    if socket.assigns.user_id == user_id do
      Phoenix.PubSub.subscribe(Qot.PubSub, "notes:user:#{user_id}")

      send(self(), :after_join)
      {:ok, socket}
    else
      {:error, %{reason: "unauthorized"}}
    end
  end

  @impl true
  def handle_info(:after_join, socket) do
    user_id = socket.assigns.user_id
    {:ok, notes} = Notes.list(user_id)

    notes_payload = %{
      type: "notes",
      notes:
        Enum.map(notes, fn note ->
          %{id: note.id, data: Base.encode64(note.data)}
        end)
    }

    push(socket, "message", notes_payload)
    {:noreply, socket}
  end

  @impl true
  def handle_info({:note_created, note}, socket) do
    push(socket, "message", %{
      type: "note",
      id: note.id,
      data: Base.encode64(note.data)
    })

    {:noreply, socket}
  end

  @impl true
  def handle_info({:note_deleted, id}, socket) do
    push(socket, "message", %{
      type: "delete",
      id: id
    })

    {:noreply, socket}
  end

  @impl true
  def handle_in("message", %{"type" => "note", "id" => id, "data" => data}, socket) do
    user_id = socket.assigns.user_id

    case Base.decode64(data) do
      {:ok, binary_data} ->
        # This will trigger PubSub broadcast to all user's clients
        {:ok, _note} = Notes.set(user_id, id, binary_data)
        {:noreply, socket}

      :error ->
        {:reply, {:error, %{reason: "Invalid base64"}}, socket}
    end
  end

  @impl true
  def handle_in("message", %{"type" => "delete", "id" => id}, socket) do
    user_id = socket.assigns.user_id
    # This will trigger PubSub broadcast to all user's clients
    :ok = Notes.delete(user_id, id)
    {:noreply, socket}
  end
end
