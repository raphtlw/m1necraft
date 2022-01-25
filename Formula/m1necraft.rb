class M1necraft < Formula
  desc "Patch Minecraft launcher to run Minecraft natively on ARM."
  homepage "https://m1necraft.vercel.app"
  url "https://github.com/raphtlw/m1necraft/tarball/v0.1.0"
  sha256 "4dc57244ac7971c0af5da30b5787d7c20803393decb955d8af915a8d95049f8d"
  license "GPLv3"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--bin", "m1necraft", *std_cargo_args
  end

  test do
    system "#{bin}/m1necraft", "--help"
  end
end
