# Contributing to iNames - Inclusive Multicultural Names Generator

Thank you for your interest in contributing to iNames! We welcome contributions that help make our name collection more diverse and inclusive.

## How to Contribute

### Adding New Names

We especially welcome contributions that add names from cultures not yet well-represented in our collection. When adding names:

1. **Use Latin characters only** - All names must be transliterated to Latin/English characters
2. **Respect cultural authenticity** - Ensure names are real names used in the culture, not made-up
3. **Provide context** - If possible, add a comment about the origin or meaning of names
4. **Keep it appropriate** - Only add names that are respectful and appropriate

### Where to Add Names

- **Adjectives**: Add to `data/adjectives.txt` - these should be names that can work as descriptive qualities
- **Nouns**: Add to `data/nouns.txt` - these are the main name components

### Code Contributions

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-names`)
3. Make your changes
4. Run tests (`cargo test`)
5. Run clippy (`cargo clippy`)
6. Format code (`cargo fmt`)
7. Commit your changes (`git commit -m 'Add Sanskrit names'`)
8. Push to your branch (`git push origin feature/amazing-names`)
9. Open a Pull Request

### Reporting Issues

- Use the issue tracker to report bugs
- Describe the issue clearly
- Include steps to reproduce if applicable

### Pull Request Guidelines

- Keep PRs focused on a single feature or fix
- Update tests if you change functionality
- Update documentation if needed
- Ensure all tests pass
- Follow the existing code style

## Development Setup

```bash
# Clone the repo
git clone https://github.com/hamzeghalebi/inames.git
cd inames

# Build the project
cargo build

# Run tests
cargo test

# Run the generator
cargo run -- --amount 5
```

## Code of Conduct

Please be respectful and inclusive in all interactions. We aim to create a welcoming environment for contributors from all backgrounds.

## Questions?

Feel free to open an issue if you have questions about contributing.