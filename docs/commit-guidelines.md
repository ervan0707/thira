# Commit Message Guidelines

## Overview

Thira enforces a standardized commit message format based on [Conventional Commits](https://www.conventionalcommits.org/). This guide explains the required format and how to write effective commit messages.

## Message Format

```
<type>(<scope>): <subject>

[optional body]

[optional footer(s)]
```

### Required Fields

1. **Type**: The category of change (feat, fix, etc.)
2. **Scope**: The area of the change (optional)
3. **Subject**: A brief description of the change

## Types

Default supported types:

- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation changes
- `style`: Code style changes (formatting, semicolons, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or modifying tests
- `build`: Build system changes
- `ci`: CI configuration changes
- `chore`: General maintenance
- `revert`: Reverting previous changes

## Scopes

Default supported scopes:

- `api`: API-related changes
- `ui`: User interface changes
- `db`: Database changes
- `core`: Core functionality
- `cli`: Command-line interface
- `config`: Configuration changes
- `deps`: Dependencies
- `tests`: Test infrastructure

## Rules and Validation

### Subject Line

- Must start with type: `type(scope): subject`
- Minimum length: 3 characters
- Maximum length: 72 characters
- Should be written in imperative mood
- No period at the end

### Body (Optional)

- Separated from subject by blank line
- Maximum line length: 100 characters
- Should explain what and why vs. how
- Can include multiple paragraphs

## Examples

### Good Commit Messages

```
feat(api): add user authentication endpoint

Implement JWT-based authentication for users.
- Add login endpoint
- Add token validation
- Include refresh token support

Closes #123
```

```
fix(db): resolve connection pooling memory leak

The connection pool wasn't properly closing idle connections,
leading to memory leaks in long-running instances.
```

```
docs(cli): update installation instructions
```

### Bad Commit Messages

```
fixed bug  # No type or clear description
```

```
feat(api) add stuff  # Too vague, poor description
```

```
chore: updates # Not descriptive enough
```

## Configuration

Customize validation rules in `hooks.yaml`:

```yaml
lint:
  types:
    - feat
    - fix
    # Add custom types
  scopes:
    - api
    - ui
    # Add custom scopes
  min_subject_length: 3
  max_subject_length: 72
  max_body_line_length: 100
```

## Tips for Good Commit Messages

1. **Be Specific**

   - Clearly describe what changes were made
   - Avoid vague descriptions

2. **Use Imperative Mood**

   - Write as if giving a command
   - "Add feature" not "Added feature"

3. **Separate Concerns**

   - One logical change per commit
   - Break large changes into smaller commits

4. **Reference Issues**

   - Link to relevant issues/tickets
   - Use keywords like "Fixes", "Closes"

5. **Provide Context**
   - Explain why changes were made
   - Document important decisions

## Common Issues and Solutions

1. **Invalid Type**

   - Check supported types
   - Use closest matching type
   - Add custom types in config

2. **Invalid Scope**

   - Verify scope exists
   - Consider if scope is needed
   - Add new scopes in config

3. **Subject Too Short/Long**
   - Be concise but clear
   - Break into multiple commits if needed
   - Focus on essential information

## Further Reading

- [Conventional Commits](https://www.conventionalcommits.org/)
- [Angular Commit Guidelines](https://github.com/angular/angular/blob/master/CONTRIBUTING.md#-commit-message-format)
- [Git Commit Best Practices](https://chris.beams.io/posts/git-commit/)
