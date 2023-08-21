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
