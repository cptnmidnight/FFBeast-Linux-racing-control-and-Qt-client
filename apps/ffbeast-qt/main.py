from __future__ import annotations

import sys
from typing import Any

from PySide6.QtCore import Qt
from PySide6.QtGui import QFont
from PySide6.QtWidgets import (
    QApplication,
    QGridLayout,
    QHBoxLayout,
    QInputDialog,
    QLabel,
    QMainWindow,
    QPushButton,
    QPlainTextEdit,
    QTabWidget,
    QVBoxLayout,
    QWidget,
)

from profile_store import ProfileStore
from service_client import FFBeastServiceClient
from settings_tabs import (
    ADC_SPECS,
    ADVANCED_WHEEL_SPECS,
    DRIVE_SPECS,
    GPIO_SPECS,
    MaintenancePage,
    ProfilesPage,
    SettingsPage,
    WHEEL_SETUP_SPECS,
)


class MainWindow(QMainWindow):
    def __init__(self) -> None:
        super().__init__()
        self.setWindowTitle("FFBeast Racing Control")
        self.resize(1080, 760)

        self.client = FFBeastServiceClient(self)
        self._handshake: dict[str, Any] = {}
        self._profiles = ProfileStore()
        self._saved_profiles: dict[str, dict[str, Any]] = self._profiles.load_profiles()
        self._build_ui()
        self._connect_signals()

    def _build_ui(self) -> None:
        root = QWidget(self)
        layout = QVBoxLayout(root)
        layout.setContentsMargins(20, 20, 20, 20)
        layout.setSpacing(16)

        title = QLabel("FFBeast Linux Racing Control", self)
        title.setFont(QFont("DejaVu Sans", 20, QFont.Weight.Bold))
        layout.addWidget(title)

        subtitle = QLabel(
            "Sim-racing focused setup for FFBeast wheels. Peripheral and calibration controls live under Advanced.",
            self,
        )
        subtitle.setWordWrap(True)
        subtitle.setStyleSheet("color: #5b6470;")
        layout.addWidget(subtitle)

        controls = QHBoxLayout()
        controls.setSpacing(10)

        self.start_service_button = QPushButton("Start Service")
        self.connect_button = QPushButton("Connect Wheel")
        self.handshake_button = QPushButton("Load Wheel Setup")
        self.start_telemetry_button = QPushButton("Start Telemetry")
        self.stop_telemetry_button = QPushButton("Stop Telemetry")

        for button in (
            self.start_service_button,
            self.connect_button,
            self.handshake_button,
            self.start_telemetry_button,
            self.stop_telemetry_button,
        ):
            button.setMinimumHeight(36)
            controls.addWidget(button)

        controls.addStretch(1)
        layout.addLayout(controls)

        self.tabs = QTabWidget(self)
        self.drive_page = SettingsPage(
            "Drive",
            "Drive",
            "Primary force-feedback setup for steering feel, travel, soft stops, and dampening.",
            DRIVE_SPECS,
            self.tabs,
        )
        self.wheel_setup_page = SettingsPage(
            "Wheel Setup",
            "Wheel Setup",
            "Core wheel controls for force enablement, direction, and power-related setup.",
            WHEEL_SETUP_SPECS,
            self.tabs,
        )
        self.advanced_page = SettingsPage(
            "Advanced",
            "Advanced",
            "Calibration, control-loop tuning, and optional peripheral configuration for pedals or add-on hardware.",
            ADVANCED_WHEEL_SPECS + GPIO_SPECS + ADC_SPECS,
            self.tabs,
        )

        self.save_settings_button = QPushButton("Save Settings")
        self.reset_center_button = QPushButton("Reset Center")
        self.reboot_button = QPushButton("Reboot Wheel")
        self.dfu_button = QPushButton("Enter DFU")
        self.maintenance_page = MaintenancePage(
            self.save_settings_button,
            self.reset_center_button,
            self.reboot_button,
            self.dfu_button,
            self.tabs,
        )

        self.profile_selector = self._build_profile_selector()
        self.profiles_page = ProfilesPage(self.profile_selector, self.tabs)

        self.tabs.addTab(self.drive_page, "Drive")
        self.tabs.addTab(self.wheel_setup_page, "Wheel Setup")
        self.tabs.addTab(self.profiles_page, "Profiles")
        self.tabs.addTab(self.maintenance_page, "Maintenance")
        self.tabs.addTab(self.advanced_page, "Advanced")
        layout.addWidget(self.tabs, 1)

        status_grid = QGridLayout()
        status_grid.setHorizontalSpacing(24)
        status_grid.setVerticalSpacing(10)

        self.service_state = QLabel("Stopped")
        self.device_state = QLabel("Disconnected")
        self.firmware_value = QLabel("-")
        self.position_value = QLabel("-")
        self.torque_value = QLabel("-")
        self.buttons_value = QLabel("-")

        rows = [
            ("Service", self.service_state),
            ("Device", self.device_state),
            ("Firmware", self.firmware_value),
            ("Position", self.position_value),
            ("Torque", self.torque_value),
            ("Buttons", self.buttons_value),
        ]

        for row, (label, value) in enumerate(rows):
            status_grid.addWidget(QLabel(label), row, 0)
            value.setTextInteractionFlags(Qt.TextSelectableByMouse)
            status_grid.addWidget(value, row, 1)

        layout.addLayout(status_grid)

        self.log_view = QPlainTextEdit(self)
        self.log_view.setReadOnly(True)
        self.log_view.setPlaceholderText("Service logs and request flow")
        layout.addWidget(self.log_view, 1)

        self.setCentralWidget(root)

    def _build_profile_selector(self):
        from PySide6.QtWidgets import QComboBox

        selector = QComboBox(self)
        self._refresh_profile_selector(selector)
        return selector

    def _connect_signals(self) -> None:
        self.start_service_button.clicked.connect(self.client.start)
        self.connect_button.clicked.connect(self.client.connect_hardware)
        self.handshake_button.clicked.connect(self.client.get_handshake)
        self.start_telemetry_button.clicked.connect(self.client.start_telemetry)
        self.stop_telemetry_button.clicked.connect(self.client.stop_telemetry)
        self.save_settings_button.clicked.connect(self.client.save_settings)
        self.reset_center_button.clicked.connect(self.client.reset_center)
        self.reboot_button.clicked.connect(self.client.reboot_device)
        self.dfu_button.clicked.connect(self.client.switch_to_dfu)

        self.profiles_page.save_requested.connect(self._save_profile)
        self.profiles_page.load_requested.connect(self._load_profile_into_pages)
        self.profiles_page.apply_requested.connect(self._apply_profile)

        self.client.connected_changed.connect(self._on_service_state_changed)
        self.client.log_message.connect(self._append_log)
        self.client.request_failed.connect(self._on_request_failed)
        self.client.response_received.connect(self._on_response_received)
        self.client.telemetry_status.connect(self._on_telemetry_status)
        self.client.telemetry_disconnected.connect(self._on_telemetry_disconnected)
        self.drive_page.apply_requested.connect(self._apply_updates)
        self.wheel_setup_page.apply_requested.connect(self._apply_updates)
        self.advanced_page.apply_requested.connect(self._apply_updates)

    def closeEvent(self, event) -> None:  # type: ignore[override]
        self.client.stop()
        super().closeEvent(event)

    def _append_log(self, message: str) -> None:
        self.log_view.appendPlainText(message)

    def _on_service_state_changed(self, running: bool) -> None:
        self.service_state.setText("Running" if running else "Stopped")

    def _on_request_failed(self, method: str, error: str) -> None:
        self._append_log(f"{method} failed: {error}")

    def _on_response_received(self, method: str, result: Any) -> None:
        if method == "get_handshake" and isinstance(result, dict):
            handshake = result.get("Handshake", {})
            self._handshake = handshake
            status = handshake.get("status", {})
            self.drive_page.set_values(handshake.get("fx", {}))
            self.wheel_setup_page.set_values(handshake.get("hw", {}))
            self.advanced_page.set_values(
                handshake.get("hw", {})
                | self._flatten_gpio(handshake.get("gpio", {}))
                | self._flatten_adc(handshake.get("adc", {}))
            )
            self._update_status(status)
            self._append_log("Wheel setup loaded")
            return

        if method == "connect_hardware":
            self.device_state.setText("Connected")
            return

        if method == "start_telemetry":
            self._append_log("Telemetry streaming enabled")
            return

        if method == "stop_telemetry":
            self._append_log("Telemetry streaming disabled")
            return

        if method == "save_settings":
            self._append_log("Settings saved to device")
            return

        if method == "reset_center":
            self._append_log("Wheel center reset")
            return

        if method == "reboot_device":
            self._append_log("Wheel reboot requested")
            return

        if method == "switch_to_dfu":
            self._append_log("DFU mode requested")
            return

        if method.startswith("update_"):
            self._append_log(f"{method} applied")
            return

        if method == "get_status" and isinstance(result, dict):
            self._update_status(result.get("Status", {}))

    def _on_telemetry_status(self, status: dict) -> None:
        self.device_state.setText("Connected")
        self._update_status(status)

    def _on_telemetry_disconnected(self, reason: str) -> None:
        self.device_state.setText("Disconnected")
        self._append_log(f"Telemetry disconnected: {reason}")

    def _update_status(self, status: dict) -> None:
        firmware = status.get("firmware", {})
        version = ".".join(str(firmware.get(part, "-")) for part in ("major", "minor", "patch"))
        self.firmware_value.setText(version)
        self.position_value.setText(str(status.get("position", "-")))
        self.torque_value.setText(str(status.get("torque", "-")))
        buttons = status.get("buttons", 0)
        self.buttons_value.setText(hex(buttons) if isinstance(buttons, int) else str(buttons))

    def _apply_updates(self, page_name: str, updates: list[dict]) -> None:
        for update in updates:
            payload = update["payload"]["UpdateHardwareField"]
            method = f"update_{page_name.lower().replace(' ', '_')}_{update['name']}"
            self.client.update_field(method, payload["field_id"], payload["data"], payload["index"])
        self._append_log(f"{page_name} updates queued")

    def _refresh_profile_selector(self, selector=None) -> None:
        selector = selector or self.profile_selector
        current = selector.currentText() if selector else ""
        selector.clear()
        selector.addItems(sorted(self._saved_profiles))
        if current:
            index = selector.findText(current)
            if index >= 0:
                selector.setCurrentIndex(index)

    def _save_profile(self) -> None:
        name, accepted = QInputDialog.getText(self, "Save Profile", "Profile name:")
        if not accepted or not name.strip():
            return
        profile_name = name.strip()
        self._saved_profiles[profile_name] = {
            "drive": self.drive_page.values(),
            "wheel_setup": self.wheel_setup_page.values(),
        }
        self._profiles.save_profiles(self._saved_profiles)
        self._refresh_profile_selector()
        self.profile_selector.setCurrentText(profile_name)
        self._append_log(f"Saved profile: {profile_name}")

    def _load_profile_into_pages(self) -> None:
        profile_name = self.profile_selector.currentText()
        profile = self._saved_profiles.get(profile_name)
        if not profile:
            self._append_log("No saved profile selected")
            return
        self.drive_page.set_values(profile.get("drive", {}))
        self.wheel_setup_page.set_values(profile.get("wheel_setup", {}))
        self._append_log(f"Loaded profile into pages: {profile_name}")

    def _apply_profile(self) -> None:
        profile_name = self.profile_selector.currentText()
        profile = self._saved_profiles.get(profile_name)
        if not profile:
            self._append_log("No saved profile selected")
            return
        self.drive_page.set_values(profile.get("drive", {}))
        self.wheel_setup_page.set_values(profile.get("wheel_setup", {}))
        self._apply_updates("Drive", self.drive_page.collect_updates())
        self._apply_updates("Wheel Setup", self.wheel_setup_page.collect_updates())
        self._append_log(f"Applied profile to device: {profile_name}")

    def _flatten_gpio(self, gpio: dict[str, Any]) -> dict[str, int]:
        return {
            "extension_mode": int(gpio.get("extension_mode", 0)),
            "spi_mode": int(gpio.get("spi_mode", 0)),
            "spi_latch_mode": int(gpio.get("spi_latch_mode", 0)),
            "spi_latch_delay": int(gpio.get("spi_latch_delay", 0)),
            "spi_clk_pulse_length": int(gpio.get("spi_clk_pulse_length", 0)),
        }

    def _flatten_adc(self, adc: dict[str, Any]) -> dict[str, int]:
        flattened: dict[str, int] = {}
        for prefix, values in (
            ("raxis_min", adc.get("raxis_min", [])),
            ("raxis_max", adc.get("raxis_max", [])),
            ("raxis_to_button_low", adc.get("raxis_to_button_low", [])),
            ("raxis_to_button_high", adc.get("raxis_to_button_high", [])),
            ("raxis_smoothing", adc.get("raxis_smoothing", [])),
            ("raxis_invert", adc.get("raxis_invert", [])),
        ):
            if isinstance(values, list):
                for index, value in enumerate(values[:3]):
                    flattened[f"{prefix}_{index}"] = int(value)
        return flattened


def main() -> int:
    app = QApplication(sys.argv)
    window = MainWindow()
    window.show()
    return app.exec()


if __name__ == "__main__":
    raise SystemExit(main())
