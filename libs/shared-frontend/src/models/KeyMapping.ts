export interface KeyMapping {
    id: string;
    source_type: 'axis' | 'button';
    index: number;
    trigger: 'high' | 'low';
    key: string;
    button?: string;
    threshold?: number;
    threshold_min?: number;
    threshold_max?: number;
}
