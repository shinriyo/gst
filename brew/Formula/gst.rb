class Gst < Formula
  desc "A TUI Git Status Tool written in Rust"
  homepage "https://github.com/shinriyo/gst"
  version "0.1.0"
  
  if OS.mac? && Hardware::CPU.arm?
    url "https://github.com/shinriyo/gst/releases/download/v#{version}/gst-aarch64-apple-darwin.tar.gz"
    sha256 "YOUR_AARCH64_DARWIN_SHA256"
  elsif OS.mac? && Hardware::CPU.intel?
    url "https://github.com/shinriyo/gst/releases/download/v#{version}/gst-x86_64-apple-darwin.tar.gz"
    sha256 "YOUR_X86_64_DARWIN_SHA256"
  elsif OS.linux? && Hardware::CPU.intel?
    url "https://github.com/shinriyo/gst/releases/download/v#{version}/gst-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "YOUR_X86_64_LINUX_SHA256"
  end

  def install
    bin.install "gst"
  end

  test do
    system "#{bin}/gst", "--version"
  end
end 