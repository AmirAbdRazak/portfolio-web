<script lang="ts">
	export let username: string;
	import Device from 'svelte-device-info';
</script>

<div
	class="fixed bottom-0 left-0 right-0 top-0 z-50 flex h-screen w-full flex-col items-center justify-center overflow-hidden bg-slate-800"
>
	<div
		class="loader mb-4 h-12 w-12 rounded-full border-4 border-t-4 border-slate-100 ease-linear"
	/>
	<h2 class="text-center text-xl font-semibold text-white">Loading...</h2>
	<p class="w-1/3 py-2 text-center text-white">
		Preparing your chart {username}
	</p>
	{#if Device.isPhone || Device.isTablet}
		<p class="w-1/2 py-2 text-center text-sm text-slate-200">
			Tip: You can pinch the screen to zoom on the chart :D
		</p>
	{:else}
		<p class="w-1/2 py-2 text-center text-sm text-slate-200">
			Tip: Hold down your <b class="text-rose-400">CTRL</b> key and
			<b class="text-rose-400">scroll down</b> to zoom in the chart
			<i class="text-slate-400">(you can also click and drag :D)</i>
		</p>
	{/if}
</div>

<style>
	.loader {
		border-top-color: #475569;
		-webkit-animation: spinner 1.5s linear infinite;
		animation: spinner 1.5s linear infinite;
	}

	@-webkit-keyframes spinner {
		0% {
			-webkit-transform: rotate(0deg);
		}
		100% {
			-webkit-transform: rotate(360deg);
		}
	}

	@keyframes spinner {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}
</style>
