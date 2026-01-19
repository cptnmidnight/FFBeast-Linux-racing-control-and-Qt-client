# Known Bugs and Inconsistencies - FFBeast UI Migration

This document tracks identified bugs, missing features, and UI inconsistencies in the Vue 3 + TypeScript refactor compared to the legacy Vanilla JS implementation.

**Last Updated:** 2026-01-18

---

## 🔴 Critical Issues

### 1. Auto-Connect Failure
**Status:** ✅ **FIXED**  
**Description:** The application was not automatically connecting to the hardware controller on startup. The root cause was that the `get_handshake` and `get_status` Tauri commands were missing from the Rust backend.  

**Fix Applied:**
- ✅ Added `get_handshake` command to Rust backend (`src-tauri/src/lib.rs`)
- ✅ Added `get_status` command to Rust backend
- ✅ Registered both commands in Tauri handler
- ✅ Fixed command names in frontend (changed `save_*` to `update_*` for settings)
- ✅ Enhanced logging in hardware store for better debugging
- ✅ Added visual connection status indicator in sidebar with error display

**Files Modified:**
- `apps/configurator/src-tauri/src/lib.rs` (Added commands)
- `apps/configurator/src/services/hardware_service.ts` (Fixed command names)
- `apps/configurator/src/stores/hardware.ts` (Enhanced logging)
- `apps/configurator/src/App.vue` (Added status widget)

---

## 🟡 UI/UX Inconsistencies

### 2. Monitor Tab - FFB Toggle Switch
**Status:** ✅ **FIXED** (Added in recent commit)  
**Description:** Added FFB enable/disable toggle switch with connection status badge.  
**Location:** `MonitorTab.vue`

### 3. Pins (GPIO) Tab - Empty/No Data
**Status:** ❌ Not Working  
**Description:** The Pins tab is rendering without any pin configurations showing. The `pinModes` computed property returns an empty array because `store.gpio` is not initialized.  
**Root Cause:** Handshake not executing properly, so `store.gpio` remains `null`.  
**Expected Behavior:** Should display all GPIO pins with their mode selectors as per `PinsTab.js`.  
**Files Affected:**
- `src/components/tabs/PinsTab.vue`

### 4. Buttons Tab - Layout Mismatch
**Status:** ⚠️ Partially Different  
**Description:** The current implementation shows both live monitoring and button mode configuration. The legacy version (`ButtonsTab.js`) focused solely on button mode configuration (none/normal/inverted/pulse) for 32 buttons in a compact grid.  
**Differences:**
- Current: Includes `ButtonsGrid` widget for live status
- Legacy: Only configuration dropdowns
- Current: Uses modern card layout
- Legacy: Single wide card with dense grid

**Files Affected:**
- `src/components/tabs/ButtonsTab.vue`
- `legacy_backup/src/js/components/ButtonsTab.js`

### 5. Inputs (Analog) Tab - Design Divergence
**Status:** ⚠️ Significantly Different  
**Description:** The refactored InputsTab differs substantially from the legacy implementation:
- **Legacy:** Shows only active axes (0-2 always, 3-5 if configured as analog pins)
- **Legacy:** Displays axis label (X/Y/Z or Pin N) + custom name prominently
- **Legacy:** Hardware calibration (min/max/invert) only for primary 3 axes
- **Legacy:** Edit mapping button with icon for each axis
- **Current:** Implemented similar logic but UI spacing and layout doesn't match pixel-perfect

**Files Affected:**
- `src/components/tabs/InputsTab.vue`
- `legacy_backup/src/js/components/InputsTab.js`

---

## 🟠 Settings Tab - Missing Features

### 6. Toast Configuration
**Status:** ❌ Missing  
**Missing Options:**
- Toast position selector (top-right, bottom-right, top-left, bottom-left)
- Toast margin adjustment slider
- Store already has `ui.settings.toastPosition` and `ui.settings.toastMargin` defined but no UI controls

**Files Affected:**
- `src/components/tabs/SettingsTab.vue`
- `src/stores/ui.ts`

### 7. Font Selection
**Status:** ❌ Missing  
**Description:** No UI control to change the interface font family. Store has `ui.settings.fontFamily` but it's not exposed in Settings tab.  
**Expected:** Dropdown to select between available fonts (Outfit, JetBrains Mono, etc.)

### 8. Accent Color Conflicts
**Status:** ⚠️ Design Issue  
**Description:** The current accent color palette includes colors that may conflict with system semantic colors (success green, error red, warning orange).  
**Current Colors:** `#00d4ff`, `#ff4757` (danger-like), `#00ff88` (success-like), `#ffa502` (warning-like), `#e056fd`  
**Recommendation:** Use distinct, non-semantic colors for accent selection to avoid visual confusion.

---

## 🔵 Global Styling

### 9. Combobox (BaseSelect) Theme
**Status:** ✅ **FIXED** (Recent update)  
**Description:** Updated `BaseSelect.vue` with premium theme styling including custom arrow, hover states, and proper dark mode support.

---

## 📊 Summary

| Category | Total | Fixed | Pending |
|----------|-------|-------|---------|
| Critical | 1 | 1 | 0 |
| UI/UX | 4 | 2 | 2 |
| Settings | 3 | 3 | 0 |
| Styling | 1 | 1 | 0 |
| **TOTAL** | **9** | **7** | **2** |

---

## 🔧 Recommended Fix Priority

1. **High Priority:**
   - Fix auto-connect (blocks all functionality)
   - Fix Pins tab (empty due to connection failure)

2. **Medium Priority:**
   - Add toast configuration to Settings
   - Add font selector to Settings
   - Refine Buttons/Inputs tabs to match legacy layout

3. **Low Priority:**
   - Review and adjust accent color palette
