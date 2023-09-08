<script lang="ts">
	import * as Popover from '$lib/components/ui/popover';
	import * as Select from '$lib/components/ui/select';
	import { Slider } from '$lib/components/ui/slider';
	import { Input } from '$lib/components/ui/input';
	import { Settings } from 'lucide-svelte';
	import { Label } from '$lib/components/ui/label';

	export let chartType: string;
	export let chartScale: 'linear' | 'logarithmic';
	export let limit: number[];
	export let offset: number[];
	export let dateRange: 'Week' | 'Month' | 'Quarter' | 'Year' | 'Custom';
	export let startTimestamp: number;
	export let endTimestamp: number;
	export let invalidDateAlert: boolean;

	let displayChartScale: string = 'Linear';
	let startYear = 2002;
	let startMonth = 1;
	let endYear = new Date().getFullYear();
	let endMonth = new Date().getMonth();

	const dateRangeMap = {
		Week: 'Weekly',
		Month: 'Monthly',
		Quarter: 'Quarterly',
		Year: 'Annually',
		Custom: 'Custom'
	};

	$: {
		invalidDateAlert = false;
		endTimestamp = Date.now() / 1000;
		if (dateRange === 'Week') {
			let dateNow = new Date();

			dateNow.setDate(dateNow.getDate() - 7);
			startTimestamp = dateNow.getTime() / 1000;
		} else if (dateRange === 'Month') {
			let dateNow = new Date();

			dateNow.setMonth(dateNow.getMonth() - 1);
			startTimestamp = dateNow.getTime() / 1000;
		} else if (dateRange === 'Quarter') {
			let dateNow = new Date();
			const currMonth = dateNow.getMonth();

			if (currMonth < 4) {
				dateNow.setFullYear(dateNow.getFullYear() - 1);
				dateNow.setMonth(dateNow.getMonth() + 9);
			} else {
				dateNow.setMonth(dateNow.getMonth() - 3);
			}

			startTimestamp = dateNow.getTime() / 1000;
		} else if (dateRange === 'Year') {
			let dateNow = new Date();

			dateNow.setFullYear(dateNow.getFullYear() - 1);
			startTimestamp = dateNow.getTime() / 1000;
		} else if (dateRange === 'Custom') {
			const customStart = new Date(`${startYear}-${startMonth}-01`);
			const customEnd = new Date(`${endYear}-${endMonth}-01`);

			if (!isNaN(customStart.getTime()) && !isNaN(customEnd.getTime())) {
				startTimestamp = customStart.getTime() / 1000;
				endTimestamp = customEnd.getTime() / 1000;
			} else if (!startYear || !startMonth || !endYear || !endMonth) {
				const fixedStart = new Date(
					`${startYear || 2002}-${startMonth || 1}-01`
				);
				const fixedEnd = new Date(
					`${endYear || new Date().getFullYear()}-${
						endMonth || new Date().getMonth()
					}-01`
				);

				startTimestamp = fixedStart.getTime() / 1000;
				endTimestamp = fixedEnd.getTime() / 1000;
			} else {
				invalidDateAlert = true;
			}
		}
	}
</script>

<Popover.Root>
	<Popover.Trigger
		type="button"
		class="cursor-pointer items-center justify-center rounded-r-lg border-y border-r border-rose-700 bg-rose-400 px-5 py-2 text-center text-xl font-semibold text-zinc-100 hover:bg-rose-500 focus:outline-none md:py-3"
	>
		<Settings />
	</Popover.Trigger>
	<Popover.Content class="space-y-5 border-0 bg-zinc-900">
		<div class="flex flex-row justify-between">
			<Select.Root
				onSelectedChange={(e) => {
					const val = e?.value;
					if (typeof val == 'string') {
						chartType = val;
					}
				}}
			>
				<Select.Trigger class="w-50 min-w-[7.5rem] text-zinc-200">
					<Select.Value bind:placeholder={chartType} />
				</Select.Trigger>
				<Select.Content class="border-0 bg-zinc-700 text-zinc-200">
					<Select.Item value="Artist">Artist</Select.Item>
					<Select.Item value="Album">Album</Select.Item>
					<Select.Item value="Track">Track</Select.Item>
				</Select.Content>
			</Select.Root>
			<Select.Root
				onSelectedChange={(e) => {
					const val = e?.value;
					if (
						typeof val == 'string' &&
						(val == 'linear' || val == 'logarithmic')
					) {
						chartScale = val;
						displayChartScale = val == 'linear' ? 'Linear' : 'Log';
					}
				}}
			>
				<Select.Trigger class="w-50 min-w-[7.5rem] text-zinc-200">
					<Select.Value bind:placeholder={displayChartScale} />
				</Select.Trigger>
				<Select.Content class="w-50 border-0 bg-zinc-700 text-zinc-200">
					<Select.Item value="linear">Linear</Select.Item>
					<Select.Item value="logarithmic">Log</Select.Item>
				</Select.Content>
			</Select.Root>
		</div>
		<div class="space-y-2">
			<Label for="limitSlider" class="text-zinc-200"
				>Number of Entries: {limit}</Label
			>
			<Slider class="mx-auto w-60" bind:value={limit} max={100} step={1} />
		</div>
		<div class="space-y-2">
			<Label for="limitSlider" class="text-zinc-200">Offset by: {offset}</Label>
			<Slider class="mx-auto w-60" bind:value={offset} max={100} step={1} />
		</div>
		<div class="space-y-2">
			<Label for="dateRangeSelect" class="text-zinc-200">Date Range</Label>
			<Select.Root
				onSelectedChange={(e) => {
					const val = e?.value;
					if (
						typeof val == 'string' &&
						(val == 'Week' ||
							val == 'Month' ||
							val == 'Quarter' ||
							val == 'Year' ||
							val == 'Custom')
					) {
						dateRange = val;
					}
				}}
			>
				<Select.Trigger class="w-50 min-w-[9rem] text-zinc-200">
					<Select.Value bind:placeholder={dateRangeMap[dateRange]} />
				</Select.Trigger>
				<Select.Content class="border-0 bg-zinc-700 text-zinc-200">
					<Select.Item value="Week">Weekly</Select.Item>
					<Select.Item value="Month">Monthly</Select.Item>
					<Select.Item value="Quarter">Quarterly</Select.Item>
					<Select.Item value="Year">Annually</Select.Item>
					<Select.Item value="Custom">Custom</Select.Item>
				</Select.Content>
			</Select.Root>
		</div>
		<div
			class="{dateRange == 'Custom'
				? 'flex'
				: 'hidden'} flex-row justify-between"
		>
			<div class="grid w-full max-w-sm items-center gap-1.5">
				<Label for="year" class="text-zinc-200">Start Year</Label>
				<Input
					class="remove-arrow w-28 text-zinc-200"
					placeholder="Year"
					type="number"
					bind:value={startYear}
				/>
			</div>
			<div class="grid w-full max-w-sm gap-1.5">
				<Label for="month" class="text-zinc-200">Start Month</Label>
				<Input
					class="remove-arrow w-28 text-zinc-200"
					placeholder="Month"
					type="number"
					bind:value={startMonth}
				/>
			</div>
		</div>
		<div
			class="{dateRange == 'Custom'
				? 'flex'
				: 'hidden'} flex-row justify-between"
		>
			<div class="grid w-full max-w-sm items-center gap-1.5">
				<Label for="year" class="text-zinc-200">End Year</Label>
				<Input
					class="remove-arrow w-28 text-zinc-200"
					placeholder="Year"
					type="number"
					bind:value={endYear}
				/>
			</div>
			<div class="grid w-full max-w-sm gap-1.5">
				<Label for="month" class="text-zinc-200">End Month</Label>
				<Input
					class="remove-arrow w-28 text-zinc-200"
					placeholder="Month"
					type="number"
					bind:value={endMonth}
				/>
			</div>
		</div>
	</Popover.Content>
</Popover.Root>
