<template>
  <div class="wheel-card">
    <div class="wheel-container">
      <div class="wheel-svg" :style="{ transform: `rotate(${rotationDegrees}deg)` }">
        <svg viewBox="0 0 100 100" class="steering-wheel">
          <circle cx="50" cy="50" r="45" class="wheel-rim" />
          <rect x="48" y="10" width="4" height="15" class="wheel-marker" />
          <path d="M 20 50 L 80 50 M 50 50 L 50 85" class="wheel-spokes" />
        </svg>
      </div>
      <div class="position-value">{{ currentPosition }}°</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  position: number;
}>();

const currentPosition = computed(() => props.position);
const rotationDegrees = computed(() => currentPosition.value); // Map 1:1 for now
</script>

<style scoped>
.wheel-card {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 250px;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 1.5rem;
  backdrop-filter: blur(10px);
}

.wheel-container {
  position: relative;
  width: 200px;
  height: 200px;
}

.wheel-svg {
  width: 100%;
  height: 100%;
  transition: transform 0.05s linear;
}

.steering-wheel {
  width: 100%;
  height: 100%;
}

.wheel-rim {
  fill: none;
  stroke: #222;
  stroke-width: 8;
}

.wheel-marker {
  fill: var(--accent);
}

.wheel-spokes {
  stroke: #333;
  stroke-width: 4;
  fill: none;
}

.position-value {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-family: var(--font-mono);
  font-size: 1.2rem;
  font-weight: 800;
  color: var(--text-main);
  text-shadow: 0 0 10px rgba(0,0,0,0.5);
}
</style>
