<template>
  <section class="inputs-card">
    <div class="card-header">
      <h3>{{ $t('analog_inputs_title') }}</h3>
    </div>
    <div class="analog-grid">
      <div v-for="(val, index) in values" :key="index" class="analog-item">
        <div class="analog-bar-wrapper">
          <div class="analog-bar" :style="{ height: (val / 4095 * 100) + '%' }"></div>
        </div>
        <span class="analog-label">A{{ index }}</span>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
defineProps<{
  values: number[];
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

.analog-grid {
  display: flex;
  justify-content: space-between;
  gap: 10px;
  height: 120px;
}

.analog-item {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.analog-bar-wrapper {
  flex: 1;
  width: 12px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 6px;
  position: relative;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
}

.analog-bar {
  width: 100%;
  background: var(--accent);
  border-radius: 6px;
  transition: height 0.05s linear;
}

.analog-label {
  font-size: 0.7rem;
  color: var(--text-dim);
  font-family: var(--font-mono);
}
</style>
