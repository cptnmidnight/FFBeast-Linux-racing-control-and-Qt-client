<template>
  <div class="logs-tab">
    <BaseCard :title="$t('tab_logs') + ' (Live)'" class="full-height">
      <template #header>
        <div class="header-content">
          <h3>{{ $t('tab_logs') }} (Live)</h3>
          <div class="actions">
            <button class="btn-small" @click="clearLogs">Clear</button>
            <button class="btn-small" @click="exportLogs">Export</button>
          </div>
        </div>
      </template>
      <div class="logs-wrapper" ref="scrollContainer">
        <div v-for="(log, index) in logs" :key="index" :class="['log-line', log.level]">
          <span class="time">[{{ log.time }}]</span>
          <span class="source" v-if="log.source">[{{ log.source.toUpperCase().slice(0,2) }}]</span>
          <span class="level">[{{ log.level.toUpperCase() }}]</span>
          <span class="message">{{ log.message }}</span>
        </div>
      </div>
    </BaseCard>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick, onMounted } from 'vue';
import BaseCard from '../common/BaseCard.vue';
import { useLogStore } from '../../stores/logs';
import { storeToRefs } from 'pinia';

const logStore = useLogStore();
const { logs } = storeToRefs(logStore);
const scrollContainer = ref<HTMLElement | null>(null);

const scrollToBottom = async () => {
  await nextTick();
  if (scrollContainer.value) {
    scrollContainer.value.scrollTop = scrollContainer.value.scrollHeight;
  }
};

// Watch for new logs to auto-scroll
watch(() => logs.value.length, scrollToBottom);

const clearLogs = () => {
  logStore.clearLogs();
};

const exportLogs = () => {
  const content = logs.value.map(l => `[${l.time}] [${l.level.toUpperCase()}] ${l.message}`).join('\n');
  
  // Create a blob and download it
  const blob = new Blob([content], { type: 'text/plain' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `ffbeast-logs-${new Date().toISOString().slice(0, 19).replace(/:/g, '-')}.txt`;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
};

onMounted(() => {
    scrollToBottom();
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
.source { color: var(--primary); margin-right: 8px; font-weight: bold; }
.level { font-weight: bold; margin-right: 8px; width: 60px; display: inline-block; }
.info .level { color: var(--info); }
.warn .level { color: var(--warning); }
.error .level { color: var(--danger); }
.debug .level { color: #888; }
.message { color: var(--text-main); }
</style>
