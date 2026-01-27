<template>
  <div class="torque-chart-container">
    <div class="chart-header" v-if="label">
      <span class="chart-label">{{ label }}</span>
      <span class="chart-value">{{ modelValue.toFixed(0) }}</span>
    </div>
    <div class="canvas-wrapper" ref="wrapper">
      <canvas ref="canvas"></canvas>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';

const props = defineProps({
  modelValue: { type: Number, required: true },
  max: { type: Number, default: 10000 },
  min: { type: Number, default: -10000 },
  height: { type: [Number, String], default: 100 },
  color: { type: String, default: '#4CAF50' }, // Default Green
  label: { type: String, default: 'Torque' },
  sampleCount: { type: Number, default: 300 },
  orientation: { type: String, default: 'horizontal', validator: (v: string) => ['horizontal', 'vertical'].includes(v) }
});

const canvas = ref<HTMLCanvasElement | null>(null);
const wrapper = ref<HTMLDivElement | null>(null);
const ctx = ref<CanvasRenderingContext2D | null>(null);
// Initialize history with 0s
const history = ref<number[]>([]); 
let animationFrameId: number;
let updateInterval: ReturnType<typeof setInterval>;

const updateHistory = () => {
    history.value.push(props.modelValue);
    if (history.value.length > props.sampleCount) {
        history.value.shift();
    }
};

const resizeCanvas = () => {
    if (!canvas.value || !wrapper.value) return;
    const dpr = window.devicePixelRatio || 1;
    const rect = wrapper.value.getBoundingClientRect();
    
    let targetHeight = rect.height;
    if (typeof props.height === 'number') {
        targetHeight = props.height;
    }
    
    // Ensure we have a valid height
    if (targetHeight === 0) targetHeight = 100;
    
    canvas.value.width = rect.width * dpr;
    canvas.value.height = targetHeight * dpr;
    
    canvas.value.style.width = `${rect.width}px`;
    canvas.value.style.height = `${targetHeight}px`;
    
    if (ctx.value) {
        ctx.value.scale(dpr, dpr);
    }
}

const draw = () => {
    if (!canvas.value || !ctx.value || !wrapper.value) return;
    
    // We rely on canvas.width/height being set by resizeCanvas
    const width = canvas.value.width / (window.devicePixelRatio || 1);
    const height = canvas.value.height / (window.devicePixelRatio || 1);
    
    ctx.value.clearRect(0, 0, width, height);
    
    const range = props.max - props.min;
    
    // Draw center line explanation:
    // For Horizontal: Time is X, Value is Y. Center line is at Y corresponding to Val=0.
    // For Vertical: Time is Y, Value is X. Center line is at X corresponding to Val=0.
    
    ctx.value.beginPath();
    ctx.value.strokeStyle = 'rgba(255, 255, 255, 0.1)';
    ctx.value.lineWidth = 1;

    if (props.orientation === 'horizontal') {
        const zeroY = height - ((0 - props.min) / range) * height;
        ctx.value.moveTo(0, zeroY);
        ctx.value.lineTo(width, zeroY);
    } else {
        const zeroX = ((0 - props.min) / range) * width;
        ctx.value.moveTo(zeroX, 0);
        ctx.value.lineTo(zeroX, height);
    }
    ctx.value.stroke();
    
    ctx.value.beginPath();
    ctx.value.strokeStyle = props.color;
    ctx.value.lineWidth = 2;
    // Glow effect
    ctx.value.shadowBlur = 4;
    ctx.value.shadowColor = props.color;
    
    // Draw line
    if (history.value.length > 0) {
       for (let i = 0; i < history.value.length; i++) {
        const val = history.value[i];
        // Clamp value visually
        const clampedVal = Math.max(props.min, Math.min(props.max, val));
        const normalized = (clampedVal - props.min) / range;
        
        let x, y;

        if (props.orientation === 'horizontal') {
             // Y goes down in canvas (0 at top, height at bottom)
             // We want min val at bottom (height), max val at top (0)
             y = height - (normalized * height);
             x = (i / (props.sampleCount - 1)) * width;
        } else {
             // Vertical Mode:
             // Value on X axis. Min at left (0), Max at right (width).
             x = normalized * width;
             // Time on Y axis. Oldest (0) at Top? Or Bottom?
             // Standard scrolling charts usually flow New -> Old or Old -> New.
             // Let's do Oldest at Top (0) to Newest at Bottom (height).
             y = (i / (props.sampleCount - 1)) * height;
        }
        
        if (i === 0) {
            ctx.value.moveTo(x, y);
        } else {
            ctx.value.lineTo(x, y);
        }
      }
      ctx.value.stroke();
    }
    
    ctx.value.shadowBlur = 0; // Reset
    
    animationFrameId = requestAnimationFrame(draw);
};

// Watch triggers
watch(() => props.height, () => {
    resizeCanvas();
});

watch(() => props.orientation, () => {
   resizeCanvas();
});


onMounted(() => {
    // Fill history initially
    history.value = new Array(props.sampleCount).fill(0);

    if (canvas.value) {
        ctx.value = canvas.value.getContext('2d');
        resizeCanvas();
        window.addEventListener('resize', resizeCanvas);
        
        draw();
        
        // 60Hz update rate
        updateInterval = setInterval(updateHistory, 1000 / 60);
    }
});

onUnmounted(() => {
    window.removeEventListener('resize', resizeCanvas);
    cancelAnimationFrame(animationFrameId);
    clearInterval(updateInterval);
});
</script>

<style scoped>
.torque-chart-container {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%; /* Ensure it fills container */
    height: 100%; /* Default to fill */
    min-height: 0; /* Flexbox fix */
}

.chart-header {
    display: flex;
    justify-content: space-between;
    font-size: 0.85rem;
    color: var(--text-dim);
    font-weight: 600;
    flex-shrink: 0;
}

.chart-value {
    color: var(--text-main);
    font-family: var(--font-mono);
}

.canvas-wrapper {
  width: 100%;
  flex: 1;
  display: block;
  position: relative;
  overflow: hidden;
  min-height: 0;
}

canvas {
    display: block;
}
</style>
