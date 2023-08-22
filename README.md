# Dead Drop

Very simple Web-based text notepad that allows anyone to write and read to it.  Everything exists in memory.  It's only intended for temporary notes.

## Quickstart

This will build the FE and start the BE.

```bash
yarn
cargo watch -x run
```

## Setup

### Helm

#### Install

```bash
helm install --create-namespace -n dead-pub dead-pub devops/kubernetes/charts/dead-drop -f devops/kubernetes/charts/dead-drop/values.yaml -f devops/kubernetes/values/dead-pub.yaml
```

#### Upgrade

```bash
helm upgrade -n dead-pub dead-pub devops/kubernetes/charts/dead-drop -f devops/kubernetes/charts/dead-drop/values.yaml -f devops/kubernetes/values/dead-pub.yaml
```

## TODO

- [ ] Add browser-encrypted notes (probably just symmetric encryption using the URL "hash")
- [ ] Add merging/patching on save, so multiple clients can update a note without overwriting each-other
- [ ] Consider moving to memcached or separate server from storage so more requests might be handled by the server
