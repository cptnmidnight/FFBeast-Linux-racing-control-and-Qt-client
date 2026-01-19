export interface AxisMapping {
    name: string;
    min: number;
    max: number;
    invert: boolean;
    keyLow: string;
    keyHigh: string;
    btnLow: number | null;
    btnHigh: number | null;
}
