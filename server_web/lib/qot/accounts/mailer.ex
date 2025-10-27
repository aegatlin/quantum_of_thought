defmodule Qot.Accounts.Mailer do
  @moduledoc """
  Email functionality for authentication (magic links).
  """

  import Swoosh.Email

  @doc """
  Sends a magic link email to the given email address.
  """
  def send_magic_link(email, token) do
    magic_link_url = build_magic_link_url(token)

    email_content =
      new()
      |> to(email)
      |> from({"Quantum of Thought", "noreply@quantumofthought.com"})
      |> subject("Your login link for Quantum of Thought")
      |> html_body("""
      <html>
        <body>
          <h2>Login to Quantum of Thought</h2>
          <p>Click the link below to log in to your account:</p>
          <p><a href="#{magic_link_url}">Log in to Quantum of Thought</a></p>
          <p>This link will expire in 15 minutes.</p>
          <p>If you didn't request this email, you can safely ignore it.</p>
        </body>
      </html>
      """)
      |> text_body("""
      Login to Quantum of Thought

      Click the link below to log in to your account:
      #{magic_link_url}

      This link will expire in 15 minutes.

      If you didn't request this email, you can safely ignore it.
      """)

    deliver(email_content)
  end

  defp build_magic_link_url(token) do
    frontend_url = System.get_env("FRONTEND_URL") || "http://localhost:5173"
    "#{frontend_url}/auth/verify?token=#{token}"
  end

  defp deliver(email) do
    # In development, Swoosh.Adapters.Local will capture emails
    # You can view them at http://localhost:4000/dev/mailbox (if you add the route)
    # In production, this will use the configured adapter (e.g., Resend)
    Swoosh.deliver(email, Qot.Accounts.Mailer)
  end
end
