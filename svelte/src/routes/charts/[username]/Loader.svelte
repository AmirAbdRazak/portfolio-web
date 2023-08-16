<script lang="ts">
	export let username: string;
</script>

<div
	class="fixed top-0 left-0 right-0 bottom-0 w-full h-screen z-50 overflow-hidden bg-slate-800 flex flex-col items-center justify-center"
>
	<div
		class="loader ease-linear rounded-full border-4 border-t-4 border-slate-100 h-12 w-12 mb-4"
	/>
	<h2 class="text-center text-white text-xl font-semibold">Loading...</h2>
	<p class="w-1/3 py-2 text-center text-white">Preparing your chart {username}</p>
	<p class="w-1/2 py-2 text-center text-slate-200 text-sm">
		Tip: Hold down your <b class="text-rose-400">CTRL</b> key and
		<b class="text-rose-400">scroll down</b> to zoom in the chart
		<i class="text-slate-400">(you can also click and drag :D)</i>
	</p>
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
