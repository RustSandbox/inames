# Homebrew Setup

To make this package available via Homebrew, you'll need to:

1. Create a new repository called `homebrew-tap` in your GitHub account
2. After each release, the GitHub Action will generate a formula file
3. Copy the generated `names.rb` formula to your tap repository

## Manual Formula Creation

If you want to create the formula manually:

```bash
# After creating a release, get the SHA256 checksums:
shasum -a 256 names-*-darwin.tar.gz
shasum -a 256 names-*-linux.tar.gz

# Update the formula with the new version and checksums
```

## Example Tap Structure

Your `homebrew-tap` repository should look like:

```
homebrew-tap/
└── Formula/
    └── names.rb
```

Users can then install with:
```bash
brew tap hamzeghalebi/tap
brew install names
```