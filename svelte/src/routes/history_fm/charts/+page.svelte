<script>
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	onMount(() => {
		goto('/charts');
	});
</script>

<div
	class="fixed bottom-0 left-0 right-0 top-0 z-50 flex h-screen w-full flex-col items-center justify-center overflow-hidden bg-slate-800"
>
	<p class="w-1/3 text-center text-white">Page is deprecated. Redirecting...</p>
</div>
