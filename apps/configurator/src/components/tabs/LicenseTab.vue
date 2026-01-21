<template>
  <div class="license-tab">
    <BaseCard :title="$t('license.info')">
      <div class="license-status">
        <div class="status-icon" :class="{ active: isLicensed }">
          {{ isLicensed ? '✅' : '❌' }}
        </div>
        <div class="status-details">
          <h4>{{ isLicensed ? $t('status.activated') : $t('status.trial') }}</h4>
          <div class="id-row">
            <span class="label">{{ $t('license.device_id') }}:</span>
            <code>{{ deviceId }}</code>
            <button class="btn-icon" @click="copyId" :title="$t('tooltip_copy_id')">📋</button>
          </div>
          <div v-if="serialKey" class="id-row">
            <span class="label">{{ $t('license.serial_key') }}:</span>
            <code>{{ serialKey }}</code>
            <button class="btn-icon" @click="copySerialKey" :title="$t('tooltip_copy_serial')">📋</button>
          </div>
        </div>
      </div>

      <div class="license-activation">
        <h3>{{ $t('license.activation') }}</h3>
        <p class="desc">{{ $t('license.serial_placeholder') }}</p>
        <div class="input-group">
          <input type="text" :placeholder="$t('license.serial_placeholder')" v-model="licenseKey" />
          <div class="btn-group">
            <button class="btn-primary" @click="activate">{{ $t('buttons.activate') }}</button>
            <button class="btn-outline" @click="importFile">{{ $t('buttons.import') }}</button>
          </div>
        </div>
      </div>
    </BaseCard>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import { useUIStore } from '../../stores/ui';
import { formatHexArray } from '../../utils/format';
import BaseCard from '../common/BaseCard.vue';

const store = useHardwareStore();
const ui = useUIStore();

const isLicensed = computed(() => store.status?.is_registered ?? false);
const licenseKey = ref('');
const deviceId = computed(() => formatHexArray(store.status?.device_id));
const serialKey = computed(() => formatHexArray(store.status?.serial_key));

const copyId = async () => {
  await navigator.clipboard.writeText(deviceId.value);
  ui.showToast('ID copied to clipboard', 'info');
};

const copySerialKey = async () => {
  await navigator.clipboard.writeText(serialKey.value);
  ui.showToast('Serial key copied to clipboard', 'info');
};

const activate = async () => {
  if (!licenseKey.value) {
    ui.showToast('Please enter a serial key', 'error');
    return;
  }
  try {
    await store.activateLicense(licenseKey.value);
    ui.showToast('License activated! Rebooting device...', 'success');
  } catch (err) {
    ui.showToast('Activation failed: ' + err, 'error');
  }
};

const importFile = () => {
  ui.showToast('Import from file not yet implemented', 'info');
};
</script>

<style scoped>
.license-tab {
  padding: var(--content-padding);
}

.license-status {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  margin-bottom: 2.5rem;
  background: rgba(255, 255, 255, 0.03);
  padding: 1.5rem;
  border-radius: var(--radius-md);
  border: 1px solid var(--border);
}

.status-icon {
  font-size: 2.5rem;
}

.status-details h4 {
  margin: 0 0 10px;
  color: var(--text-main);
  font-size: 1.1rem;
}

.id-row {
  display: flex;
  align-items: center;
  gap: 10px;
}

.id-row .label {
  color: var(--text-dim);
  font-size: 0.85rem;
}

code {
  background: rgba(0,0,0,0.3);
  padding: 4px 10px;
  border-radius: 4px;
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: 0.9rem;
  border: 1px solid var(--border-bright);
}

.btn-icon {
  font-size: 1.1rem;
  opacity: 0.6;
}

.btn-icon:hover { opacity: 1; }

.license-activation h3 {
  font-size: 1rem;
  margin-bottom: 10px;
}

.desc {
  color: var(--text-dim);
  font-size: 0.85rem;
  margin-bottom: 1rem;
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

input {
  width: 100%;
  padding: 12px;
  font-family: var(--font-mono);
}

.btn-group {
  display: flex;
  gap: 10px;
}

.btn-primary {
  background: var(--accent);
  color: #000;
  font-weight: bold;
  padding: 10px 24px;
  border-radius: var(--radius-sm);
  flex: 1;
}

.btn-outline {
  border: 1px solid var(--border-bright);
  color: var(--text-main);
  padding: 10px 24px;
  border-radius: var(--radius-sm);
}
</style>
