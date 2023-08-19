import { writable, type Writable } from 'svelte/store';

export const START_TIMESTAMP = 1124414573;
export interface ChartFormData {
	chart_type: string;
	offset?: number;
	limit?: number;
	start_timestamp: number;
	end_timestamp: number;
	chart_scale: 'linear' | 'logarithmic';
}

export const formDataStore: Writable<ChartFormData> = writable({
	chart_type: 'Artist',
	offset: 0,
	limit: 10,
	start_timestamp: START_TIMESTAMP,
	end_timestamp: Date.now() * 1000,
	chart_scale: 'linear'
});
