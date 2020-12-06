# semver-sort

Sort input lines by semantic version, inspired by `sort(1)`.

```sh
$ cat foo.txt
hi there 0.1.0
whoa there 2.0.0
no way 0.1.0-alpha
$ semver-sort -k3 --prerelease < foo.txt
no way 0.1.0-alpha
hi there 0.1.0
whoa there 2.0.0
```

# installation

Releases for macOS, darwin, and windows are available on GitHub.

```sh
# linux:
$ curl -sL https://github.com/chrisdickinson/semver-sort/releases/download/v1.0.0/semver-sort_x64_linux.tar.gz | tar xfz -
$ sudo mv semver-sort /usr/local/bin

# macOS
$ curl -sL https://github.com/chrisdickinson/semver-sort/releases/download/v1.0.0/semver-sort_x64_darwin.tar.gz | tar xfz -
$ sudo mv semver-sort /usr/local/bin
```

# license

MIT
