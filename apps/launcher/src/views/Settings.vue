<template>
  <div class="settings-view">
    <header>
      <h1>⚙️ Configurações</h1>
    </header>

    <div class="settings-container">
      <!-- Wheel Default Settings -->
      <BaseCard :title="$t('settings.wheel.title')">
        <BaseSlider 
          v-model="wheelSettings.motion_range"
          :label="$t('settings.wheel.motion_range')"
          :help="$t('settings.wheel.help_motion_range')"
          :min="180"
          :max="1440"
          :step="10"
          suffix="°"
        />
        
        <BaseSlider 
          v-model="wheelSettings.total_force"
          :label="$t('settings.wheel.total_force')"
          :help="$t('settings.wheel.help_total_force')"
          :min="0"
          :max="100"
          suffix="%"
        />

        <BaseSlider 
          v-model="wheelSettings.integrated_spring_strength"
          :label="$t('settings.wheel.integrated_spring')"
          :help="$t('settings.wheel.help_integrated_spring')"
          :min="0"
          :max="100"
          suffix="%"
        />

        <BaseSwitch
          v-model="wheelSettings.invert_game_force"
          :label="$t('settings.wheel.invert_force')"
          :help="$t('settings.wheel.help_invert_force')"
        />

        <template #footer>
          <button class="btn-primary" @click="saveWheelSettings">💾 {{ $t('settings.actions.save_wheel') }}</button>
          <button class="btn-secondary" @click="applyToHardware">⚡ {{ $t('settings.actions.apply_wheel') }}</button>
        </template>
      </BaseCard>

      <!-- Soft Stop -->
      <BaseCard :title="$t('settings.soft_stop.title')">
        <BaseSlider 
          v-model="wheelSettings.soft_stop_strength"
          :label="$t('settings.soft_stop.strength')"
          :help="$t('settings.soft_stop.help_strength')"
          :min="0"
          :max="100"
          suffix="%"
        />

        <BaseSlider 
          v-model="wheelSettings.soft_stop_range"
          :label="$t('settings.soft_stop.range')"
          :help="$t('settings.soft_stop.help_range')"
          :min="0"
          :max="255"
          suffix="°"
        />

        <BaseSlider 
          v-model="wheelSettings.soft_stop_dampening"
          :label="$t('settings.soft_stop.dampening')"
          :help="$t('settings.soft_stop.help_dampening')"
          :min="0"
          :max="1000"
          :step="10"
        />
      </BaseCard>

      <!-- Dampening -->
      <BaseCard :title="$t('settings.dampening.title')">
        <BaseSlider 
          v-model="wheelSettings.static_dampening_strength"
          :label="$t('settings.dampening.static')"
          :help="$t('settings.dampening.help_static')"
          :min="0"
          :max="1000"
          :step="10"
        />

        <BaseSlider 
          v-model="wheelSettings.dynamic_dampening_strength"
          :label="$t('settings.dampening.dynamic')"
          :help="$t('settings.dampening.help_dynamic')"
          :min="0"
          :max="1000"
          :step="10"
        />
      </BaseCard>

      <!-- DirectX Effects -->
      <BaseCard :title="$t('settings.directx.title')">
        <BaseSlider 
          v-model="wheelSettings.direct_x_constant"
          :label="$t('settings.directx.constant')"
          :help="$t('settings.directx.help_constant')"
          :min="0"
          :max="100"
          suffix="%"
        />

        <BaseSlider 
          v-model="wheelSettings.direct_x_periodic"
          :label="$t('settings.directx.periodic')"
          :help="$t('settings.directx.help_periodic')"
          :min="0"
          :max="100"
          suffix="%"
        />

        <BaseSlider 
          v-model="wheelSettings.direct_x_spring"
          :label="$t('settings.directx.spring')"
          :help="$t('settings.directx.help_spring')"
          :min="0"
          :max="100"
          suffix="%"
        />
      </BaseCard>

      <!-- Gamepad Mapping -->
      <BaseCard :title="$t('settings.gamepad.title')">
        <div class="axis-grid">
          <div class="axis-item">
            <label>
              <input type="checkbox" v-model="gamepadMapping.axis_x_enabled" />
              {{ $t('settings.gamepad.axis_x') }} {{ $t('settings.gamepad.enabled') }}
            </label>
            <label v-if="gamepadMapping.axis_x_enabled">
              <input type="checkbox" v-model="gamepadMapping.axis_x_inverted" />
              {{ $t('settings.gamepad.inverted') }}
            </label>
          </div>

          <div class="axis-item">
            <label>
              <input type="checkbox" v-model="gamepadMapping.axis_y_enabled" />
              {{ $t('settings.gamepad.axis_y') }} {{ $t('settings.gamepad.enabled') }}
            </label>
            <label v-if="gamepadMapping.axis_y_enabled">
              <input type="checkbox" v-model="gamepadMapping.axis_y_inverted" />
              {{ $t('settings.gamepad.inverted') }}
            </label>
          </div>

          <div class="axis-item">
            <label>
              <input type="checkbox" v-model="gamepadMapping.axis_z_enabled" />
              {{ $t('settings.gamepad.axis_z') }} {{ $t('settings.gamepad.enabled') }}
            </label>
            <label v-if="gamepadMapping.axis_z_enabled">
              <input type="checkbox" v-model="gamepadMapping.axis_z_inverted" />
              {{ $t('settings.gamepad.inverted') }}
            </label>
          </div>

          <div class="axis-item">
            <label>
              <input type="checkbox" v-model="gamepadMapping.axis_rz_enabled" />
              {{ $t('settings.gamepad.axis_rz') }} {{ $t('settings.gamepad.enabled') }}
            </label>
            <label v-if="gamepadMapping.axis_rz_enabled">
              <input type="checkbox" v-model="gamepadMapping.axis_rz_inverted" />
              {{ $t('settings.gamepad.inverted') }}
            </label>
          </div>
        </div>

        <BaseSlider 
          v-model="gamepadMapping.deadzone"
          :label="$t('settings.gamepad.deadzone')"
          :help="$t('settings.gamepad.help_deadzone')"
          :min="0"
          :max="1"
          :step="0.01"
        />

        <template #footer>
          <button class="btn-primary" @click="saveGamepadMapping">💾 {{ $t('settings.gamepad.save') }}</button>
        </template>
      </BaseCard>

      <!-- Keyboard Mapping Service -->
      <BaseCard title="⌨️ Serviço de Mapeamento (Teclado)">
        <div class="service-status">
          <span class="status-dot" :class="{ active: serviceActive }"></span>
          <span class="status-text">Status: {{ serviceActive ? 'Ativo' : 'Inativo' }}</span>
        </div>

        <div class="card-actions">
           <button class="btn-primary" @click="startService" :disabled="serviceActive">▶️ Iniciar</button>
           <button class="btn-secondary" @click="stopService" :disabled="!serviceActive">⏹️ Parar</button>
           <button class="btn-secondary" @click="restartService">🔄 Reiniciar</button>
        </div>

        <p class="desc">
            O serviço de mapeamento converte os eixos e botões do volante em teclas do teclado ou botões virtuais,
            permitindo jogar jogos que não tem suporte nativo a Force Feedback.
        </p>
      </BaseCard>

      <!-- Axis Mapping Component -->
      <AxisMapping />

      <!-- UI Customization -->
      <BaseCard :title="$t('settings.ui.title')">
        <div class="setting-row">
          <label>{{ $t('settings.ui.primary_color') }}</label>
          <div class="color-picker">
            <div 
              v-for="color in accentColors" 
              :key="color"
              class="color-dot"
              :style="{ background: color }"
              @click="setAccentColor(color)"
            ></div>
          </div>
        </div>

        <BaseSlider 
          v-model="fontSize"
          :label="$t('settings.ui.font_size')"
          :help="$t('settings.ui.help_font_size')"
          :min="12"
          :max="20"
          suffix="px"
          @update:modelValue="applyFontSize"
        />
      </BaseCard>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, onUnmounted } from 'vue';
import { SettingsService } from '../services/tauri';
import type { DefaultWheelSettings, GamepadAxisMapping } from '../types/settings';
import BaseCard from '../components/common/BaseCard.vue';
import BaseSlider from '../components/common/BaseSlider.vue';
import BaseSwitch from '../components/common/BaseSwitch.vue';
import AxisMapping from '../components/AxisMapping.vue';
import { invoke } from '@tauri-apps/api/core';

export default defineComponent({
  name: 'SettingsView',
  components: {
    BaseCard,
    BaseSlider,
    BaseSwitch,
    AxisMapping
  },
  setup() {
    const serviceActive = ref(false);
    
    // Check service status periodically
    const checkServiceStatus = async () => {
        try {
            serviceActive.value = await invoke('keyboard_service_is_active');
        } catch (e) {
            console.error(e);
        }
    };
    
    let statusInterval: number;

    const startService = async () => {
        await invoke('keyboard_service_start');
        await checkServiceStatus();
    };

    const stopService = async () => {
        await invoke('keyboard_service_stop');
        await checkServiceStatus();
    };

    const restartService = async () => {
        await invoke('keyboard_service_restart');
        await checkServiceStatus();
    };

    const wheelSettings = ref<DefaultWheelSettings>({
      // General Settings
      motion_range: 900,
      total_force: 100,

      // Spring & Dampening
      integrated_spring_strength: 100,
      static_dampening_strength: 0,
      dynamic_dampening_strength: 0,

      // Soft Stop
      soft_stop_strength: 100,
      soft_stop_range: 10,
      soft_stop_dampening: 0,

      // DirectX Effects
      direct_x_constant: 100,
      direct_x_periodic: 100,
      direct_x_spring: 100,

      // Inversion
      invert_game_force: false,

      // Legacy
      power_limit: 100,
      braking_limit: 100
    });

    const gamepadMapping = ref<GamepadAxisMapping>({
      axis_x_enabled: true,
      axis_x_inverted: false,
      axis_y_enabled: true,
      axis_y_inverted: false,
      axis_z_enabled: true,
      axis_z_inverted: false,
      axis_rz_enabled: true,
      axis_rz_inverted: false,
      deadzone: 0.1
    });

    const fontSize = ref(16);
    const accentColors = ['#8b5cf6', '#d946ef', '#ec4899', '#f59e0b', '#10b981', '#06b6d4'];

    const loadSettings = async () => {
      try {
        wheelSettings.value = await SettingsService.getDefaultWheelSettings();
        gamepadMapping.value = await SettingsService.getGamepadMapping();
        
        // Load UI settings from localStorage
        const savedFontSize = localStorage.getItem('launcher_font_size');
        if (savedFontSize) fontSize.value = parseInt(savedFontSize);
      } catch (e) {
        console.error('Failed to load settings:', e);
      }
    };

    const saveWheelSettings = async () => {
      try {
        await SettingsService.saveDefaultWheelSettings(wheelSettings.value);
        alert('Configurações do volante salvas!');
      } catch (e) {
        alert('Erro ao salvar: ' + e);
      }
    };

    const applyToHardware = async () => {
      try {
        await SettingsService.applyWheelSettingsToHardware(wheelSettings.value);
        alert('Configurações aplicadas ao volante!');
      } catch (e) {
        alert('Erro ao aplicar: ' + e);
      }
    };

    const saveGamepadMapping = async () => {
      try {
        await SettingsService.saveGamepadMapping(gamepadMapping.value);
        alert('Mapeamento salvo!');
      } catch (e) {
        alert('Erro ao salvar: ' + e);
      }
    };

    const setAccentColor = (color: string) => {
      document.documentElement.style.setProperty('--primary-color', color);
      localStorage.setItem('launcher_accent_color', color);
    };

    const applyFontSize = () => {
      document.documentElement.style.fontSize = `${fontSize.value}px`;
      localStorage.setItem('launcher_font_size', fontSize.value.toString());
    };

    onMounted(() => {
      loadSettings();
      checkServiceStatus();
      statusInterval = window.setInterval(checkServiceStatus, 2000);
    });

    onUnmounted(() => {
        if (statusInterval) clearInterval(statusInterval);
    });

    return {
      wheelSettings,
      gamepadMapping,
      fontSize,
      accentColors,
      saveWheelSettings,
      applyToHardware,
      saveGamepadMapping,
      setAccentColor,
      applyFontSize,
      // Keyboard Service
      serviceActive,
      startService,
      stopService,
      restartService
    };
  }
});
</script>

<style scoped>
.settings-view {
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
}

header h1 {
  margin-bottom: 2rem;
  color: var(--text-color);
}

.settings-container {
  display: flex;
  flex-direction: column;
  gap: 0;
}

.setting-row {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.setting-row label {
  flex: 1;
  font-weight: 600;
  font-size: 0.9rem;
  color: var(--text-color);
}

.axis-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.axis-item {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  padding: 1rem;
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid var(--border-color);
  border-radius: 8px;
}

.axis-item label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.85rem;
}

.axis-item input[type="checkbox"] {
  accent-color: var(--primary-color);
}

.color-picker {
  display: flex;
  gap: 1rem;
  flex: 2;
}

.color-dot {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  cursor: pointer;
  border: 2px solid transparent;
  transition: transform 0.2s;
}

.color-dot:hover {
  transform: scale(1.2);
  border-color: var(--text-color);
}

.btn-primary, .btn-secondary {
  flex: 1;
  padding: 0.8rem 1.5rem;
  border: none;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  background: var(--primary-color);
  color: white;
}

.btn-primary:hover {
  background: var(--primary-hover);
  transform: translateY(-1px);
}

.btn-secondary {
  background: transparent;
  border: 1px solid var(--primary-color);
  color: var(--primary-color);
}

.btn-secondary:hover {
  background: var(--primary-color);
  color: white;
  transform: translateY(-1px);
}

.card-actions {
  display: flex;
  gap: 1rem;
  margin-top: 1.5rem;
  margin-bottom: 1rem;
}

.service-status {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 1rem;
  font-weight: 600;
  font-size: 1.1rem;
}

.status-dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: #ef4444; /* Red */
  box-shadow: 0 0 10px rgba(239, 68, 68, 0.5);
}

.status-dot.active {
  background: #10b981; /* Green */
  box-shadow: 0 0 10px rgba(16, 185, 129, 0.5);
}

.desc {
    opacity: 0.7;
    font-size: 0.9rem;
    line-height: 1.5;
}

.btn-primary:disabled, .btn-secondary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  pointer-events: none;
}
</style>
