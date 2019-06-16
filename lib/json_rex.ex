defmodule JsonRex do
  use Rustler, otp_app: :jrex, crate: :jrex

  @moduledoc """
  Documentation for Jrex
  """

  def encode(_data), do: :erlang.nif_error("nif not loaded")
  def decode(_data), do: :erlang.nif_error("nif not loaded")
end
