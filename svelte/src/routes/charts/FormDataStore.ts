import { writable, type Writable } from 'svelte/store';

export const START_TIMESTAMP = 1124414573;
export interface ChartFormData {
	chartType: string;
	offset?: number;
	limit?: number;
	startTimestamp: number;
	endTimestamp: number;
	chartScale: 'linear' | 'logarithmic';
	dateRange: 'Week' | 'Month' | 'Quarter' | 'Year' | 'Custom';
}

const oneMonthAgo = new Date();
oneMonthAgo.setMonth(oneMonthAgo.getMonth() - 1);
const startTimestamp = oneMonthAgo.getTime() / 1000;

export const formDataStore: Writable<ChartFormData> = writable({
	chartType: 'Artist',
	offset: 0,
	limit: 10,
	startTimestamp,
	endTimestamp: Date.now() / 1000,
	chartScale: 'linear',
	dateRange: 'Month'
});
