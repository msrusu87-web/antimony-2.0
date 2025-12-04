# Contributing to ANTIMONY COIN 2.0

First off, thank you for considering contributing to Antimony Coin 2.0! It's people like you that make ATMN such a great project.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code.

### Our Pledge

In the interest of fostering an open and welcoming environment, we as contributors and maintainers pledge to make participation in our project and our community a harassment-free experience for everyone.

## How Can I Contribute?

### Reporting Bugs

**Security Issues**: Please email security@antimony.carphatian.ro. Do NOT open public issues for security vulnerabilities.

**Other Bugs**: Use GitHub Issues with:
- Clear title
- Exact steps to reproduce
- Expected vs. actual behavior
- System info (OS, Node version, etc.)
- Screenshots if applicable

### Suggesting Enhancements

Use GitHub Issues with label `enhancement`:
- Clear use case
- Benefits to the community
- Possible drawbacks
- Links to similar features in other projects

### Pull Requests

1. **Fork** the repository
2. **Create a branch**: `git checkout -b feature/amazing-feature`
3. **Make changes** following our style guide
4. **Add tests** for new functionality
5. **Update docs** if needed
6. **Commit with clarity**: `git commit -m 'feat: add amazing feature'`
7. **Push**: `git push origin feature/amazing-feature`
8. **Open PR** with detailed description

## Development Setup

### Prerequisites
- Node.js 18+
- Docker & Docker Compose
- Git

### Quick Start
```bash
git clone https://github.com/msrusu87-web/antimony-2.0.git
cd antimony-2.0
make setup
make testnet
```

### Running Tests
```bash
make test              # All tests
make test-core        # Core blockchain
make test-contracts   # Smart contracts
```

## Style Guide

### Commit Messages
```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**: feat, fix, docs, style, refactor, test, chore
**Scope**: core, evm, explorer, wallet, bridge, etc.
**Subject**: Lowercase, imperative, no period

Example:
```
feat(core): implement dynamic difficulty adjustment

The difficulty now adjusts based on network hashrate
to maintain 12-second block times.

Closes #123
```

### Code Style

**JavaScript/TypeScript**:
- Use ESLint config: `npm run lint`
- Prettier formatting: `npm run format`
- 2-space indentation
- const/let (no var)
- Async/await over callbacks

**Rust**:
- `cargo fmt` for formatting
- `cargo clippy` for linting
- Descriptive variable names
- Document public APIs

**Solidity**:
- Follow OpenZeppelin patterns
- Natspec comments for all functions
- SafeMath (or ^0.8.0 built-in checks)
- Event logging for state changes

## Testing Requirements

- **Unit Tests**: 80%+ coverage
- **Integration Tests**: Critical paths
- **End-to-End Tests**: Full workflows
- **Security Tests**: All contract functionality

Run tests before submitting PR:
```bash
npm test
npm run test:coverage
```

## Documentation

- **Code Comments**: Why, not what
- **READMEs**: Clear setup instructions
- **API Docs**: All public methods
- **Examples**: Working code samples
- **Architecture Docs**: System design

Update docs/ if adding features.

## Peer Review Process

1. **Automated Checks**: GitHub Actions must pass
2. **Code Review**: Minimum 1 maintainer approval
3. **Testing**: Verified by reviewers
4. **Merge**: Squash commits, clear message

## Release Process

1. Update version in package.json
2. Update CHANGELOG.md
3. Create git tag: `git tag v1.0.0`
4. Push tag: `git push origin v1.0.0`
5. GitHub Actions creates release

## Getting Help

- **Discord**: [Join Community](https://discord.gg/atmony)
- **Issues**: Ask in GitHub Issues
- **Docs**: Check [Developer Guide](./atmn-docs/README.md)
- **Email**: dev@antimony.carphatian.ro

## Recognition

Contributors will be recognized:
- In CONTRIBUTORS.md
- On project website
- In monthly community calls
- Eligible for community grants

## Legal

By submitting contributions, you agree:
- Your code is original or properly licensed
- Your work can be used under the MIT license
- You grant Antimony Foundation a perpetual license
- You have authority to grant these rights

---

**Thank you for contributing to ANTIMONY COIN 2.0! 🚀**

Last Updated: December 4, 2025
