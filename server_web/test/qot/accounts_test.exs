defmodule Qot.AccountsTest do
  use Qot.DataCase, async: true

  alias Qot.Accounts

  describe "create_user/1" do
    test "creates a user with valid email" do
      assert {:ok, user} = Accounts.create_user(%{email: "test@example.com"})
      assert user.email == "test@example.com"
      assert user.id != nil
    end

    test "fails with invalid email" do
      assert {:error, changeset} = Accounts.create_user(%{email: "invalid"})
      assert changeset.errors[:email] != nil
    end

    test "fails with duplicate email" do
      {:ok, _user} = Accounts.create_user(%{email: "test@example.com"})
      assert {:error, changeset} = Accounts.create_user(%{email: "test@example.com"})
      assert changeset.errors[:email] != nil
    end
  end

  describe "get_or_create_user/1" do
    test "creates new user if doesn't exist" do
      assert {:ok, user} = Accounts.get_or_create_user("new@example.com")
      assert user.email == "new@example.com"
    end

    test "returns existing user if already exists" do
      {:ok, user1} = Accounts.create_user(%{email: "existing@example.com"})
      {:ok, user2} = Accounts.get_or_create_user("existing@example.com")

      assert user1.id == user2.id
    end
  end

  describe "Token.generate_jwt/1" do
    test "generates valid JWT token" do
      {:ok, user} = Accounts.create_user(%{email: "test@example.com"})

      {:ok, token, _claims} = Accounts.Token.generate_jwt(user.id)
      assert is_binary(token)
    end
  end

  describe "Token.verify_jwt/1" do
    test "verifies valid JWT token" do
      {:ok, user} = Accounts.create_user(%{email: "test@example.com"})
      {:ok, token, _claims} = Accounts.Token.generate_jwt(user.id)

      {:ok, claims} = Accounts.Token.verify_jwt(token)
      assert claims["user_id"] == user.id
    end

    test "rejects invalid JWT" do
      assert {:error, _} = Accounts.Token.verify_jwt("invalid.token.here")
    end
  end

  describe "create_magic_link/1" do
    test "creates magic link and sends email" do
      email = "test@example.com"
      assert :ok = Accounts.create_magic_link(email)
    end
  end

  describe "verify_magic_link/1" do
    test "rejects invalid token" do
      assert {:error, :invalid_or_expired_token} = Accounts.verify_magic_link("invalid_token")
    end
  end
end
