# FFBeast Qt Client

This is the new Qt frontend target for FFBeast.

## Current Scope

- Starts `ffbeast-service` as a subprocess
- Talks to it over line-delimited JSON on stdio
- Sends backend requests from `libs/backend_api`
- Receives both synchronous responses and async telemetry events

## Run

From a shell with `PySide6` available:

```bash
python3 apps/ffbeast-qt/main.py
```

You can override the service command with `FFBEAST_SERVICE_CMD`:

```bash
FFBEAST_SERVICE_CMD="cargo run -p ffbeast-service --quiet" python3 apps/ffbeast-qt/main.py
```

## Next Steps

- Add settings tabs for effects, hardware, GPIO, and ADC
- Persist frontend preferences
- Replace the remaining Tauri frontend once feature parity is sufficient
