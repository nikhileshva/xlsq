# Deployment Guide

This guide covers how to distribute your `xlsq` binary through various channels.

## 🚀 GitHub Releases (Automated)

### Prerequisites
1. Push your code to GitHub
2. The release workflow is already configured in `.github/workflows/release.yml`

### Creating a Release
```bash
# Tag and push a release
git tag v0.1.0
git push origin v0.1.0
```

This automatically:
- Builds binaries for Linux, macOS (Intel + ARM), and Windows
- Creates a GitHub release
- Uploads all binaries as release assets

### Manual Release (Alternative)
```bash
# Build for current platform
cargo build --release

# Build for other platforms (requires cross)
cargo install cross
cross build --target x86_64-unknown-linux-gnu --release
cross build --target x86_64-pc-windows-msvc --release
cross build --target x86_64-apple-darwin --release
```

## 📦 Cargo/crates.io

### Prerequisites
1. Create an account at [crates.io](https://crates.io/)
2. Get your API token: `cargo login`

### Publishing
```bash
# Check package before publishing
cargo package --list
cargo package --dry-run

# Publish to crates.io
cargo publish
```

### Updates
```bash
# Update version in Cargo.toml, then:
cargo publish
```

## 🍺 Homebrew

### Creating Your Own Tap
```bash
# Create a homebrew tap repository
# Repository name should be homebrew-<tap-name>
# e.g., homebrew-xlsq

# Add the formula (already created in Formula/xlsq.rb)
# Update the SHA256 hash after creating a release

# Users can then install with:
# brew tap nikhileshva/xlsq
# brew install xlsq
```

### Submitting to homebrew-core (for popular tools)
- Fork [homebrew/homebrew-core](https://github.com/Homebrew/homebrew-core)
- Add your formula to `Formula/xlsq.rb`
- Submit a pull request

## 🐳 Docker

### Building and Pushing
```bash
# Build image
docker build -t xlsq:latest .
docker build -t xlsq:v0.1.0 .

# Tag for registry
docker tag xlsq:latest ghcr.io/nikhileshva/xlsq:latest
docker tag xlsq:v0.1.0 ghcr.io/nikhileshva/xlsq:v0.1.0

# Push to GitHub Container Registry
echo $GITHUB_TOKEN | docker login ghcr.io -u nikhileshva --password-stdin
docker push ghcr.io/nikhileshva/xlsq:latest
docker push ghcr.io/nikhileshva/xlsq:v0.1.0
```

### Automated Docker Builds
Add to your GitHub Actions workflow:
```yaml
- name: Build and push Docker image
  uses: docker/build-push-action@v5
  with:
    push: true
    tags: ghcr.io/nikhileshva/xlsq:latest,ghcr.io/nikhileshva/xlsq:${{ github.ref_name }}
```

## 📱 Package Managers

### AUR (Arch Linux)
Create a PKGBUILD file and submit to AUR.

### Snap (Ubuntu/Linux)
Create a `snap/snapcraft.yaml` file.

### Chocolatey (Windows)
Create a chocolatey package.

### npm (JavaScript ecosystem)
You can wrap your binary in an npm package for Node.js users.

## 🔄 Distribution Checklist

### Before First Release
- [ ] Update `Cargo.toml` metadata (author, description, repository)
- [ ] Write comprehensive README
- [ ] Add LICENSE file
- [ ] Set up CI/CD pipeline
- [ ] Test on multiple platforms
- [ ] Write changelog

### For Each Release
- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md`
- [ ] Create git tag
- [ ] Verify automated builds pass
- [ ] Test downloaded binaries work
- [ ] Update documentation if needed

### Post-Release
- [ ] Announce on social media/forums
- [ ] Update package managers (Homebrew, AUR, etc.)
- [ ] Monitor for issues and feedback

## 🎯 Recommended Distribution Strategy

For a new project like `xlsq`:

1. **Start with GitHub Releases** - Easiest and most flexible
2. **Publish to Cargo** - Essential for Rust users
3. **Create install script** - User-friendly one-liner installation
4. **Add Docker support** - For containerized environments
5. **Consider Homebrew tap** - Popular among macOS/Linux developers

As your project grows:
- Submit to homebrew-core
- Create packages for various Linux distributions
- Consider Windows-specific distribution methods

## 📊 Usage Analytics

Consider adding telemetry (opt-in) to understand:
- Which platforms are most popular
- Which features are used most
- Performance metrics

This helps prioritize future development and distribution efforts.