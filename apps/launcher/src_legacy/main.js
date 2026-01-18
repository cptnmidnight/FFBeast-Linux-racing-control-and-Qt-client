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
    // Fetch both saved games and detect installed games
    const [savedGames, scannedGames] = await Promise.all([
      invoke("get_games"),
      invoke("scan_games").catch(e => {
        console.warn("Scan failed:", e);
        return [];
      })
    ]);

    // Create a map of games by ID, prioritizing saved games
    const gamesMap = new Map();

    // First add scanned games
    scannedGames.forEach(game => {
      gamesMap.set(game.id, { ...game, isScanned: true });
    });

    // Then overwrite/add saved games (persisted config takes precedence)
    savedGames.forEach(game => {
      gamesMap.set(game.id, { ...game, isScanned: false });
    });

    // Save to global for configureGame to access
    window.loadedGames = gamesMap;

    const games = Array.from(gamesMap.values());

    if (games.length === 0) {
      gameListEl.innerHTML = `
        <div class="card" style="grid-column: 1 / -1; opacity: 0.7; text-align: center; padding: 3rem;">
          <p>${t("library.no_games")}</p>
          <button onclick="invoke('scan_games').then(loadGames)" class="btn-secondary" style="margin-top: 1rem;">
            Retry Scan
          </button>
        </div>
      `;
      return;
    }

    gameListEl.innerHTML = games.map(game => {
      // Basic gradient placeholder if no cover
      const bgStyle = game.cover_path
        ? `background-image: url('${convertFileSrc(game.cover_path)}'); background-size: cover;`
        : `background: linear-gradient(135deg, #2b2b2b 0%, #1a1a1a 100%);`;

      return `
      <div class="card game-card" style="position: relative; overflow: hidden; padding: 0; min-height: 200px; display: flex; flex-direction: column;">
        <div class="game-cover" style="height: 120px; ${bgStyle} width: 100%;">
           ${!game.cover_path ? `<div style="display: flex; justify-content: center; align-items: center; height: 100%; color: #888; font-size: 3rem;">🎮</div>` : ''}
        </div>
        <div class="game-info" style="padding: 1rem; flex: 1; display: flex; flex-direction: column;">
          <h3 style="margin: 0 0 0.5rem 0; font-size: 1.1rem;">${game.name}</h3>
          
          <div style="margin-top: auto; display: flex; gap: 0.5rem;">
            <button onclick="launchGame('${game.id}')" class="btn-primary" style="flex: 1;">
              ${t("library.launch")}
            </button>
             <button onclick="configureGame('${game.id}')" class="btn-secondary" style="padding: 0 0.8rem;" title="Configure Profile">
              ⚙️
            </button>
          </div>
        </div>
        ${game.isScanned ? `<span class="badge" style="position: absolute; top: 8px; right: 8px; background: rgba(0,0,0,0.6); color: #fff; padding: 2px 6px; border-radius: 4px; font-size: 0.7rem;">Detected</span>` : ''}
      </div>
    `}).join("");
  } catch (e) {
    console.error("Error loading games:", e);
    gameListEl.innerHTML = `<div class="error-msg">Failed to load library: ${e}</div>`;
  }
}

// Helper to convert local paths to Tauri asset URLs if needed (for images)
function convertFileSrc(filePath) {
  return window.__TAURI__.core.convertFileSrc(filePath);
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

window.configureGame = (id) => {
  const game = window.loadedGames.get(id);
  if (!game) return;

  const modal = document.querySelector("#add-game-modal");
  document.querySelector("#game-name-input").value = game.name;
  document.querySelector("#game-path-input").value = game.path;

  if (game.wheel_profile) {
    document.querySelector("#game-range-input").value = game.wheel_profile.motion_range;
    document.querySelector("#game-force-input").value = game.wheel_profile.total_force;
  }

  modal.classList.add("active");
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

  // Use the ID from the name if creating new, or preserve ID if editing existing
  // Ideally we should have a hidden ID field, but for now let's derive or lookup
  let id = name.toLowerCase().replace(/\s+/g, '-');

  // If we are editing a known scanned game, preserve its ID
  // (Simple heuristic: checking if name matches any loaded game)
  if (window.loadedGames) {
    for (const [gId, g] of window.loadedGames.entries()) {
      if (g.name === name) {
        id = gId;
        break;
      }
    }
  }

  const game = {
    id: id,
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
