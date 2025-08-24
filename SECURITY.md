# Security Policy

## Supported Versions

Use this section to tell people about which versions of your project are
currently being supported with security updates.

| Version | Supported          |
| ------- | ------------------ |
| 5.1.x   | :white_check_mark: |
| 5.0.x   | :x:                |
| 4.0.x   | :white_check_mark: |
| < 4.0   | :x:                |

## Security Features

### Implemented Security Measures

- **SHA Verification**: All releases and security workflows include commit SHA verification
- **Rate Limit Protection**: GitHub API rate limiting with monitoring and backoff strategies
- **Workflow Concurrency Controls**: Prevents overlapping workflows that could lead to rate limit issues
- **Dependency Security**: Daily security audits with cargo-audit and cargo-deny
- **License Compliance**: Automated license checking with GPL detection
- **Supply Chain Security**: Dependency vulnerability scanning and update management

### Rate Limit Protection

The project implements several measures to prevent GitHub API rate limit vulnerabilities:

1. **Staggered Scheduling**: Dependency management tools run on different days
2. **Concurrency Controls**: Prevents multiple workflows from running simultaneously
3. **Retry Logic**: Implements exponential backoff for API calls
4. **Rate Limit Monitoring**: Daily monitoring of GitHub API usage

## Reporting a Vulnerability

Use this section to tell people how to report a vulnerability.

Tell them where to go, how often they can expect to get an update on a
reported vulnerability, what to expect if the vulnerability is accepted or
declined, etc.

### Security Contact

Please report security vulnerabilities by creating a private security advisory
through GitHub's security reporting feature, or by contacting the maintainers directly.

### Response Timeline

- **Initial Response**: Within 48 hours
- **Vulnerability Assessment**: Within 7 days  
- **Fix Timeline**: Critical vulnerabilities patched within 2 weeks
- **Disclosure**: Coordinated disclosure after fix is available
