<script lang="ts">
	import * as Tooltip from '$lib/components/ui/tooltip';

	export let imgSrc: string | undefined = undefined;
	export let imgAlt: string | undefined = undefined;
</script>

<Tooltip.Root openDelay={2}>
	<Tooltip.Trigger>
		{#if imgSrc}
			<img
				src={imgSrc}
				alt={imgAlt}
				class="mx-auto h-24 w-24 rounded-lg object-contain"
			/>
		{/if}
		<slot />
	</Tooltip.Trigger>
	<Tooltip.Content class="bg-gradient-to-br from-indigo-100 to-violet-300">
		<p class="text-zinc-800">{imgAlt}</p>
	</Tooltip.Content>
</Tooltip.Root>
