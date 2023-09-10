<script lang="ts">
	import * as Tooltip from '$lib/components/ui/tooltip';
	export let imgSrc: string;
	export let imgAlt: string;
</script>

<Tooltip.Root openDelay={2}>
	<Tooltip.Trigger>
		<img
			src={imgSrc}
			alt={imgAlt}
			class="h-24 w-24 rounded-lg object-contain"
		/>
	</Tooltip.Trigger>
	<Tooltip.Content class="bg-gradient-to-br from-indigo-100 to-violet-300">
		<p class="text-zinc-800">{imgAlt}</p>
	</Tooltip.Content>
</Tooltip.Root>
