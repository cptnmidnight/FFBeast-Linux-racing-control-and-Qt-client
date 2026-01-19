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
        <BaseSlider 
          v-model="fontSize" 
          :label="$t('settings_font_size')" 
          :min="12" 
          :max="24" 
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
            <span class="info-value">Windows (Tauri)</span>
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
import BaseCard from '../common/BaseCard.vue';
import BaseSelect from '../common/BaseSelect.vue';
import BaseSlider from '../common/BaseSlider.vue';

const { locale } = useI18n();

const language = ref(locale.value);
const fontSize = ref(16);

const langOptions = [
  { label: 'English', value: 'en' },
  { label: 'Português (Brasil)', value: 'pt-BR' },
];

const accentColors = [
  '#00d4ff', '#ff4757', '#00ff88', '#ffa502', '#e056fd'
];

const changeLang = (val: string) => {
  locale.value = val;
};

const setAccent = (color: string) => {
  document.documentElement.style.setProperty('--accent', color);
  document.documentElement.style.setProperty('--accent-glow', color + '66');
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
