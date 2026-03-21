from __future__ import annotations

import json
import os
import shlex
from dataclasses import dataclass
from typing import Any

from PySide6.QtCore import QObject, QProcess, Signal


def _service_command() -> list[str]:
    override = os.environ.get("FFBEAST_SERVICE_CMD", "").strip()
    if override:
        return shlex.split(override)
    return ["cargo", "run", "-p", "ffbeast-service", "--quiet"]


@dataclass
class PendingRequest:
    method: str


class FFBeastServiceClient(QObject):
    connected_changed = Signal(bool)
    log_message = Signal(str)
    request_failed = Signal(str, str)
    response_received = Signal(str, object)
    telemetry_status = Signal(dict)
    telemetry_disconnected = Signal(str)

    def __init__(self, parent: QObject | None = None) -> None:
        super().__init__(parent)
        self._process = QProcess(self)
        self._process.setProgram(_service_command()[0])
        self._process.setArguments(_service_command()[1:])
        self._process.readyReadStandardOutput.connect(self._read_stdout)
        self._process.readyReadStandardError.connect(self._read_stderr)
        self._process.errorOccurred.connect(self._on_process_error)
        self._process.started.connect(lambda: self.connected_changed.emit(True))
        self._process.finished.connect(lambda *_: self.connected_changed.emit(False))
        self._next_id = 1
        self._pending: dict[int, PendingRequest] = {}
        self._stdout_buffer = ""

    def start(self) -> None:
        if self._process.state() != QProcess.NotRunning:
            return
        self.log_message.emit(f"Starting service: {' '.join(_service_command())}")
        self._process.start()

    def stop(self) -> None:
        if self._process.state() == QProcess.NotRunning:
            return
        self._process.terminate()
        if not self._process.waitForFinished(2000):
            self._process.kill()

    def send_request(self, method: str, request: Any) -> int:
        if self._process.state() == QProcess.NotRunning:
            raise RuntimeError("ffbeast-service is not running")

        request_id = self._next_id
        self._next_id += 1
        payload = {"id": request_id, "request": request}
        encoded = json.dumps(payload, separators=(",", ":")) + "\n"
        self._pending[request_id] = PendingRequest(method=method)
        self._process.write(encoded.encode("utf-8"))
        self.log_message.emit(f"-> {method}")
        return request_id

    def connect_hardware(self) -> int:
        return self.send_request("connect_hardware", "ConnectHardware")

    def get_handshake(self) -> int:
        return self.send_request("get_handshake", "GetHandshake")

    def start_telemetry(self) -> int:
        return self.send_request("start_telemetry", "StartTelemetry")

    def stop_telemetry(self) -> int:
        return self.send_request("stop_telemetry", "StopTelemetry")

    def update_field(self, method: str, field_id: int, data: list[int], index: int = 0) -> int:
        payload = {
            "UpdateHardwareField": {
                "field_id": field_id,
                "index": index,
                "data": data,
            }
        }
        return self.send_request(method, payload)

    def save_settings(self) -> int:
        return self.send_request("save_settings", "SaveSettings")

    def reset_center(self) -> int:
        return self.send_request("reset_center", "ResetCenter")

    def reboot_device(self) -> int:
        return self.send_request("reboot_device", "RebootDevice")

    def switch_to_dfu(self) -> int:
        return self.send_request("switch_to_dfu", "SwitchToDfu")

    def _read_stdout(self) -> None:
        self._stdout_buffer += bytes(self._process.readAllStandardOutput()).decode("utf-8")
        while "\n" in self._stdout_buffer:
            line, self._stdout_buffer = self._stdout_buffer.split("\n", 1)
            line = line.strip()
            if not line:
                continue
            self._handle_message(line)

    def _read_stderr(self) -> None:
        data = bytes(self._process.readAllStandardError()).decode("utf-8").strip()
        if data:
            self.log_message.emit(data)

    def _handle_message(self, line: str) -> None:
        try:
            payload = json.loads(line)
        except json.JSONDecodeError as error:
            self.log_message.emit(f"Invalid service JSON: {error}: {line}")
            return

        if "Response" in payload:
            self._handle_response(payload["Response"])
            return

        if "Event" in payload:
            self._handle_event(payload["Event"])
            return

        self.log_message.emit(f"Unknown service message: {payload}")

    def _handle_response(self, response: dict[str, Any]) -> None:
        request_id = response.get("id")
        pending = self._pending.pop(request_id, PendingRequest(method="unknown"))
        error = response.get("error")
        if error:
            self.request_failed.emit(pending.method, error)
            self.log_message.emit(f"<- {pending.method} error: {error}")
            return

        result = response.get("result")
        self.response_received.emit(pending.method, result)
        self.log_message.emit(f"<- {pending.method} ok")

    def _handle_event(self, event: dict[str, Any]) -> None:
        telemetry = event.get("Telemetry")
        if not telemetry:
            self.log_message.emit(f"Unknown service event: {event}")
            return

        if "Status" in telemetry:
            status = telemetry["Status"].get("status", {})
            self.telemetry_status.emit(status)
            return

        if "Disconnected" in telemetry:
            reason = telemetry["Disconnected"].get("reason", "unknown")
            self.telemetry_disconnected.emit(reason)
            return

        self.log_message.emit(f"Unknown telemetry event: {telemetry}")

    def _on_process_error(self, error: QProcess.ProcessError) -> None:
        self.log_message.emit(f"Service process error: {error}")
        self.connected_changed.emit(False)
