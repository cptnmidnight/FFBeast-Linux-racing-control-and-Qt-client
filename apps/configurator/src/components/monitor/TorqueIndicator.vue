<template>
  <section class="status-card">
    <div class="card-header">
      <h3>{{ $t('status_card_title') }}</h3>
    </div>
    <div class="metrics-grid">
      <div class="metric">
        <label>{{ $t('label_torque') }}</label>
        <div class="bar-container">
          <div class="bar torque-bar" :style="{ width: Math.abs(torque) + '%' }"></div>
        </div>
        <span class="metric-value">{{ torque }}%</span>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
defineProps<{
  torque: number;
}>();
</script>

<style scoped>
section {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 1.5rem;
  backdrop-filter: blur(10px);
}

.card-header h3 {
  font-size: 0.9rem;
  color: var(--text-dim);
  text-transform: uppercase;
  letter-spacing: 1px;
  margin-bottom: 1.5rem;
}

.bar-container {
  height: 8px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 4px;
  margin: 10px 0;
  overflow: hidden;
}

.bar {
  height: 100%;
  border-radius: 4px;
  transition: width 0.1s ease;
}

.torque-bar {
  background: var(--accent);
  box-shadow: 0 0 10px var(--accent-glow);
}

.metric-value {
  font-family: var(--font-mono);
  font-size: 0.85rem;
  color: var(--text-dim);
}

label {
  font-size: 0.9rem;
  color: var(--text-main);
}
</style>
