<template>
  <div class="settings-tab">
    <div class="grid-layout">
      <!-- UI Settings -->
      <BaseCard :title="$t('settings.ui')">
        <ThemedSelect 
          v-model="language" 
          :options="langOptions" 
          :label="$t('settings.language')"
          @change="changeLang"
        />
        
        <div class="font-controls">
          <ThemedSelect 
            v-model="uiStore.settings.fontFamily" 
            :options="fontOptions" 
            :label="$t('settings.ui_font')"
            class="font-select"
            @change="(v: string | number) => uiStore.setFontFamily(String(v))"
          />
          <button 
            class="btn-outline btn-small" 
            :disabled="isLoadingFonts"
            @click="loadSystemFonts"
            :title="$t('settings.load_system_fonts')"
          >
            <span v-if="isLoadingFonts" class="spinner"></span>
            <svg v-else width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M4 12C4 16.4183 7.58172 20 12 20C14.5376 20 16.8066 18.8184 18.2917 16.9667M19.9583 14C19.986 13.3469 20 12.6806 20 12C20 7.58172 16.4183 4 12 4C9.46237 4 7.19342 5.18165 5.70835 7.03328" stroke="var(--accent)" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M4.5 7H7.5V4" stroke="var(--accent)" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M19.5 17H16.5V20" stroke="var(--accent)" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </button>
        </div>
        
        <ThemedSlider 
          v-model="uiStore.settings.fontSize" 
          :label="$t('settings.font_size')" 
          :min="12" 
          :max="24" 
          @update:model-value="uiStore.setFontSize"
        />
        
        <div class="theme-selector">
          <label class="theme-selector__label">{{ $t('settings.accent_color') }}</label>
          <div class="color-grid">
            <div 
              v-for="color in accentColors" 
              :key="color" 
              class="color-dot"
              :style="{ background: color, borderColor: uiStore.settings.accentColor === color ? '#fff' : 'transparent' }"
              @click="uiStore.setAccentColor(color)"
            ></div>
            
            <!-- Custom Color Picker -->
            <div 
              class="color-dot custom-color"
              :style="{ background: 'conic-gradient(from 0deg, red, yellow, lime, aqua, blue, magenta, red)', borderColor: !accentColors.includes(uiStore.settings.accentColor) ? '#fff' : 'transparent' }"
              :title="$t('settings.custom_color') || 'Custom Color'"
            >
              <input 
                type="color" 
                class="color-input"
                :value="uiStore.settings.accentColor"
                @input="(e) => handleCustomColor((e.target as HTMLInputElement).value)"
              />
            </div>
          </div>
        </div>
      </BaseCard>

      <!-- Toast Settings -->
      <BaseCard :title="$t('settings.toasts.notifications')">
        <ThemedSelect 
          v-model="uiStore.settings.toastPosition" 
          :options="toastPositionOptions" 
          :label="$t('settings.toasts.position')"
        />
        
        <ThemedSlider 
          v-model="uiStore.settings.toastMargin" 
          :label="$t('settings.toasts.margin')" 
          :min="10" 
          :max="100" 
          value-suffix="px"
        />
        
        <button class="btn-test" @click="testToast">{{ $t('buttons.test_toast') }}</button>
      </BaseCard>

      <!-- Advanced Settings -->
      <BaseCard :title="$t('settings.advanced')">
        <div class="control-row">
          <div class="control-info">
            <span class="control-label">{{ $t('settings.min_log_level') }}</span>
            <span class="control-desc">{{ $t('settings.min_log_level_desc') }}</span>
          </div>
          <ThemedSelect 
            v-model="uiStore.settings.minLogLevel" 
            :options="logLevelOptions"
            class="compact-select"
            @change="(v: string | number) => uiStore.setMinLogLevel(Number(v))"
          />
        </div>
        
        <div class="control-row">
          <div class="control-info">
            <span class="control-label">{{ $t('settings.debug_mode') }}</span>
            <span class="control-desc">{{ $t('settings.debug_desc') }}</span>
          </div>
          <ThemedSwitch 
            v-model="uiStore.settings.debugMode" 
            @change="uiStore.toggleDebugMode"
          />
        </div>
      </BaseCard>

      <!-- App Info -->
      <BaseCard :title="$t('app.info')">
        <div class="info-list">
          <div class="info-item">
            <span class="info-item__label">{{ $t('settings.version') }}</span>
            <span class="info-item__value">v{{ versions.app }}</span>
          </div>
          <div class="info-item">
            <span class="info-item__label">Controller SDK</span>
            <span class="info-item__value">v{{ versions.controller }}</span>
          </div>
          <div class="info-item">
            <span class="info-item__label">{{ $t('settings.build_date') }}</span>
            <span class="info-item__value">2026-01-21</span>
          </div>
          <div class="info-item">
            <span class="info-item__label">{{ $t('settings.build_date') }}</span>
            <span class="info-item__value">2026-01-21</span>
          </div>
        </div>
        <template #footer>
          <div class="info-list">
            <div class="info-item">
              <span class="info-item__label">{{ $t('app.developer') }}</span>
              <span class="info-item__value">Osni Pezzini Junior</span>
            </div>
            <div class="info-item">
              <span class="info-item__label">{{ $t('app.contact') }}</span>
              <span class="info-item__value">osnipezzini@gmail.com</span>
            </div>
          </div>
        </template>
      </BaseCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useUIStore } from '../../stores/ui';
import { HardwareService } from '../../services/hardware_service';
import BaseCard from '../common/BaseCard.vue';
import ThemedSelect from '@shared/components/atoms/ThemedSelect.vue';
import ThemedSlider from '@shared/components/atoms/ThemedSlider.vue';
import ThemedSwitch from '@shared/components/atoms/ThemedSwitch.vue';

const { locale, t } = useI18n();
const uiStore = useUIStore();

const language = ref(locale.value);
const versions = ref({ app: '...', controller: '...' });

const fontOptions = ref([
  { label: 'System UI', value: 'system-ui' },
  { label: 'Sans Serif', value: 'sans-serif' },
  { label: 'Serif', value: 'serif' },
  { label: 'Monospace', value: 'monospace' },
]);

// Extended interface for FontData


const isLoadingFonts = ref(false);

// Extend window interface usually needs a .d.ts, but we can cast to any for now
const loadSystemFonts = async () => {
  if (isLoadingFonts.value) return;
  
  isLoadingFonts.value = true;
  try {
    // Check if API is available
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    if ('queryLocalFonts' in window) {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const fonts = await (window as any).queryLocalFonts();
      
      const uniqueFamilies = new Set<string>();
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      fonts.forEach((font: any) => uniqueFamilies.add(font.family));
      
      const sortedFonts = Array.from(uniqueFamilies).sort().map(family => ({
        label: family,
        value: family // Use font name directly, CSS handles quotes handling usually, but we might want to ensure quotes if spaces
      }));
      
      if (sortedFonts.length > 0) {
        fontOptions.value = [
          { label: 'System UI (Default)', value: 'system-ui' },
          ...sortedFonts
        ];
        uiStore.showToast(t('settings.toasts.fonts_loaded'), 'success');
      }
    } else {
        console.warn('Local Font Access API not supported, falling back to basic list');
        uiStore.showToast('Local Font API not supported', 'warn');
    }
  } catch (err) {
    console.error('Failed to load system fonts:', err);
    uiStore.showToast('Failed to load fonts', 'error');
  } finally {
    isLoadingFonts.value = false;
  }
};

onMounted(async () => {
  try {
    versions.value = await HardwareService.getVersions();
  } catch (e) {
    console.error('Failed to get versions:', e);
    versions.value = { app: 'Unknown', controller: 'Unknown' };
  }
});

const langOptions = [
  { label: 'English', value: 'en' },
  { label: 'Español', value: 'es' },
  { label: 'Português (Brasil)', value: 'pt-BR' },
  { label: 'Русский', value: 'ru' },
];



const handleCustomColor = (color: string) => {
  uiStore.setAccentColor(color);
  localStorage.setItem('ffbeast_custom_accent', color);
};

const toastPositionOptions = computed(() => [
  { label: t('toasts.positions.top_right'), value: 'top-right' },
  { label: t('toasts.positions.bottom_right'), value: 'bottom-right' },
  { label: t('toasts.positions.top_left'), value: 'top-left' },
  { label: t('toasts.positions.bottom_left'), value: 'bottom-left' },
]);

// Log Levels
const logLevelOptions = [
  { label: 'Error', value: 1 },
  { label: 'Warn', value: 2 },
  { label: 'Info', value: 3 },
  { label: 'Debug', value: 4 },
  { label: 'Trace', value: 5 },
];

const accentColors = [
  '#00d4ff', // Cyan
  '#e056fd', // Purple
  '#0088ff', // Blue
  '#00ccaa', // Teal
  '#ff6b9d', // Pink
];

const changeLang = (val: string | number) => {
  const lang = String(val);
  locale.value = lang;
  uiStore.setLanguage(lang);
};

const testToast = () => {
  uiStore.showToast(t('toasts.test_notification'), 'success');
};
</script>

<style scoped>
.settings-tab {
  padding: var(--content-padding);
}

.grid-layout {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 24px;
}

.control-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.control-row:last-child {
  border-bottom: none;
}

.control-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.control-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-main);
}

.control-desc {
  font-size: 12px;
  color: var(--text-secondary);
}

.font-controls {
  display: flex;
  gap: 12px;
  align-items: flex-end;
}

.font-select {
  flex: 1;
}

.btn-small {
  padding: 8px 12px;
  height: 42px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.2rem;
  margin-top: 1px;
}

.spinner {
  width: 16px;
  height: 16px;
  border: 2px solid var(--text-dim);
  border-top-color: var(--accent);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.compact-select {
  width: 180px;
}

.theme-selector {
  margin-top: 24px;
}

.theme-selector__label {
  font-size: 12px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  color: var(--text-secondary);
  display: block;
  margin-bottom: 12px;
}

.color-grid {
  display: flex;
  gap: 12px;
}

.color-dot {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  cursor: pointer;
  border: 2px solid transparent;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

.color-dot:hover {
  transform: translateY(-2px);
  scale: 1.1;
}

.custom-color {
  position: relative;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
}

.color-input {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 200%;
  height: 200%;
  padding: 0;
  border: none;
  opacity: 0;
  cursor: pointer;
}

.info-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.info-item {
  display: flex;
  justify-content: space-between;
  font-size: 14px;
}

.info-item__label {
  color: var(--text-secondary);
}

.info-item__value {
  font-family: var(--font-mono);
  color: var(--text-main);
  font-weight: 500;
}

.btn-test {
  margin-top: 16px;
  width: 100%;
  padding: 10px;
  background: var(--accent-primary);
  color: var(--text-on-accent, #000);
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-test:hover {
  filter: brightness(1.1);
  box-shadow: 0 0 15px var(--accent-glow);
}
</style>
