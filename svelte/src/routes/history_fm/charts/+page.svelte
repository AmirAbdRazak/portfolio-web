<script lang="ts">
	import chart_image from '$lib/assets/chartexample.png';
	import { formDataStore } from './FormDataStore';
	import { goto } from '$app/navigation';

	let username: string;
	let chart_type: string;
	let limit: number = 10;
	let offset: number = 0;
	let chart_scale: 'linear' | 'logarithmic';

	function handleSubmit(event: Event) {
		event.preventDefault();
		formDataStore.set({ chart_type, limit, offset, chart_scale });
		goto(`charts/${username}`, { replaceState: false });
	}
</script>

<section class="bg-slate-800 px-4 py-10 lg:px-10 lg:py-20">
	<div
		class="flex flex-col md:grid max-w-screen-xl px-4 py-8 mx-auto lg:gap-8 xl:gap-0 lg:py-16 lg:grid-cols-12"
	>
		<div class="mr-auto place-self-center lg:col-span-7">
			<h1
				class="max-w-2xl mb-4 text-4xl text-center md:text-left font-extrabold tracking-tight leading-none md:text-5xl xl:text-6xl text-white"
			>
				<!-- svelte-ignore a11y-missing-attribute -->
				Check out your listening trends for <a class="text-rose-400">LastFM</a> users
			</h1>
			<p
				class="max-w-2xl mb-6 text-center md:text-left font-light lg:mb-8 md:text-lg lg:text-xl text-slate-200"
			>
				Compare and observe how your music taste evolved over time, tracks you've been (or had been)
				obsessed with, artists you've been a number one fan of and more!
			</p>
			<div class="flex px-5 pb-10 lg:hidden">
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<!-- svelte-ignore a11y-no-static-element-interactions -->
				<div
					class="lg:hidden transform transition-transform duration-300 ease-in-out hover:scale-110"
					on:click={() => {
						goto('charts/ryzlesalt');
					}}
				>
					<img
						src={chart_image}
						alt="temp chart"
						class="pt-7 pb-5 object-contain drop-shadow-2xl"
					/>
					<p class="text-center text-slate-100 text-sm font-light">
						If you want to try out without a LastFM account, click here!
					</p>
				</div>
			</div>
			<form method="POST" on:submit={handleSubmit}>
				<div class="flex flex-col">
					<div class="flex flex-col sm:flex-row pb-5">
						<input
							class="flex sm:inline-flex items-center justify-center mx-auto py-2 mb-5 sm:mb-0 md:mr-auto md:pr-5 md:py-3 w-80 text-slate-800 font-medium drop-shadow-lg text-center bg-slate-100 border-2 border-slate-700 rounded-lg focus:ring-3 focus:ring-slate-800 focus:outline-none"
							bind:value={username}
							placeholder="Enter your username"
						/>
						<div
							class="group flex sm:inline-flex mx-auto focus-within:ring-4 focus-within:ring-slate-800 rounded-lg sm:ml-auto"
						>
							<button
								class="hidden md:block items-center justify-center px-5 py-2 md:py-3 text-sm md:text-base font-semibold text-center bg-rose-400 border-y border-l rounded-l-lg text-slate-100 border-rose-700 hover:bg-rose-500 focus:outline-none"
								type="submit"
							>
								Generate chart
							</button>
							<button
								class="md:hidden items-center justify-center px-5 py-2 md:py-3 text-base font-semibold text-center bg-rose-400 border-y border-l rounded-l-lg text-slate-100 border-rose-700 hover:bg-rose-500 focus:outline-none"
								type="submit"
							>
								Generate
							</button>
							<select
								id="chart_type"
								class="items-center justify-center px-5 py-2 md:py-3 text-base font-semibold text-center bg-rose-400 border text-slate-100 border-rose-700 hover:bg-rose-500 focus:outline-none cursor-pointer"
								bind:value={chart_scale}
							>
								<option selected value="linear">Linear</option>
								<option value="logarithmic">Log</option>
							</select>
							<select
								id="chart_type"
								class="items-center justify-center px-5 py-2 md:py-3 text-base font-semibold text-center bg-rose-400 border-y border-r rounded-r-lg text-slate-100 border-rose-700 hover:bg-rose-500 focus:outline-none cursor-pointer"
								bind:value={chart_type}
							>
								<option selected value="Artist">Artist</option>
								<option value="Album">Album</option>
								<option value="Track">Track</option>
							</select>
						</div>
					</div>
					<div class="flex flex-col">
						<div class="pb-2">
							<label for="limit-range" class="inline-flex mb-2 text-sm font-medium text-white"
								>Data Limit:
							</label>
							<input
								bind:value={limit}
								class="inline-flex mb-2 text-md border-none bg-transparent text-white focus:outline-none pl-1 pr-5"
							/>
						</div>
						<input
							id="limit-range"
							type="range"
							bind:value={limit}
							class="w-full h-1 mb-6 rounded-lg appearance-none cursor-pointer range-sm bg-slate-700"
						/>
						<div class="pb-2">
							<label for="offset-range" class="inline-flex mb-2 text-sm font-medium text-white"
								>Data Offset:
							</label>
							<input
								bind:value={offset}
								class="inline-flex mb-2 text-md border-none bg-transparent text-white focus:outline-none pl-1 pr-5"
							/>
						</div>
						<input
							id="offset-range"
							type="range"
							bind:value={offset}
							class="w-full h-1 mb-6 rounded-lg appearance-none cursor-pointer range-sm bg-slate-700"
						/>
					</div>
				</div>
			</form>
		</div>
		<div class="hidden lg:flex lg:col-span-5">
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<!-- svelte-ignore a11y-no-static-element-interactions -->
			<div
				class="hidden lg:mt-20 lg:flex lg:flex-col transform transition-transform duration-300 ease-in-out hover:scale-110"
				on:click={() => {
					goto('charts/ryzlesalt');
				}}
			>
				<img src={chart_image} alt="temp chart" class="pt-7 pb-5 object-contain drop-shadow-2xl" />
				<p class="text-center text-slate-100 text-sm font-light">
					If you want to try out without a LastFM account, click here!
				</p>
			</div>
		</div>
	</div>
</section>
