import type { KeyMapping } from './KeyMapping';

export interface AxisMapping {
    name: string;
    key_low: string;
    key_high: string;
    threshold_low: number;
    threshold_high: number;
}

export interface KeyboardProfile {
    id: string;
    name: string;
    key_mappings: KeyMapping[];
    axis_mappings: AxisMapping[];
    axis_names: Record<number, string>;
}
