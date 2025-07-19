class Inames < Formula
  desc "Inclusive multicultural random name generator"
  homepage "https://github.com/RustSandbox/inames"
  version "0.1.0"
  license any_of: ["MIT", "Apache-2.0"]

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/RustSandbox/inames/releases/download/v0.1.0/inames-v0.1.0-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_ARM64"
    else
      url "https://github.com/RustSandbox/inames/releases/download/v0.1.0/inames-v0.1.0-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_X86_64"
    end
  end

  on_linux do
    url "https://github.com/RustSandbox/inames/releases/download/v0.1.0/inames-v0.1.0-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "PLACEHOLDER_SHA256_LINUX"
  end

  def install
    bin.install "inames"
  end

  test do
    assert_match(/\w+-\w+/, shell_output("#{bin}/inames"))
  end
end