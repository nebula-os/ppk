## URL
All packages can be referred to as URLs. Typical package URL usually consists has a structure of `<source>:<package-name>?<parameter>&<another-parameter>&arg-<arg-name>=<arg-value>`.

### Arguments
Named arguments are a way to pass values to the package.

### Package
The first kind of source is the `pkg` source. It informs `ppk` to search for packages in its own connected repositories and caches.

Format
```
pkg:<package-name>?
      from=<version>
    & from-excluding=<version>
    & to=<version>
    & to-excluding=<version>
    & repo=<repo-name>
    & arg-<arg-name>=<arg-value>
```

### File
`file` is a kind of source that informs `ppk` to search for the requested packaged in the file system. Path given should either point to the `manifest.toml` file or the folder which contains it.

Format
```
file:<package-path>?
      arg-<arg-name>=<arg-value>
```