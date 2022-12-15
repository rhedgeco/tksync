# Typekit Sync
This is an unofficial tool for downloading fonts from Adobe Fonts (Typekit). You must have a valid adobe account to use this tool as it takes advantage of the Web Projects feature.

# Usage
### Sync all projects
To sync all projects just run:
```
tksync
```

### Add new Typekit Project
To add a new project to be tracked by tksync, run the following:
```
tksync add [OPTIONS] <ID> <NAME> <PATH>

Arguments:
  <ID>    Id of the typekit project
  <NAME>  Name of typekit project
  <PATH>  Path to download project fonts to

Options:
  -r, --replace  Overwrite existing project id if it exists
```

## [MIT LICENSE](LICENSE.md)