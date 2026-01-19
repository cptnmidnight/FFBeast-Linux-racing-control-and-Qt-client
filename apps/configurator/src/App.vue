<template>
  <div class="app-container">
    <nav class="sidebar">
      <div class="brand">
        <img src="./assets/logo_app.png" alt="FFBeast Logo" class="brand-logo">
        <span class="brand-name">FFBeast UI</span>
      </div>
      
      <div class="nav-items">
        <div 
          v-for="tab in tabs" 
          :key="tab.id" 
          :class="['nav-item', { active: currentTab === tab.id }, tab.class]" 
          @click="currentTab = tab.id"
        >
          <span class="icon">{{ tab.icon }}</span>
          <span class="label">{{ $t(tab.label) }}</span>
        </div>
      </div>

      <!-- Connection Status in Sidebar -->
      <div class="connection-status" :class="{ connected: store.isConnected, error: store.lastError }">
        <div class="status-indicator"></div>
        <div class="status-text">
          <span v-if="store.isConnecting">Connecting...</span>
          <span v-else-if="store.isConnected">{{ $t('status_connected') }}</span>
          <span v-else>{{ $t('status_disconnected') }}</span>
        </div>
        <div v-if="store.lastError" class="error-text" :title="store.lastError">
          {{ store.lastError.substring(0, 30) }}...
        </div>
      </div>
      <div class="sidebar-footer">
        <div :class="['status-indicator', { connected: store.isConnected, disconnected: !store.isConnected }]">
          <div class="status-dot"></div>
          <span class="status-text">{{ $t(statusTextKey) }}</span>
        </div>
      </div>
    </nav>

    <main class="content">
      <header class="content-header">
        <h2 class="tab-title">{{ $t(currentTabLabel) }}</h2>
        <div class="header-actions">
          <button class="btn-outline reboot" @click="handleReboot">{{ $t('btn_reboot') }}</button>
          <button class="btn-outline" @click="handleResetCenter">{{ $t('btn_reset_center') }}</button>
          <button class="btn-primary" @click="handleSave">{{ $t('btn_save') }}</button>
        </div>
      </header>

      <div class="tab-viewport">
        <Transition name="fade" mode="out-in">
          <MonitorTab v-if="currentTab === 'monitor'" />
          <EffectsTab v-else-if="currentTab === 'effects'" />
          <HardwareTab v-else-if="currentTab === 'hardware'" />
          <ProtocolTab v-else-if="currentTab === 'protocol'" />
          <PinsTab v-else-if="currentTab === 'pins'" />
          <LicenseTab v-else-if="currentTab === 'license'" />
          <ToolsTab v-else-if="currentTab === 'tools'" />
          <ButtonsTab v-else-if="currentTab === 'buttons'" />
          <InputsTab v-else-if="currentTab === 'inputs'" />
          <LogsTab v-else-if="currentTab === 'logs'" />
          <SettingsTab v-else-if="currentTab === 'settings'" />
          <div v-else class="placeholder-content">
            <p>{{ $t('tab_in_progress') }} ({{ currentTab }})</p>
          </div>
        </Transition>
      </div>
    </main>

    <ToastContainer />
    <BaseTooltip />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useHardwareStore } from './stores/hardware';
import MonitorTab from './components/tabs/MonitorTab.vue';
import EffectsTab from './components/tabs/EffectsTab.vue';
import HardwareTab from './components/tabs/HardwareTab.vue';
import ProtocolTab from './components/tabs/ProtocolTab.vue';
import PinsTab from './components/tabs/PinsTab.vue';
import LicenseTab from './components/tabs/LicenseTab.vue';
import ToolsTab from './components/tabs/ToolsTab.vue';
import ButtonsTab from './components/tabs/ButtonsTab.vue';
import InputsTab from './components/tabs/InputsTab.vue';
import LogsTab from './components/tabs/LogsTab.vue';
import SettingsTab from './components/tabs/SettingsTab.vue';
import ToastContainer from './components/common/ToastContainer.vue';
import BaseTooltip from './components/common/BaseTooltip.vue';
import { useUIStore } from './stores/ui';
import { useI18n } from 'vue-i18n';

const store = useHardwareStore();
const ui = useUIStore();
const { t } = useI18n();
const currentTab = ref('monitor');

const tabs = [
  { id: 'monitor', label: 'tab_monitor', icon: '📊' },
  { id: 'effects', label: 'tab_effects', icon: '⚡' },
  { id: 'hardware', label: 'tab_hardware', icon: '⚙️' },
  { id: 'protocol', label: 'tab_protocol', icon: '🔌' },
  { id: 'pins', label: 'tab_pins', icon: '📍' },
  { id: 'buttons', label: 'tab_buttons', icon: '🔘' },
  { id: 'inputs', label: 'tab_inputs', icon: '🎮' },
  { id: 'license', label: 'tab_license', icon: '🔑' },
  { id: 'tools', label: 'tab_tools', icon: '🛠️' },
  { id: 'logs', label: 'tab_logs', icon: '📝' },
  { id: 'settings', label: 'tab_settings', icon: '🛠️', class: 'mt-auto' },
];

const currentTabLabel = computed(() => {
  return tabs.find(t => t.id === currentTab.value)?.label || '';
});

const statusTextKey = computed(() => {
  return store.isConnected ? 'status_connected' : 'status_disconnected';
});

const handleReboot = () => store.reboot();
const handleResetCenter = () => store.resetCenter();
const handleSave = () => store.saveToEeprom();

onMounted(() => {
  store.init();
  setupGlobalTooltips();
});

const setupGlobalTooltips = () => {
  document.addEventListener('mouseover', (e) => {
    const target = (e.target as HTMLElement).closest('[data-help]');
    if (target) {
      const helpKey = target.getAttribute('data-help');
      if (helpKey) {
        ui.showTooltip(t(helpKey), e.clientX, e.clientY);
      }
    }
  });

  document.addEventListener('mousemove', (e) => {
    if (ui.tooltip.show) {
      ui.tooltip.x = e.clientX;
      ui.tooltip.y = e.clientY;
    }
  });

  document.addEventListener('mouseout', (e) => {
    const target = (e.target as HTMLElement).closest('[data-help]');
    if (target) {
      ui.hideTooltip();
    }
  });
};
</script>

<style scoped>
.app-container {
  display: flex;
  width: 100%;
  height: 100%;
}

.sidebar {
  width: 200px;
  background: var(--bg-sidebar);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
}

.brand {
  padding: 1.5rem 1rem;
  border-bottom: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.75rem;
}

.brand-logo {
  width: 48px;
  height: 48px;
  object-fit: contain;
  filter: drop-shadow(0 0 10px var(--accent-glow));
}

.brand-name {
  font-weight: 800;
  font-size: 1.1rem;
  color: var(--accent);
  letter-spacing: 1px;
}

.nav-links {
  list-style: none;
  padding: 0.75rem;
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  border-radius: var(--radius-sm);
  cursor: pointer;
  color: var(--text-dim);
  transition: all 0.2s;
  font-size: 0.9rem;
}

.nav-item:hover {
  background: rgba(255, 255, 255, 0.05);
  color: var(--text-main);
}

.nav-item.active {
  background: var(--accent-muted);
  color: var(--accent);
  border-left: 3px solid var(--accent);
  padding-left: 9px;
}

.nav-icon {
  font-size: 1.1rem;
  width: 20px;
  text-align: center;
}

.mt-auto {
  margin-top: auto;
}

.connection-status {
  padding: 12px;
  margin: 8px;
  background: rgba(255, 0, 0, 0.1);
  border: 1px solid rgba(255, 0, 0, 0.3);
  border-radius: var(--radius-sm);
  font-size: 0.75rem;
}

.connection-status.connected {
  background: rgba(0, 255, 0, 0.1);
  border-color: rgba(0, 255, 0, 0.3);
}

.connection-status .status-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--danger);
  display: inline-block;
  margin-right: 6px;
  animation: pulse 2s infinite;
}

.connection-status.connected .status-indicator {
  background: var(--success);
  animation: none;
}

.connection-status .status-text {
  color: var(--text-main);
  font-weight: 600;
  margin-bottom: 4px;
}

.connection-status .error-text {
  color: var(--danger);
  font-size: 0.65rem;
  margin-top: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.3; }
}

.sidebar-footer {
  padding: 1rem;
  border-top: 1px solid var(--border);
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 0.75rem;
  color: var(--text-dim);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #444;
}

.status-indicator.connected .status-dot {
  background: var(--success);
  box-shadow: 0 0 10px var(--success);
}

.status-indicator.disconnected .status-dot {
  background: var(--danger);
  box-shadow: 0 0 10px var(--danger);
}

.content {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  background: var(--bg-main);
}

.content-header {
  padding: 1rem 1.5rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: var(--bg-card);
  border-bottom: 1px solid var(--border);
  backdrop-filter: blur(10px);
}

.tab-title {
  font-size: 1.2rem;
  font-weight: 600;
}

.header-actions {
  display: flex;
  gap: 10px;
}

.tab-viewport {
  flex: 1;
  overflow-y: auto;
}

/* Button variants - will be moved to common components later */
.btn-primary {
  background: var(--accent);
  color: #000;
  font-weight: bold;
  padding: 8px 16px;
  border-radius: var(--radius-sm);
}

.btn-primary:hover {
  filter: brightness(1.1);
  box-shadow: 0 0 15px var(--accent-glow);
}

.btn-outline {
  border: 1px solid var(--border-bright);
  color: var(--text-main);
  padding: 8px 16px;
  border-radius: var(--radius-sm);
}

.btn-outline:hover {
  background: rgba(255, 255, 255, 0.05);
  border-color: var(--text-dim);
}

.reboot {
  color: var(--warning);
  border-color: rgba(255, 165, 2, 0.3);
}

.reboot:hover {
  border-color: var(--warning);
  background: rgba(255, 165, 2, 0.1);
}

.placeholder-content {
  padding: 3rem;
  color: var(--text-dim);
  text-align: center;
  font-style: italic;
}

/* Transitions */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateY(10px);
}
</style>
