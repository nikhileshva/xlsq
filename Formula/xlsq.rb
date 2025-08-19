class Xlsq < Formula
  desc "Fast and lightweight CLI tool for reading and searching Excel files"
  homepage "https://github.com/nikhileshva/xlsq"
  url "https://github.com/nikhileshva/xlsq/archive/v0.1.0.tar.gz"
  sha256 "YOUR_SHA256_HERE" # Update this when you create the release
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    # Test the binary exists and shows help
    assert_match "A CLI tool for reading and searching Excel files", shell_output("#{bin}/xlsq --help")
  end
end