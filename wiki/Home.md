# SigmaOS-bash

SigmaOS integration component for **Bash shell integration**.

## Commands

```bash
sigmaos-bash install   # install package + deploy defaults
sigmaos-bash deploy    # copy config templates to $HOME
sigmaos-bash status    # binary/package/config report
sigmaos-bash reset     # remove deployed config
sigmaos-bash open      # launch the application
```

## Design

- Shell-based component manager (Omarchy-style per-app repos)
- Secure defaults; no secrets stored
- Deterministic `test.sh` with isolated `$HOME`
