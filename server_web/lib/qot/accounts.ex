defmodule Qot.Accounts do
  @moduledoc """
  The Accounts context - handles user authentication and token management.
  """

  import Ecto.Query, warn: false
  alias Qot.Repo
  alias Qot.Accounts.{User, Token, Mailer}

  ## User functions

  @doc """
  Gets a user by ID.
  Returns {:ok, user} or {:error, :not_found}
  """
  def get_user(id) do
    case Repo.get(User, id) do
      nil -> {:error, :not_found}
      user -> {:ok, user}
    end
  end

  @doc """
  Gets a user by email.
  Returns {:ok, user} or {:error, :not_found}
  """
  def get_user_by_email(email) do
    case Repo.get_by(User, email: email) do
      nil -> {:error, :not_found}
      user -> {:ok, user}
    end
  end

  @doc """
  Creates a new user with the given email.
  Returns {:ok, user} or {:error, changeset}
  """
  def create_user(attrs) do
    %User{}
    |> User.changeset(attrs)
    |> Repo.insert()
  end

  @doc """
  Gets or creates a user by email.
  Returns {:ok, user}
  """
  def get_or_create_user(email) do
    case get_user_by_email(email) do
      {:ok, user} -> {:ok, user}
      {:error, :not_found} -> create_user(%{email: email})
    end
  end

  ## Magic Link functions

  @doc """
  Creates a magic link token and sends it via email.
  Returns :ok or {:error, reason}
  """
  def create_magic_link(email) do
    # Generate token
    token = Token.generate_magic_link_token()
    hashed_token = Token.hash_token(token)
    expires_at = Token.magic_link_expires_at()

    # Store hashed token in database
    query = """
    INSERT INTO magic_link_tokens (token, email, expires_at, inserted_at)
    VALUES ($1, $2, $3, $4)
    """

    case Repo.query(query, [hashed_token, email, expires_at, DateTime.utc_now()]) do
      {:ok, _} ->
        # Send email with the unhashed token
        Mailer.send_magic_link(email, token)
        :ok

      {:error, reason} ->
        {:error, reason}
    end
  end

  @doc """
  Verifies a magic link token and returns JWT + refresh token.
  Returns {:ok, %{access_token: jwt, refresh_token: token, user: user}} or {:error, reason}
  """
  def verify_magic_link(token) do
    hashed_token = Token.hash_token(token)

    # Find and delete the magic link token (one-time use)
    query = """
    DELETE FROM magic_link_tokens
    WHERE token = $1 AND expires_at > $2
    RETURNING email
    """

    case Repo.query(query, [hashed_token, DateTime.utc_now()]) do
      {:ok, %{rows: [[email]]}} ->
        # Token is valid, get or create user
        {:ok, user} = get_or_create_user(email)

        # Generate tokens
        {:ok, access_token} = Token.generate_jwt(user.id)
        refresh_token = Token.generate_refresh_token()

        # Store refresh token
        {:ok, _} = create_refresh_token(user.id, refresh_token)

        {:ok, %{access_token: access_token, refresh_token: refresh_token, user: user}}

      {:ok, %{rows: []}} ->
        {:error, :invalid_or_expired_token}

      {:error, reason} ->
        {:error, reason}
    end
  end

  ## Refresh Token functions

  # Creates a refresh token for a user.
  # Returns {:ok, refresh_token} or {:error, reason}
  defp create_refresh_token(user_id, token) do
    hashed_token = Token.hash_token(token)
    expires_at = Token.refresh_token_expires_at()

    query = """
    INSERT INTO refresh_tokens (token, user_id, expires_at, inserted_at)
    VALUES ($1, $2, $3, $4)
    """

    case Repo.query(query, [hashed_token, user_id, expires_at, DateTime.utc_now()]) do
      {:ok, _} -> {:ok, token}
      {:error, reason} -> {:error, reason}
    end
  end

  @doc """
  Refreshes an access token using a refresh token.
  Returns {:ok, %{access_token: jwt}} or {:error, reason}
  """
  def refresh_access_token(refresh_token) do
    hashed_token = Token.hash_token(refresh_token)

    # Find valid refresh token and get user_id
    query = """
    SELECT user_id FROM refresh_tokens
    WHERE token = $1 AND expires_at > $2
    """

    case Repo.query(query, [hashed_token, DateTime.utc_now()]) do
      {:ok, %{rows: [[user_id]]}} ->
        {:ok, access_token} = Token.generate_jwt(user_id)
        {:ok, %{access_token: access_token}}

      {:ok, %{rows: []}} ->
        {:error, :invalid_or_expired_token}

      {:error, reason} ->
        {:error, reason}
    end
  end

  @doc """
  Revokes a refresh token (logout).
  Returns :ok or {:error, reason}
  """
  def revoke_refresh_token(refresh_token) do
    hashed_token = Token.hash_token(refresh_token)

    query = """
    DELETE FROM refresh_tokens WHERE token = $1
    """

    case Repo.query(query, [hashed_token]) do
      {:ok, _} -> :ok
      {:error, reason} -> {:error, reason}
    end
  end

  @doc """
  Cleans up expired tokens (magic links and refresh tokens).
  Should be run periodically.
  """
  def cleanup_expired_tokens do
    now = DateTime.utc_now()

    Repo.query("DELETE FROM magic_link_tokens WHERE expires_at < $1", [now])
    Repo.query("DELETE FROM refresh_tokens WHERE expires_at < $1", [now])

    :ok
  end
end
