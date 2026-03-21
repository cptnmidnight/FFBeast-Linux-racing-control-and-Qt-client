from __future__ import annotations

import json
from pathlib import Path
from typing import Any


PROFILE_DIR = Path.home() / ".config" / "ffbeast-qt"
PROFILE_FILE = PROFILE_DIR / "profiles.json"


class ProfileStore:
    def __init__(self) -> None:
        PROFILE_DIR.mkdir(parents=True, exist_ok=True)

    def load_profiles(self) -> dict[str, dict[str, Any]]:
        if not PROFILE_FILE.exists():
            return {}
        try:
            return json.loads(PROFILE_FILE.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError):
            return {}

    def save_profiles(self, profiles: dict[str, dict[str, Any]]) -> None:
        PROFILE_FILE.write_text(
            json.dumps(profiles, indent=2, sort_keys=True),
            encoding="utf-8",
        )
