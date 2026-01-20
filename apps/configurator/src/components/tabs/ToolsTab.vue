<template>
  <div class="tools-tab">
    <div class="grid-layout">
      <!-- FFB Diagnostics -->
      <BaseCard :title="$t('tool_ffb_diagnostics')">
        <p class="description">{{ $t('tool_ffb_desc') }}</p>
        <div class="test-controls">
          <BaseSlider 
            v-model="testValues.constant" 
            :label="$t('tool_constant_force')" 
            :min="-100" 
            :max="100" 
            suffix="%"
            @update:model-value="(v: number) => updateFFB(1, v)"
            @change="resetFFB(1)"
          />
          <BaseSlider 
            v-model="testValues.sine" 
            :label="$t('tool_sine_wave')" 
            :min="0" 
            :max="100" 
            suffix="%"
            @update:model-value="(v: number) => updateFFB(2, v)"
            @change="resetFFB(2)"
          />
          <BaseSlider 
            v-model="testValues.damper" 
            :label="$t('tool_damping_effect')" 
            :min="0" 
            :max="100" 
            suffix="%"
            @update:model-value="(v: number) => updateFFB(3, v)"
            @change="resetFFB(3)"
          />
        </div>
        <div class="tools-grid">
          <button class="tool-btn stop" @click="stopAll">{{ $t('tool_stop_all') }}</button>
        </div>
      </BaseCard>

      <!-- Maintenance -->
      <BaseCard :title="$t('tool_maintenance')">
        <p class="description">{{ $t('tool_maintenance_desc') }}</p>
        <div class="tools-grid">
          <button class="tool-btn" @click="recalibrateCenter">{{ $t('tool_recalibrate') }}</button>
          <button class="tool-btn warn" @click="enterDfu">{{ $t('tool_enter_dfu') }}</button>
          <button class="tool-btn danger" @click="factoryReset">{{ $t('tool_factory_reset') }}</button>
        </div>
      </BaseCard>

      <!-- Input Mapping Service -->
      <BaseCard :title="$t('tool_mapping_service')">
        <p class="description">{{ $t('tool_mapping_desc') }}</p>
        <div class="service-status-row">
          <span class="status-label">Status:</span>
          <span :class="['status-badge', { active: mappingActive }]">
            {{ mappingActive ? $t('status_service_active') : $t('status_service_inactive') }}
          </span>
        </div>
        <div class="tools-grid">
          <button 
            v-if="!mappingActive" 
            class="tool-btn success" 
            @click="toggleService(true)"
          >
            {{ $t('btn_start_service') }}
          </button>
          <button 
            v-else 
            class="tool-btn stop" 
            @click="toggleService(false)"
          >
            {{ $t('btn_stop_service') }}
          </button>
        </div>
      </BaseCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, onMounted } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import { useUIStore } from '../../stores/ui';
import BaseCard from '../common/BaseCard.vue';
import BaseSlider from '../common/BaseSlider.vue';

const store = useHardwareStore();
const ui = useUIStore();

const testValues = reactive({
  constant: 0,
  sine: 0,
  damper: 0
});

const updateFFB = (type: number, val: number) => {
  // Map % to 0..32767
  const hwVal = (val / 100) * 32767;
  store.sendFFBTest(type, hwVal);
};

const resetFFB = (type: number) => {
  // Zero out values
  if (type === 1) testValues.constant = 0;
  if (type === 2) testValues.sine = 0;
  if (type === 3) testValues.damper = 0;
  store.sendFFBTest(type, 0);
};

const stopAll = () => {
  testValues.constant = 0;
  testValues.sine = 0;
  testValues.damper = 0;
  store.sendFFBTest(1, 0);
  store.sendFFBTest(2, 0);
  store.sendFFBTest(3, 0);
  ui.showToast('All FFB tests stopped', 'info');
};

const recalibrateCenter = async () => {
  await store.resetCenter();
  ui.showToast('Center recalibrated!', 'success');
};

const enterDfu = async () => {
  if (confirm('Enter DFU Mode? Device will disconnect and enter firmware update mode.')) {
    await store.enterDfu();
    ui.showToast('Switching to DFU...', 'warn');
  }
};

const factoryReset = () => {
  if (confirm('Are you sure you want to reset all settings to factory defaults?')) {
    ui.showToast('Factory reset not implemented in mock', 'warn');
  }
};

const mappingActive = ref(false);

const toggleService = async (active: boolean) => {
  try {
    const { HardwareService } = await import('../../services/hardware_service');
    
    if (active) {
      // Load saved axis mappings from localStorage
      const savedMappings = localStorage.getItem('ffbeast_axis_mappings');
      if (savedMappings) {
        try {
          const mappings = JSON.parse(savedMappings);
          const keyMappings: any[] = [];
          
          // Determine which axes are active (same logic as InputsTab)
          const activeAxes = [0, 1, 2, 3, 4, 5].filter(i => {
            if (i < 3) return true;
            return store.gpio?.pin_mode[i] === 2; // Analog mode
          });
          
          mappings.forEach((mapping: any, arrayIdx: number) => {
            // Get the real hardware axis index
            const actualAxisIndex = activeAxes[arrayIdx];
            if (actualAxisIndex === undefined) return;
            
            // High threshold mapping
            if (mapping.keyHigh) {
              keyMappings.push({
                id: `axis_${actualAxisIndex}_high_${mapping.keyHigh}`,
                source_type: 'axis',
                index: actualAxisIndex,  // Use actual hardware index
                trigger: 'high',
                key: mapping.keyHigh,
                threshold: 3500
              });
            }
            
            // Low threshold mapping
            if (mapping.keyLow) {
              keyMappings.push({
                id: `axis_${actualAxisIndex}_low_${mapping.keyLow}`,
                source_type: 'axis',
                index: actualAxisIndex,  // Use actual hardware index
                trigger: 'low',
                key: mapping.keyLow,
                threshold: 500
              });
            }
          });
          
          if (keyMappings.length > 0) {
            await HardwareService.setKeyboardMapping(keyMappings);
            store.log('info', `Loaded ${keyMappings.length} axis keyboard mappings from config`);
          } else {
            ui.showToast('No keyboard mappings configured. Configure in Inputs tab.', 'warn');
          }
        } catch (err) {
          store.log('error', `Failed to parse saved mappings: ${err}`);
        }
      } else {
        ui.showToast('No keyboard mappings configured. Configure in Inputs tab.', 'warn');
      }
    }
    
    await HardwareService.setKeyboardServiceActive(active);
    mappingActive.value = active;
    
    // Using import for log store too if needed, but it's easier to just use the one from hardware store or keep the import and USE IT
    const { useLogStore } = await import('../../stores/logs');
    const logStore = useLogStore();

    logStore.addLog(
      active ? 'info' : 'warn', 
      active ? 'Keyboard mapping service started' : 'Keyboard mapping service stopped', 
      'backend'
    );

    ui.showToast(
      active ? 'Mapping service started' : 'Mapping service stopped',
      active ? 'success' : 'info'
    );
  } catch (err) {
    ui.showToast(`Failed to toggle service: ${err}`, 'error');
    const { useLogStore } = await import('../../stores/logs');
    useLogStore().addLog('error', `Mapping service error: ${err}`, 'backend');
  }
};

// Sync initial state
onMounted(async () => {
    try {
        const { HardwareService } = await import('../../services/hardware_service');
        mappingActive.value = await HardwareService.getKeyboardServiceActive();
    } catch (err) {
        console.error('Failed to get service status:', err);
    }
});
</script>

<style scoped>
.tools-tab {
  padding: var(--content-padding);
}

.grid-layout {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 1.5rem;
}

.description {
  color: var(--text-dim);
  font-size: 0.9rem;
  margin-bottom: 1.5rem;
}

.service-status-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 1.5rem;
  padding: 12px;
  background: rgba(0,0,0,0.2);
  border-radius: var(--radius-sm);
}

.status-label {
  font-size: 0.85rem;
  color: var(--text-dim);
}

.status-badge {
  font-size: 0.75rem;
  font-weight: bold;
  padding: 4px 10px;
  border-radius: 20px;
  background: rgba(255,255,255,0.1);
  color: var(--text-dim);
  text-transform: uppercase;
}

.status-badge.active {
  background: rgba(0, 212, 255, 0.2);
  color: var(--primary);
  box-shadow: 0 0 10px rgba(0, 212, 255, 0.2);
}

.tools-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}

.tool-btn {
  padding: 12px;
  background: rgba(255,255,255,0.05);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  color: var(--text-main);
  font-size: 0.9rem;
  cursor: pointer;
  transition: all 0.2s ease;
}

.tool-btn:hover {
  background: rgba(255,255,255,0.1);
  border-color: var(--text-dim);
}

.tool-btn.success {
  color: var(--primary);
  border-color: rgba(0, 212, 255, 0.3);
}

.tool-btn.success:hover {
  background: rgba(0, 212, 255, 0.1);
}

.tool-btn.stop {
  color: var(--warning);
  border-color: rgba(255, 165, 2, 0.3);
}

.tool-btn.warn {
  color: var(--warning);
}

.tool-btn.danger {
  color: var(--danger);
  grid-column: span 2;
  margin-top: 10px;
}
</style>
