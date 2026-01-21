import { ref } from 'vue';
import type { AxisMapping } from '../models/AxisMapping';

const NAMES_KEY = 'ffbeast_axis.names';
const MAPPINGS_KEY = 'ffbeast_axis.mappings';

export const DEFAULT_NAMES = ["Throttle", "Brake", "Clutch", "Aux 4", "Aux 5", "Aux 6", "Aux 7", "Aux 8"];

export function useMappingPersistence() {
    const axisNames = ref<string[]>([]);
    const mappings = ref<AxisMapping[]>([]);

    function load() {
        const savedNames = localStorage.getItem(NAMES_KEY);
        axisNames.value = savedNames ? JSON.parse(savedNames) : [...DEFAULT_NAMES];

        const savedMappings = localStorage.getItem(MAPPINGS_KEY);
        if (savedMappings) {
            mappings.value = JSON.parse(savedMappings).map((m: AxisMapping) => ({
                ...m,
                thresholdLow: m.thresholdLow ?? 2000,
                thresholdHigh: m.thresholdHigh ?? 30000
            }));
        } else {
            mappings.value = Array(8).fill(0).map(() => ({
                name: '',
                min: 0,
                max: 32767,
                invert: false,
                keyLow: '',
                keyHigh: '',
                btnLow: null,
                btnHigh: null,
                thresholdLow: 2000,
                thresholdHigh: 30000
            }));
        }
    }

    function save(newNames?: string[], newMappings?: AxisMapping[]) {
        if (newNames) {
            axisNames.value = newNames;
            localStorage.setItem(NAMES_KEY, JSON.stringify(newNames));
        }
        if (newMappings) {
            mappings.value = newMappings;
            localStorage.setItem(MAPPINGS_KEY, JSON.stringify(newMappings));
        }
    }

    return {
        axisNames,
        mappings,
        load,
        save
    };
}
