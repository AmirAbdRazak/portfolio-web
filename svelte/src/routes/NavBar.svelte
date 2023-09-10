<script lang="ts">
	import { page } from '$app/stores';
	import { Moon, Sun } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { stateStore } from './StateStore';
	let showMobileMenu = false;
	let isDarkMode: boolean;
	$: isDarkMode = true;
</script>

<nav
	class={` drop-shadow-md ${
		$page.route.id && $page.route.id == '/'
			? showMobileMenu
				? 'bg-zinc-800'
				: 'bg-gradient-to-b from-indigo-100 to-indigo-200 drop-shadow-none'
			: 'bg-zinc-900'
	}`}
>
	<div class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
		<div class="relative flex h-16 items-center justify-between">
			<div class="absolute inset-y-0 left-0 flex items-center sm:hidden">
				<!-- Mobile menu button-->
				<button
					type="button"
					class="relative inline-flex items-center justify-center rounded-md p-2 text-zinc-400 hover:bg-zinc-800 hover:text-zinc-200 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-zinc-800"
					aria-controls="mobile-menu"
					aria-expanded="false"
					on:click={() => {
						showMobileMenu = !showMobileMenu;
					}}
				>
					<span class="absolute -inset-0.5" />
					<span class="sr-only">Open main menu</span>
					<!--
            Icon when menu is closed.

            Menu open: "hidden", Menu closed: "block"
          -->
					<svg
						class="block h-6 w-6"
						fill="none"
						viewBox="0 0 24 24"
						stroke-width="1.5"
						stroke="currentColor"
						aria-hidden="true"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
						/>
					</svg>
					<!--
            Icon when menu is open.

            Menu open: "block", Menu closed: "hidden"
          -->
					<svg
						class="hidden h-6 w-6"
						fill="none"
						viewBox="0 0 24 24"
						stroke-width="1.5"
						stroke="currentColor"
						aria-hidden="true"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							d="M6 18L18 6M6 6l12 12"
						/>
					</svg>
				</button>
			</div>
			<div
				class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start"
			>
				<div
					class="flex flex-shrink-0 items-center rounded-md px-2 text-lg text-zinc-300"
				>
					<a
						class={`font-semibold ${
							$page.route.id && $page.route.id == '/'
								? showMobileMenu
									? 'text-violet-100'
									: 'text-zinc-800'
								: 'text-zinc-300'
						}`}
						href="/"
					>
						Amir <i
							class="not-italic {$page.route.id &&
							$page.route.id.split('/').length > 0 &&
							$page.route.id.split('/')[1] == 'charts'
								? 'text-rose-400'
								: 'text-violet-600'}"
							>Razak
						</i>
					</a>
				</div>
				<div
					class="hidden w-full items-center sm:ml-6 sm:flex sm:flex-row sm:justify-between"
				>
					<div class="flex flex-row items-center space-x-4">
						<a
							href="/charts"
							class={`text-md rounded-md px-3 py-2 font-medium text-white ${
								$page.route.id && $page.route.id == '/'
									? ' bg-transparent font-semibold text-zinc-800 hover:bg-violet-100'
									: 'hover:bg-zinc-800'
							}`}
							aria-current="page">Charts</a
						>
						<a
							href="/resume"
							class={`text-md rounded-md px-3 py-2 font-medium text-white ${
								$page.route.id && $page.route.id == '/'
									? ' bg-transparent font-semibold text-zinc-800 hover:bg-violet-100'
									: 'hover:bg-zinc-800'
							}`}
							aria-current="page">Resume</a
						>
					</div>
					<div
						class={$page.route.id && $page.route.id == '/resume'
							? 'block'
							: 'hidden'}
					>
						<Button
							on:click={() => {
								isDarkMode = !isDarkMode;
								stateStore.set({
									isDarkMode
								});
							}}
						>
							{#if isDarkMode}
								<Moon class="text-zinc-200" size={28} />
							{:else}
								<Sun class="text-zinc-200" size={28} />
							{/if}
						</Button>
					</div>
				</div>
			</div>
		</div>
	</div>

	<div class="{showMobileMenu ? 'block' : 'hidden'} sm:hidden" id="mobile-menu">
		<div
			class={`w-full ${
				$page.route.id && $page.route.id == '/' ? 'bg-zinc-800' : 'bg-zinc-900'
			} px-2 pb-3 pt-2`}
		>
			<a
				href="/charts"
				class={`rounded-md ${
					$page.route.id && $page.route.id == '/'
						? 'bg-violet-300 text-zinc-800'
						: 'bg-zinc-800 text-zinc-200'
				} px-3 py-2 text-base font-medium `}
				aria-current="page">Charts</a
			>
		</div>
	</div>
</nav>
