const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

let currentLanguage = 'en';
let i18nData = {};

async function loadI18n() {
  // Tries to detect system language or defaults to English
  const savedLang = localStorage.getItem('app-lang');
  currentLanguage = savedLang || (navigator.language.startsWith('pt') ? 'pt' : 'en');

  const selector = document.querySelector("#lang-selector");
  if (selector) selector.value = currentLanguage;

  try {
    const response = await fetch(`/i18n/${currentLanguage}.json`);
    i18nData = await response.json();
    applyI18n();
  } catch (e) {
    console.error("Failed to load i18n:", e);
  }
}

window.changeLanguage = async (lang) => {
  localStorage.setItem('app-lang', lang);
  await loadI18n();
  updateHardwareStatus(); // Refresh badge text
};

function t(key) {
  const keys = key.split('.');
  let value = i18nData;
  for (const k of keys) {
    value = value ? value[k] : null;
  }
  return value || key;
}

function applyI18n() {
  document.querySelectorAll('[data-i18n]').forEach(el => {
    const key = el.getAttribute('data-i18n');
    el.textContent = t(key);
  });
}

async function updateHardwareStatus() {
  const statusEl = document.querySelector("#status-display");
  if (!statusEl) return;

  try {
    const isConnected = await invoke("check_hardware");
    if (isConnected) {
      statusEl.textContent = t("app.connected");
      statusEl.className = "hardware-badge connected";
    } else {
      statusEl.textContent = t("app.searching");
      statusEl.className = "hardware-badge searching";
    }
  } catch (e) {
    console.error("Error checking hardware:", e);
  }
}

async function setupEventListeners() {
  await listen("wheel-status", (event) => {
    const status = event.payload;
    const posValEl = document.querySelector("#pos-val");
    const torqueValEl = document.querySelector("#torque-val");
    const visualEl = document.querySelector("#wheel-visual-inner");

    if (posValEl) posValEl.textContent = status.position;
    if (torqueValEl) torqueValEl.textContent = status.torque;

    if (visualEl) {
      const rotation = (status.position / 32768) * 450;
      visualEl.style.transform = `rotate(${rotation}deg)`;
    }
  });
}

async function loadGames() {
  const gameListEl = document.querySelector("#game-list");
  if (!gameListEl) return;

  try {
    const games = await invoke("get_games");
    if (games.length === 0) {
      gameListEl.innerHTML = `
        <div class="card" style="grid-column: 1 / -1; opacity: 0.7;">
          <p>${t("library.no_games")}</p>
        </div>
      `;
      return;
    }

    gameListEl.innerHTML = games.map(game => `
      <div class="card">
        <h3>${game.name}</h3>
        <p style="font-size: 0.8rem; opacity: 0.6; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
          ${game.path}
        </p>
        <button onclick="launchGame('${game.id}')" style="margin-top: 1rem; width: 100%;">
          ${t("library.launch")}
        </button>
      </div>
    `).join("");
  } catch (e) {
    console.error("Error loading games:", e);
  }
}

window.invokeResetCenter = async () => {
  try {
    await invoke("reset_center");
    console.log("Center reset");
  } catch (e) {
    alert("Error resetting: " + e);
  }
};

window.invokeReboot = async () => {
  if (confirm(t("monitor.confirm_reboot"))) {
    try {
      await invoke("reboot_device");
      console.log("Device rebooting...");
    } catch (e) {
      alert("Error rebooting: " + e);
    }
  }
};

window.launchGame = async (id) => {
  try {
    await invoke("launch_game", { id });
    console.log("Game launched:", id);
  } catch (e) {
    alert("Error launching game: " + e);
  }
};

window.openAdvancedConfig = async () => {
  try {
    await invoke("open_advanced_config");
  } catch (e) {
    console.error("Failed to open advanced config:", e);
  }
};

window.openAddGameModal = () => {
  document.querySelector("#add-game-modal").classList.add("active");
};

window.closeAddGameModal = () => {
  document.querySelector("#add-game-modal").classList.remove("active");
  document.querySelector("#game-name-input").value = "";
  document.querySelector("#game-path-input").value = "";
};

window.saveNewGame = async () => {
  const name = document.querySelector("#game-name-input").value;
  const path = document.querySelector("#game-path-input").value;
  const range = parseInt(document.querySelector("#game-range-input").value);
  const force = parseInt(document.querySelector("#game-force-input").value);

  if (!name || !path) {
    alert(t("app.fill_all_fields") || "Please fill all fields");
    return;
  }

  const game = {
    id: name.toLowerCase().replace(/\s+/g, '-'),
    name: name,
    path: path,
    arguments: [],
    environment_vars: {},
    dll_overrides: [],
    wheel_profile: {
      motion_range: range,
      total_force: force
    },
    use_compat_layer: false
  };

  try {
    await invoke("save_game", { game });
    closeAddGameModal();
    await loadGames();
  } catch (e) {
    alert("Error saving game: " + e);
  }
};

let debounceTimer;
window.debounceUpdateSettings = () => {
  const force = parseInt(document.querySelector("#total-force-slider").value);
  const range = parseInt(document.querySelector("#motion-range-slider").value);
  const damp = parseInt(document.querySelector("#dynamic-damp-slider").value);

  document.querySelector("#total-force-val").textContent = `${force}%`;
  document.querySelector("#motion-range-val").textContent = `${range}°`;
  document.querySelector("#dynamic-damp-val").textContent = damp;

  clearTimeout(debounceTimer);
  debounceTimer = setTimeout(async () => {
    try {
      // Create settings object matching the Rust struct EffectSettings
      await invoke("update_effect_settings", {
        settings: {
          motion_range: range,
          static_dampening_strength: 0,
          soft_stop_dampening_strength: 0,
          total_effect_strength: force,
          integrated_spring_strength: 100,
          soft_stop_range: 10,
          soft_stop_strength: 100,
          direct_x_constant_direction: 0,
          direct_x_spring_strength: 100,
          direct_x_constant_strength: 100,
          direct_x_periodic_strength: 100,
          dynamic_dampening_strength: damp
        }
      });

      // Create settings object matching the Rust struct HardwareSettings
      await invoke("update_hardware_settings", {
        settings: {
          encoder_cpr: 10000, // Placeholder or from some config
          integral_gain: 100,
          proportional_gain: 50,
          force_enabled: 1,
          debug_torque: 0,
          amplifier_gain: 1,
          calibration_magnitude: 50,
          calibration_speed: 10,
          power_limit: 100,
          braking_limit: 100,
          position_smoothing: 1,
          speed_buffer_size: 5,
          encoder_direction: 1,
          force_direction: 1,
          pole_pairs: 22
        }
      });
      console.log("Hardware settings updated");
    } catch (e) {
      console.error("Failed to update hardware settings:", e);
    }
  }, 200);
};

window.addEventListener("DOMContentLoaded", async () => {
  await loadI18n();
  updateHardwareStatus();
  loadGames();
  setupEventListeners();

  setInterval(updateHardwareStatus, 3000);
});
