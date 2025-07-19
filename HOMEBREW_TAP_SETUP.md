# Setting up Homebrew Tap for iNames

This guide will help you set up a Homebrew tap to distribute iNames.

## Steps to Create the Tap

1. **Create a new GitHub repository** named `homebrew-tap` under the RustSandbox organization:
   - Go to https://github.com/organizations/RustSandbox/repositories/new
   - Name it: `homebrew-tap` (or just `tap`)
   - Make it public
   - Initialize with a README

2. **Clone the tap repository locally**:
   ```bash
   git clone git@github.com:RustSandbox/homebrew-tap.git
   cd homebrew-tap
   ```

3. **Copy the formula file**:
   ```bash
   cp /path/to/inames/inames.rb Formula/inames.rb
   ```

4. **Update the formula with actual SHA256 checksums**:
   
   After creating a release (e.g., v0.1.0), download the release archives and calculate their checksums:
   
   ```bash
   # Download the archives
   curl -L -o inames-darwin.tar.gz \
     https://github.com/RustSandbox/inames/releases/download/v0.1.0/inames-v0.1.0-x86_64-apple-darwin.tar.gz
   
   curl -L -o inames-darwin-arm.tar.gz \
     https://github.com/RustSandbox/inames/releases/download/v0.1.0/inames-v0.1.0-aarch64-apple-darwin.tar.gz
   
   curl -L -o inames-linux.tar.gz \
     https://github.com/RustSandbox/inames/releases/download/v0.1.0/inames-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
   
   # Calculate SHA256
   shasum -a 256 inames-darwin.tar.gz
   shasum -a 256 inames-darwin-arm.tar.gz
   shasum -a 256 inames-linux.tar.gz
   ```

5. **Update the formula** with the actual SHA256 values in place of the placeholders.

6. **Commit and push**:
   ```bash
   git add Formula/inames.rb
   git commit -m "Add inames formula"
   git push origin main
   ```

## Testing the Tap Locally

Before pushing, you can test the formula locally:

```bash
# From the tap directory
brew install --build-from-source ./Formula/inames.rb

# Test it works
inames

# Uninstall when done testing
brew uninstall inames
```

## Testing the Published Tap

Once pushed to GitHub:

```bash
brew tap RustSandbox/tap
brew install inames
```

## Updating the Formula

When you release a new version:

1. Update the version number in the formula
2. Update all the download URLs
3. Recalculate and update all SHA256 checksums
4. Test locally
5. Commit and push

## Automation

The release workflow already includes automation to update the formula. You'll need to:

1. Add a `HOMEBREW_TAP_TOKEN` secret to your repository
2. Update the workflow to push to the correct tap repository

The workflow will automatically calculate checksums and update the formula on each release.