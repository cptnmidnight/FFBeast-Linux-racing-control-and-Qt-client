<template>
  <div class="monitor-tab">
    <div class="top-row">
      <WheelVisual :position="currentPosition" />
      <div class="monitor-controls">
        <TorqueIndicator :torque="currentTorque" />
        <div class="control-card">
          <div class="control-group">
            <span class="label">{{ $t('labels.ffb_active') }}</span>
            <ThemedSwitch 
              v-model="ffbEnabled" 
              help-key="help.force_enabled"
              @change="toggleFFB" 
            />
          </div>
          <StatusBadge 
            :text="store.isConnected ? $t('status.connected') : $t('status.disconnected')"
            :variant="store.isConnected ? 'success' : 'error'"
            :pulsing="!store.isConnected"
          />
        </div>
      </div>
    </div>

    <BaseCard v-if="uiStore.settings.debugMode" :title="$t('labels.raw_data')" class="debug-panel">
      <div class="debug-grid">
        <div class="debug-item">
          <span class="d-label">{{ $t('labels.position') }}:</span>
          <span class="d-value">{{ currentPosition }}</span>
        </div>
        <div class="debug-item">
          <span class="d-label">{{ $t('labels.torque') }}:</span>
          <span class="d-value">{{ currentTorque }}</span>
        </div>
        <div class="debug-item">
          <span class="d-label">{{ $t('labels.digital_buttons') }} (HEX):</span>
          <span class="d-value">0x{{ currentButtons.toString(16).toUpperCase().padStart(8, '0') }}</span>
        </div>
        <div class="debug-item wide">
          <span class="d-label">{{ $t('labels.analog_inputs') }}:</span>
          <span class="d-value font-mono">[{{ analogValues.join(', ') }}]</span>
        </div>
      </div>
    </BaseCard>

    <div class="middle-row">
      <AnalogMonitor :values="analogValues" :pinModes="pinModes" />
      <ButtonsGrid :buttons="currentButtons" />
    </div>

    <div class="charts-section">
      <div class="charts-header">
        <h4 class="section-title">{{ $t('monitor.torque_title') }}</h4>
        <div class="charts-actions">
           <button class="btn-tool" @click="toggleOrientation" :title="chartOrientation === 'horizontal' ? $t('monitor.orientation_vertical') : $t('monitor.orientation_horizontal')">
             {{ chartOrientation === 'horizontal' ? '⬍ ' + $t('monitor.orientation_vertical') : '⬌ ' + $t('monitor.orientation_horizontal') }}
           </button>
           <button class="btn-tool" @click="isChartExpanded = !isChartExpanded" :title="isChartExpanded ? $t('monitor.collapse') : $t('monitor.expand')">
             {{ isChartExpanded ? '⬇ ' + $t('monitor.collapse') : '⬆ ' + $t('monitor.expand') }}
           </button>
           <button class="btn-tool" @click="openPopout" :title="$t('monitor.popout')">
             ↗ {{ $t('monitor.popout') }}
           </button>
        </div>
      </div>
      <div :class="['chart-wrapper', { expanded: isChartExpanded }]">
         <TorqueChart 
            :model-value="currentTorque" 
            :max="10000" 
            :min="-10000" 
            :label="chartOrientation === 'horizontal' ? $t('monitor.torque_horizontal') : $t('monitor.torque_vertical')" 
            color="#4CAF50" 
            :orientation="chartOrientation" 
            :height="isChartExpanded ? 400 : 150"
          />
      </div>
    </div>

    <BaseCard v-if="uiStore.settings.debugMode" :title="$t('labels.backend_logs')" class="bottom-row">
      <LogsWidget :logs="logs" />
    </BaseCard>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { storeToRefs } from 'pinia';
import { useHardwareStore } from '../../stores/hardware';
import { useLogStore } from '../../stores/logs';
import { useUIStore } from '../../stores/ui';
import { useHardwareStream } from '@shared/composables/useHardwareStream';
import WheelVisual from '../monitor/WheelVisual.vue';
import TorqueIndicator from '../monitor/TorqueIndicator.vue';
import TorqueChart from '../monitor/TorqueChart.vue';
import AnalogMonitor from '../monitor/AnalogMonitor.vue';
import ButtonsGrid from '../monitor/ButtonsGrid.vue';
import LogsWidget from '../monitor/LogsWidget.vue';
import ThemedSwitch from '@shared/components/atoms/ThemedSwitch.vue';
import StatusBadge from '@shared/components/atoms/StatusBadge.vue';
import BaseCard from '../common/BaseCard.vue';
import { HardwareSettingId } from '../../models/HardwareSettingId';

const store = useHardwareStore();
const logStore = useLogStore();
const uiStore = useUIStore();
const { t } = useI18n();
const hardwareStream = useHardwareStream();

const { logs } = storeToRefs(logStore);

const currentPosition = computed(() => hardwareStream.status.value?.position ?? 0);
const currentTorque = computed(() => hardwareStream.status.value?.torque ?? 0);
const currentButtons = computed(() => hardwareStream.status.value?.buttons ?? 0);
const analogValues = computed(() => hardwareStream.status.value?.adc ?? [0, 0, 0, 0, 0, 0, 0, 0]);
const pinModes = computed(() => store.gpio?.pin_mode ?? []);

const ffbEnabled = computed({
  get: () => store.hardware?.force_enabled === 1,
  set: (val) => store.updateHW({ force_enabled: val ? 1 : 0 })
});

const toggleFFB = async (val: boolean) => {
  const numVal = val ? 1 : 0;
  await store.updateHWField(HardwareSettingId.ForceEnabled, 0, numVal);
  await store.updateHW({ force_enabled: numVal });
};

// Chart Controls
const chartOrientation = ref<'horizontal' | 'vertical'>('horizontal');
const isChartExpanded = ref(false);

const toggleOrientation = () => {
    chartOrientation.value = chartOrientation.value === 'horizontal' ? 'vertical' : 'horizontal';
};

const openPopout = async () => {
    try {
        // Try Tauri v2 approach
        try {
             const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow');
             const label = 'torque-monitor-popout';
             
             // Check if already exists to bring to focus
             const existingWin = await WebviewWindow.getByLabel(label);
             if (existingWin) {
                 await existingWin.setFocus();
                 return;
             }

             const win = new WebviewWindow(label, {
                url: 'index.html?mode=chart', // Use relative path to index.html with query param
                title: t('monitor.torque_title'),
                width: 800,
                height: 600,
                center: true,
                focus: true
            });
            
            win.once('tauri://error', (e) => {
                 console.error('Tauri window error:', e);
                 // Fallback if Tauri window fails creation asynchronously
                 window.open(window.location.href.split('?')[0] + '?mode=chart', '_blank', 'width=800,height=600');
            });

        } catch (tauriError) {
             console.warn('Tauri API not available or failed, falling back to window.open', tauriError);
             throw tauriError; // trigger outer catch
        }
    } catch (e) {
        // Fallback for browser dev or if Tauri fails
        const url = window.location.href.split('?')[0] + '?mode=chart';
        window.open(url, 'torque_monitor', 'width=800,height=600,resizable=yes,scrollbars=no');
    }
};

onMounted(async () => {
});
</script>

<style scoped>
.monitor-tab {
  padding: var(--content-padding);
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.top-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.monitor-controls {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.control-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 1rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  backdrop-filter: blur(10px);
}

.control-group {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.control-group .label {
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--text-main);
}

.middle-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.charts-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.debug-panel {
  margin-bottom: 0.5rem;
}

.debug-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 1rem;
}


.debug-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.debug-item.wide {
  grid-column: span 2;
}

.d-label {
  font-size: 0.75rem;
  color: var(--text-dim);
}

.d-value {
  font-family: var(--font-mono);
  font-size: 0.9rem;
  color: var(--accent);
}


/* Charts Section */
.charts-section {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 1rem;
  backdrop-filter: blur(10px);
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.charts-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.section-title {
  margin: 0;
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--text-dim);
  text-transform: uppercase;
  letter-spacing: 1px;
}

.charts-actions {
  display: flex;
  gap: 0.5rem;
}

.btn-tool {
  background: transparent;
  border: 1px solid var(--border);
  color: var(--text-dim);
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  font-size: 0.8rem;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: 4px;
}

.btn-tool:hover {
  background: rgba(255, 255, 255, 0.05);
  color: var(--text-main);
  border-color: var(--text-dim);
}

.chart-wrapper {
  transition: height 0.3s ease;
  overflow: hidden;
}

/* Height handled by TorqueChart prop but this helps transitions if we enforced height here */

.font-mono {
  font-family: var(--font-mono);
}
</style>
