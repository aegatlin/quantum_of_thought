defmodule QotWeb.ChannelCase do
  use ExUnit.CaseTemplate

  using do
    quote do
      @endpoint QotWeb.Endpoint

      import Phoenix.ChannelTest
      import QotWeb.ChannelCase
    end
  end

  setup tags do
    Qot.DataCase.setup_sandbox(tags)
    :ok
  end
end
