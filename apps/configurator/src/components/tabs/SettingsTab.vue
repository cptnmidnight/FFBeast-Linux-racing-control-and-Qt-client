<template>
  <div class="settings-tab">
    <div class="grid-layout">
      <!-- UI Settings -->
      <BaseCard :title="$t('settings_ui')">
        <BaseSelect 
          v-model="language" 
          :options="langOptions" 
          :label="$t('setting_language')"
          @update:model-value="changeLang"
        />
        
        <BaseSelect 
          v-model="uiStore.settings.fontFamily" 
          :options="fontOptions" 
          :label="$t('setting_ui_font')"
          @update:model-value="applyFont"
        />
        
        <BaseSlider 
          v-model="uiStore.settings.fontSize" 
          :label="$t('settings_font_size')" 
          :min="12" 
          :max="24" 
          @update:model-value="applyFontSize"
        />
        
        <div class="theme-selector">
          <label>{{ $t('settings_accent_color') }}</label>
          <div class="color-grid">
            <div 
              v-for="color in accentColors" 
              :key="color" 
              class="color-dot"
              :style="{ background: color }"
              @click="setAccent(color)"
            ></div>
          </div>
        </div>
      </BaseCard>

      <!-- Toast Settings -->
      <BaseCard :title="$t('setting_toast_notifications')">
        <BaseSelect 
          v-model="uiStore.settings.toastPosition" 
          :options="toastPositionOptions" 
          :label="$t('setting_toast_position')"
          :use-i18n="true"
        />
        
        <BaseSlider 
          v-model="uiStore.settings.toastMargin" 
          :label="$t('setting_toast_margin')" 
          :min="10" 
          :max="100" 
        />
        
        <button class="btn-test" @click="testToast">{{ $t('btn_test_toast') }}</button>
      </BaseCard>

      <!-- Advanced Settings -->
      <BaseCard :title="$t('settings_advanced') || 'Advanced Settings'">
        <div class="control-group">
          <div class="label-col">
            <span class="label">{{ $t('setting_min_log_level') }}</span>
            <span class="desc">{{ $t('setting_min_log_level_desc') }}</span>
          </div>
          <BaseSelect 
            v-model="uiStore.settings.minLogLevel" 
            :options="logLevelOptions"
            :use-i18n="true"
            class="compact-select"
            @update:model-value="uiStore.setMinLogLevel"
          />
        </div>
        
        <div class="control-group">
          <div class="label-col">
            <span class="label">{{ $t('setting_debug_mode') }}</span>
            <span class="desc">{{ $t('setting_debug_desc') }}</span>
          </div>
          <BaseSwitch 
            v-model="uiStore.settings.debugMode" 
            @update:model-value="uiStore.toggleDebugMode"
          />
        </div>
      </BaseCard>

      <!-- App Info -->
      <BaseCard :title="$t('app_info')">
        <div class="info-grid">
          <div class="info-row">
            <span class="info-label">{{ $t('settings_version') }}</span>
            <span class="info-value">v{{ versions.app }}</span>
          </div>
          <div class="info-row">
            <span class="info-label">Controller SDK</span>
            <span class="info-value">v{{ versions.controller }}</span>
          </div>
          <div class="info-row">
            <span class="info-label">{{ $t('setting_build_date') }}</span>
            <span class="info-value">2026-01-20</span>
          </div>
          <div class="info-row">
            <span class="info-label">{{ $t('setting_platform') }}</span>
            <span class="info-value">Linux (Tauri)</span>
          </div>
        </div>
        <template #footer>
          <div class="footer-links">
            <a href="#">{{ $t('link_github') }}</a>
            <a href="#">{{ $t('link_docs') }}</a>
          </div>
        </template>
      </BaseCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { useUIStore } from '../../stores/ui';
import { HardwareService } from '../../services/hardware_service';
import BaseCard from '../common/BaseCard.vue';
import BaseSelect from '../common/BaseSelect.vue';
import BaseSlider from '../common/BaseSlider.vue';
import BaseSwitch from '../common/BaseSwitch.vue';

const { locale } = useI18n();
const uiStore = useUIStore();

const language = ref(locale.value);
const versions = ref({ app: 'Loading...', controller: 'Loading...' });

onMounted(async () => {
  try {
    const v = await HardwareService.getVersions();
    versions.value = v;
  } catch (e) {
    console.error('Failed to get versions:', e);
    versions.value = { app: 'Unknown', controller: 'Unknown' };
  }
});

const langOptions = [
  { label: 'English', value: 'en' },
  { label: 'Português (Brasil)', value: 'pt-BR' },
];

const fontOptions = [
  { label: 'Outfit', value: 'Outfit' },
  { label: 'JetBrains Mono', value: 'JetBrains Mono' },
  { label: 'System Default', value: 'system-ui' },
];

const toastPositionOptions = [
  { label: 'toast_pos_top_right', value: 'top-right' },
  { label: 'toast_pos_bottom_right', value: 'bottom-right' },
  { label: 'toast_pos_top_left', value: 'top-left' },
  { label: 'toast_pos_bottom_left', value: 'bottom-left' },
];

const logLevelOptions = [
  { label: 'log_level_error', value: 1 },
  { label: 'log_level_warn', value: 2 },
  { label: 'log_level_info', value: 3 },
  { label: 'log_level_debug', value: 4 },
  { label: 'log_level_trace', value: 5 },
];

// Avoid semantic colors (success, error, warning)
const accentColors = [
  '#00d4ff', // Cyan
  '#e056fd', // Purple
  '#0088ff', // Blue
  '#00ccaa', // Teal (distinct from success)
  '#ff6b9d', // Pink (distinct from error)
];

const changeLang = (val: string) => {
  locale.value = val;
  localStorage.setItem('ffbeast_language', val);
};

const applyFont = (val: string) => {
  document.documentElement.style.setProperty('--font-main', val);
  localStorage.setItem('ffbeast_font', val);
};

const applyFontSize = (val: number) => {
  document.documentElement.style.fontSize = `${val}px`;
  localStorage.setItem('ffbeast_font_size', String(val));
};

const setAccent = (color: string) => {
  document.documentElement.style.setProperty('--accent', color);
  document.documentElement.style.setProperty('--accent-glow', color + '66');
  localStorage.setItem('ffbeast_accent', color);
};

const testToast = () => {
  console.log('[SettingsTab] Test toast button clicked');
  uiStore.showToast('This is a test toast notification! 🎉', 'success');
};
</script>

<style scoped>
.settings-tab {
  padding: var(--content-padding);
}

.grid-layout {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 1.5rem;
}

.control-group {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
}

.label-col {
  display: flex;
  flex-direction: column;
}

.label-col .label {
  font-size: 0.9rem;
  font-weight: 500;
  color: var(--text-main);
}

.label-col .desc {
  font-size: 0.75rem;
  color: var(--text-dim);
}

.compact-select {
  width: 220px;
  margin-bottom: 0 !important;
}

.theme-selector {
  margin-top: 1.5rem;
}

.theme-selector label {
  font-size: 0.85rem;
  color: var(--text-dim);
  display: block;
  margin-bottom: 10px;
}

.color-grid {
  display: flex;
  gap: 12px;
}

.color-dot {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  cursor: pointer;
  border: 2px solid transparent;
  transition: transform 0.2s;
}

.color-dot:hover {
  transform: scale(1.2);
}

.info-grid {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.info-row {
  display: flex;
  justify-content: space-between;
  font-size: 0.9rem;
}

.info-label {
  color: var(--text-dim);
}

.info-value {
  font-family: var(--font-mono);
  color: var(--text-main);
}

.footer-links {
  display: flex;
  gap: 20px;
}

.footer-links a {
  color: var(--accent);
  font-size: 0.85rem;
  text-decoration: none;
}

.footer-links a:hover {
  text-decoration: underline;
}

.btn-test {
  margin-top: 1rem;
  padding: 10px 20px;
  background: var(--accent);
  color: #000;
  border: none;
  border-radius: var(--radius-sm);
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-test:hover {
  filter: brightness(1.1);
  box-shadow: 0 0 15px var(--accent-glow);
}
</style>
