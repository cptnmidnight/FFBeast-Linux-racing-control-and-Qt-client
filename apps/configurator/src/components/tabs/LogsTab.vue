<template>
  <div class="logs-tab">
    <BaseCard :title="$t('tab_logs')" class="full-height">
      <template #header>
        <div class="header-content">
          <h3>{{ $t('tab_logs') }}</h3>
          <div class="actions">
            <button class="btn-small" @click="clearLogs">Clear</button>
            <button class="btn-small" @click="exportLogs">Export</button>
          </div>
        </div>
      </template>
      <div class="logs-wrapper" ref="scrollContainer">
        <div v-for="(log, index) in logs" :key="index" :class="['log-line', log.level]">
          <span class="time">[{{ log.time }}]</span>
          <span class="level">[{{ log.level.toUpperCase() }}]</span>
          <span class="message">{{ log.message }}</span>
        </div>
      </div>
    </BaseCard>
  </div>
</template>

<script setup lang="ts">
import { ref, onUpdated } from 'vue';
import BaseCard from '../common/BaseCard.vue';

interface LogEntry {
  time: string;
  level: 'info' | 'warn' | 'error' | 'debug';
  message: string;
}

const logs = ref<LogEntry[]>([
  { time: '21:55:01', level: 'info', message: 'Application started' },
  { time: '21:55:02', level: 'debug', message: 'HID Device discovery started' },
  { time: '21:55:05', level: 'info', message: 'Found FFBeast device on COM3' },
  { time: '21:55:10', level: 'warn', message: 'HANDSHAKE: Retrying (attempt 1)' },
]);

const scrollContainer = ref<HTMLElement | null>(null);

const clearLogs = () => {
  logs.value = [];
};

const exportLogs = () => {
  const content = logs.value.map(l => `[${l.time}] [${l.level.toUpperCase()}] ${l.message}`).join('\n');
  console.log('Exporting logs:', content);
};

onUpdated(() => {
  if (scrollContainer.value) {
    scrollContainer.value.scrollTop = scrollContainer.value.scrollHeight;
  }
});
</script>

<style scoped>
.logs-tab {
  padding: var(--content-padding);
  height: 100%;
}

.full-height {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.actions {
  display: flex;
  gap: 8px;
}

.btn-small {
  padding: 4px 10px;
  font-size: 0.75rem;
  border: 1px solid var(--border-bright);
  border-radius: var(--radius-sm);
  color: var(--text-dim);
}

.btn-small:hover {
  background: rgba(255,255,255,0.05);
  color: var(--text-main);
}

.logs-wrapper {
  flex: 1;
  background: rgba(0, 0, 0, 0.4);
  border-radius: var(--radius-sm);
  padding: 1rem;
  font-family: var(--font-mono);
  font-size: 0.85rem;
  overflow-y: auto;
}

.log-line {
  margin-bottom: 4px;
  white-space: pre-wrap;
  line-height: 1.4;
}

.time { color: var(--text-dim); margin-right: 8px; }
.level { font-weight: bold; margin-right: 8px; width: 60px; display: inline-block; }
.info .level { color: var(--info); }
.warn .level { color: var(--warning); }
.error .level { color: var(--danger); }
.debug .level { color: #888; }
.message { color: var(--text-main); }
</style>
