<template>
  <div class="settings-tab">
    <div class="grid-layout">
      <!-- UI Settings -->
      <BaseCard :title="$t('settings_ui')">
        <BaseSelect 
          v-model="language" 
          :options="langOptions" 
          label="Interface Language"
          @update:model-value="changeLang"
        />
        
        <BaseSelect 
          v-model="uiStore.settings.fontFamily" 
          :options="fontOptions" 
          label="UI Font Family"
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
      <BaseCard title="Toast Notifications">
        <BaseSelect 
          v-model="uiStore.settings.toastPosition" 
          :options="toastPositionOptions" 
          label="Toast Position"
        />
        
        <BaseSlider 
          v-model="uiStore.settings.toastMargin" 
          label="Toast Margin (px)" 
          :min="10" 
          :max="100" 
        />
      </BaseCard>

      <!-- App Info -->
      <BaseCard :title="$t('app_info')">
        <div class="info-grid">
          <div class="info-row">
            <span class="info-label">{{ $t('settings_version') }}</span>
            <span class="info-value">v2.0.0-beta</span>
          </div>
          <div class="info-row">
            <span class="info-label">Build Date</span>
            <span class="info-value">2026-01-18</span>
          </div>
          <div class="info-row">
            <span class="info-label">Platform</span>
            <span class="info-value">Linux (Tauri)</span>
          </div>
        </div>
        <template #footer>
          <div class="footer-links">
            <a href="#">Github Repository</a>
            <a href="#">Documentation</a>
          </div>
        </template>
      </BaseCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useUIStore } from '../../stores/ui';
import BaseCard from '../common/BaseCard.vue';
import BaseSelect from '../common/BaseSelect.vue';
import BaseSlider from '../common/BaseSlider.vue';

const { locale } = useI18n();
const uiStore = useUIStore();

const language = ref(locale.value);

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
  { label: 'Top Right', value: 'top-right' },
  { label: 'Bottom Right', value: 'bottom-right' },
  { label: 'Top Left', value: 'top-left' },
  { label: 'Bottom Left', value: 'bottom-left' },
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
</style>
