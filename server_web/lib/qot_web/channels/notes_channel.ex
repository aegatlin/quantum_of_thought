defmodule QotWeb.NotesChannel do
  use Phoenix.Channel
  alias Qot.Notes

  @impl true
  def join("notes:lobby", _message, socket) do
    # Subscribe to PubSub topic to receive events from Notes context
    Phoenix.PubSub.subscribe(Qot.PubSub, "notes:lobby")

    # Send all existing notes to newly connected client
    send(self(), :after_join)
    {:ok, socket}
  end

  @impl true
  def handle_info(:after_join, socket) do
    {:ok, notes} = Notes.list()

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

  # Handle PubSub broadcast from Notes context (triggered by HTTP or any source)
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

  # Handle incoming "note" message from WebSocket client
  @impl true
  def handle_in("message", %{"type" => "note", "id" => id, "data" => data}, socket) do
    case Base.decode64(data) do
      {:ok, binary_data} ->
        # This will trigger PubSub broadcast to all clients
        {:ok, _note} = Notes.set(id, binary_data)
        {:noreply, socket}

      :error ->
        {:reply, {:error, %{reason: "Invalid base64"}}, socket}
    end
  end

  # Handle incoming "delete" message from WebSocket client
  @impl true
  def handle_in("message", %{"type" => "delete", "id" => id}, socket) do
    # This will trigger PubSub broadcast to all clients
    :ok = Notes.delete(id)
    {:noreply, socket}
  end
end
