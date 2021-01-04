# Pico Package Keeper
## Overview
* [TOML-based manifests](./manifest.md)
* [Simplicity](./non-goals.md)
* [Modern technologies](./technologies.md)
* [Static, no dependencies](./dependencies.md)

## Requirements
* Linux kernel 2.6.22, with _curl_ and _chroot_ binary installed target 
* Media with at least 100M, 900MB for more complete solution as minimum
* Internet connection

## Workspace
Workspace is a central project directory, which houses package definitions, an Alpine chroot and caches for dependency downloads.
```
<workspace-directory>/
├─ workspace.toml
├─ packages/
│  ├─ <package-name>/
│  │  ├─ <package-files>
├─ work/
│  ├─ git/
│  │  ├─ <hashed-git-url-and-revision>/
│  │  │  ├─ <repository-files>
│  ├─ url/
│  │  ├─ <hashed-url>/
│  │  │  ├─ <downloaded-file-unpacked-if-archive>
│  ├─ alpine/
│  │  ├─ <alpine-chroot>
```

## Package states
* Resident - unarchived, uncompressed package residing in a file system
* Packaged - archived and compressed version of the resident package

## Package parts
1. Manifest - manifest alone is already considered a valid package
2. Directory */src* - directory containing files required by the package (configs, source, patches)
3. Directory */build* - temporary directory present only after the package was built
4. File */build/status.toml* - file which contains the status of the last build (success, fail)
5. Directory */build/bin* - directory which contains binaries wrapped in `ppk run <binary>` calls, providing correct environment for execution
6. Directory */build/artefacts* - directory which contains the raw build results

### File structure
```
<package-name>/
├─ manifest.toml
├─ src/
│  ├─ <build-files>
├─ build/
│  ├─ status.toml
│  ├─ bin/
│  │  ├─ <executable-files>
│  ├─ artefacts/
│  │  ├─ <other-application-files>
```