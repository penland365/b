b - Base 64 / 32 / 16 transcoding CLI tool
====
![PyPI](https://img.shields.io/pypi/l/Django.svg?style=plastic)
![CircleCI branch](https://img.shields.io/circleci/project/penland365/b/master/master.svg)
![GitHub last commit (master)](https://img.shields.io/github/last-commit/penland365/b/master/config.svg)


A strict implementation of the latest 
[RFC 4648 - The Base16, Base32, and Base64 Data Encodings](https://tools.ietf.org/html/rfc4648)

## tl;dr
```shell
$ b 64 encode "foobar"
Zm9vYmFy
```

## Why?
The common tool used for base64 encoding off the command line is the venerable `base64`.
As the man page for that tool states:
> base64 - encode / decode binary file as RFC 1341 MIME base64

This leads to two points:
1. RFC 1341 has been obsolted multiple times, though the implementation is most likely still valid.
2. MIME base64 encoding utilizes a different alphabet then the modern base64 RFC.

## OK, but why are you care-mad?
I once lost 20 hours of my life because `base64` provided an encoded b64 string that
did not maintain fidelity when Kubernetes decoded it.
