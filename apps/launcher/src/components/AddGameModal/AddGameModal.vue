<script lang="ts">
import { defineComponent, ref } from 'vue';
import { Game } from '../../types';
import { GameService } from '../../services/tauri';
// import { open } from '@tauri-apps/plugin-dialog'; // Requires npm install and cargo add


// Note: Using window.__TAURI__.dialog if available would be better for file picking, 
// but currently focusing on basic input if plugin-dialog is not setup. 
// Assuming manual input for now as strictly requested "keep it simple" initially.

export default defineComponent({
  name: 'AddGameModal',
  emits: ['close', 'saved'],
  setup(props, { emit }) {
    const name = ref('');
    const path = ref('');
    const args = ref('');
    const isSteam = ref(false);
    const steamId = ref<number | null>(null);

    // Advanced vars
    const envVars = ref('');
    const dllOverrides = ref('');
    
    // Config vars
    const motionRange = ref(900);
    const totalForce = ref(100);
    const springStrength = ref(100);
    const invertForce = ref(false);

    const closeModal = () => {
        emit('close');
        resetForm();
    };

    const resetForm = () => {
        name.value = '';
        path.value = '';
        args.value = '';
        isSteam.value = false;
        steamId.value = null;
        envVars.value = '';
        dllOverrides.value = '';
        motionRange.value = 900;
        totalForce.value = 100;
        springStrength.value = 100;
        invertForce.value = false;
    };

    const browseFile = async () => {
        // Placeholder for dialog open
        alert("Please manually paste the path for now. File dialog requires 'tauri-plugin-dialog'.");
        /*
        try {
            const selected = await open({
                multiple: false,
                filters: [{ name: 'Executables', extensions: ['exe', 'sh'] }]
            });
            if (selected) {
                path.value = selected as string;
            }
        } catch (e) {
            console.error(e);
        }
        */
    };

    const saveGame = async () => {
        if(!name.value || (!path.value && !isSteam.value)) {
            alert("Name and Path (or Steam ID) are required");
            return;
        }

        const id = name.value.toLowerCase().replace(/\s+/g, '-');
        
        // Parse advanced fields
        const envVarsObj: Record<string, string> = {};
        if (envVars.value) {
            envVars.value.split(';').forEach(pair => {
                const [k, v] = pair.split('=');
                if (k && v) envVarsObj[k.trim()] = v.trim();
            });
        }

        const dllList = dllOverrides.value ? dllOverrides.value.split(',').map(s => s.trim()) : [];
        
        const newGame: Game = {
            id,
            name: name.value,
            path: path.value,
            is_steam: isSteam.value,
            steam_id: steamId.value || undefined,
            arguments: args.value ? args.value.split(' ') : [],
            environment_vars: envVarsObj,
            dll_overrides: dllList,
            use_compat_layer: false,
            wheel_profile: {
                motion_range: Number(motionRange.value),
                total_force: Number(totalForce.value),
                spring_strenth: Number(springStrength.value),
                invert_force: invertForce.value
            }
        };

        try {
            await GameService.saveGame(newGame);
            emit('saved');
            closeModal();
        } catch (e) {
            console.error("Failed to save game", e);
            alert("Failed to save game: " + e);
        }
    };

    return {
        name,
        path,
        args,
        isSteam,
        steamId,
        envVars,
        dllOverrides,
        motionRange,
        totalForce,
        springStrength,
        invertForce,
        closeModal,
        saveGame,
        browseFile
    };
  }
});
</script>

<template>
  <div class="modal-overlay" @click.self="closeModal">
    <div class="modal-content">
      <h2>{{ $t('library.form.title') }}</h2>
      
      <div class="form-row">
          <div class="form-group flex-1">
            <label>{{ $t('library.form.name') }}</label>
            <input type="text" v-model="name" placeholder="Assetto Corsa" />
          </div>
          <div class="form-group checkbox-wrapper">
             <label>
                <input type="checkbox" v-model="isSteam"> Is Steam Game?
             </label>
          </div>
      </div>

      <div class="form-group" v-if="isSteam">
        <label>Steam App ID</label>
        <input type="number" v-model="steamId" placeholder="244210" />
      </div>

      <div class="form-group">
        <label>{{ $t('library.form.path') }}</label>
        <div class="input-row">
            <input type="text" v-model="path" placeholder="/path/to/game.exe" />
            <button class="btn-browse" @click="browseFile">📂</button>
        </div>
      </div>

      <div class="form-group">
        <label>{{ $t('library.form.args') }}</label>
        <input type="text" v-model="args" placeholder="--fullscreen" />
      </div>

      <!-- Advanced Section Toggle -->
      <details class="advanced-details">
          <summary>Advanced Options (Env Vars, DLLs)</summary>
          <div class="form-group">
            <label>Environment Variables (KEY=VALUE;KEY2=VAL2)</label>
            <input type="text" v-model="envVars" placeholder="DXVK_HUD=1" />
          </div>
          <div class="form-group">
            <label>DLL Overrides (d3d11,d3d9)</label>
            <input type="text" v-model="dllOverrides" />
          </div>
      </details>

      <!-- Quick Profile Settings -->
      <div class="profile-section">
        <h3>Default Wheel Profile</h3>
        <div class="row">
             <div class="col">
                <label>{{ $t('config.motion_range') }}</label>
                <input type="number" v-model="motionRange" />
             </div>
             <div class="col">
                <label>{{ $t('config.total_force') }} (%)</label>
                <input type="number" v-model="totalForce" min="0" max="100"/>
             </div>
        </div>
        <div class="row mt-2">
             <div class="col">
                <label>Spring (%)</label>
                <input type="number" v-model="springStrength" min="0" max="100"/>
             </div>
             <div class="col checkbox-col">
                <label>
                    <input type="checkbox" v-model="invertForce"> Invert Force
                </label>
             </div>
        </div>
      </div>

      <div class="actions">
        <button class="btn-secondary" @click="closeModal">{{ $t('library.form.cancel') }}</button>
        <button class="btn-primary" @click="saveGame">{{ $t('library.form.save') }}</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0,0,0,0.7);
    backdrop-filter: blur(5px);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
}

.modal-content {
    background: var(--card-bg);
    border: 1px solid var(--border-color);
    padding: 2rem;
    border-radius: 16px;
    width: 90%;
    max-width: 500px;
    box-shadow: 0 10px 40px rgba(0,0,0,0.3);
}

h2 {
    margin-bottom: 2rem;
}

.form-group {
    margin-bottom: 1.5rem;
}

label {
    display: block;
    font-size: 0.85rem;
    font-weight: 600;
    margin-bottom: 0.5rem;
    opacity: 0.8;
}

input[type="text"], input[type="number"] {
    width: 100%;
    padding: 0.8rem;
    background: var(--bg-color);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    color: var(--text-color);
    font-size: 1rem;
}

input:focus {
    outline: none;
    border-color: var(--primary-color);
}

.form-row {
    display: flex;
    gap: 1rem;
    align-items: flex-start;
}
.flex-1 { flex: 1; }
.checkbox-wrapper {
    margin-top: 1.8rem;
    white-space: nowrap;
}
.checkbox-wrapper input, .checkbox-col input {
    margin-right: 0.5rem;
    transform: scale(1.2);
}
.checkbox-col {
    display: flex;
    align-items: center;
    padding-top: 1.5rem;
}

.input-row {
    display: flex;
    gap: 0.5rem;
}

.btn-browse {
    padding: 0 1rem;
    background: var(--bg-color);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    cursor: pointer;
    font-size: 1.2rem;
}
.btn-browse:hover {
    background: var(--border-color);
}

.advanced-details {
    background: rgba(0,0,0,0.03);
    padding: 0.5rem;
    border-radius: 8px;
    margin-bottom: 1.5rem;
}
.advanced-details summary {
    cursor: pointer;
    font-weight: 600;
    opacity: 0.7;
    margin-bottom: 0.5rem;
}

.mt-2 { margin-top: 1rem; }

.profile-section {
    margin-top: 2rem;
    background: rgba(0,0,0,0.05);
    padding: 1rem;
    border-radius: 8px;
}

.profile-section h3 {
    font-size: 0.9rem;
    margin-bottom: 1rem;
    opacity: 0.7;
}

.row {
    display: flex;
    gap: 1rem;
}

.col {
    flex: 1;
}

.actions {
    margin-top: 2rem;
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
}

.btn-primary {
    background: var(--primary-color);
    color: white;
    border: none;
    padding: 0.8rem 1.5rem;
    border-radius: 8px;
    cursor: pointer;
    font-weight: 600;
}

.btn-secondary {
    background: transparent;
    border: 1px solid var(--border-color);
    color: var(--text-color);
    padding: 0.8rem 1.5rem;
    border-radius: 8px;
    cursor: pointer;
}
</style>
