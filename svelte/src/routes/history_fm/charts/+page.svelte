<script>
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	onMount(() => {
		goto('/charts');
	});
</script>

<div
	class="fixed top-0 left-0 right-0 bottom-0 w-full h-screen z-50 overflow-hidden bg-slate-800 flex flex-col items-center justify-center"
>
	<p class="w-1/3 text-center text-white">Page is deprecated. Redirecting...</p>
</div>
