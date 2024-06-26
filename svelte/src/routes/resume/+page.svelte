<script lang="ts">
	import { stateStore, type StateData } from '../StateStore';
	let stateData: StateData;

	stateStore.subscribe((data) => {
		stateData = {
			isDarkMode: data.isDarkMode
		};
	});
</script>

<div
	class={`${
		stateData.isDarkMode
			? 'bg-zinc-900 text-white'
			: 'bg-zinc-100 text-zinc-900'
	} font-serif-spectral space-y-3 px-10 py-10 text-lg md:px-20 lg:space-y-0 lg:px-40`}
>
	<h1 class="text-center text-xl font-semibold">
		MUHAMMAD AMIR BIN ABDUL RAZAK
	</h1>

	<h2 class="text-center">Backend Software Engineer</h2>
	<p class="text-center">
		Selangor, MY • amrrzk02@gmail.com • + 60 17 500-5516
	</p>
	<p class="pb-10 text-center">
		<a href="/" class="underline"> https://amirrazak.com </a>
		•
		<a href="https://github.com/AmirAbdRazak" class="underline">
			https://github.com/AmirAbdRazak
		</a>
		•
		<a href="https://linkedin.com/in/AmirAbdRazak" class="underline">
			https://linkedin.com/in/AmirAbdRazak
		</a>
	</p>
	<h2 class="text-xl font-semibold">EDUCATION</h2>
	<hr
		class={`${stateData.isDarkMode ? 'border-white' : 'border-zinc-900'} pb-4`}
	/>
	<div class="flex flex-col pb-5 md:flex-row md:justify-between">
		<p>MANAGEMENT AND SCIENCE UNIVERSITY</p>
		<p>Selangor, MY</p>
	</div>
	<div class="flex flex-col pb-5 md:flex-row md:justify-between">
		<p>Bachelor of Computer Science, CGPA: 3.65</p>
		<p>Sep ’20 - Aug ‘23</p>
	</div>
	<h2 class="text-xl font-semibold">ADDITIONAL INFORMATION</h2>
	<hr
		class={`${stateData.isDarkMode ? 'border-white' : 'border-zinc-900'} pb-4`}
	/>
	<p>
		<b> Languages/Frameworks: </b>
		Python, Javascript, Typescript, Django, NodeJS, NextJS
	</p>
	<p class="pb-5">
		<b> Technologies: </b>
		Docker, GraphQL, RESTful API, PostgreSQL, Linux, Jira, Github
	</p>

	<h2 class="text-xl font-semibold">PROFESSIONAL EXPERIENCE</h2>
	<hr
		class={`${stateData.isDarkMode ? 'border-white' : 'border-zinc-900'} pb-4`}
	/>

	<div class="flex flex-col pb-2 md:flex-row md:justify-between">
		<p>
			<b class="text-xl"> INFIN8CO SDN BHD </b>
			• Junior Software Developer
		</p>
		<p>Kuala Lumpur, MY</p>
	</div>
	<div
		class="flex flex-col space-y-2 pb-2 md:flex-row md:justify-between md:space-x-7 md:space-y-0"
	>
		<p>Docker • Python • Django • GraphQL API • PostgreSQL • Linux</p>
		<p>Sep ‘23 - Present</p>
	</div>
	<ul class="space-y-3 pb-5 text-justify md:pr-24 lg:space-y-1">
		<li>
			• Accelerated time-series reporting queries through database indexing and
			set operations, achieving 40x performance improvement.
		</li>
		<li>
			• Optimized test data generation process, reducing complexity from O(n) to
			O(log n) by dealing with batch processing with relational models.
		</li>
		<li>
			• Implemented granular feature control and permission management across a
			multi-tenant setup, including rate limiting and selective feature
			enablement per tenant, to enhance security and customization.
		</li>
		<li>
			• Expanded test coverage for the CSV endpoint, implementing comprehensive
			edge case handling to enhance product robustness.
		</li>
	</ul>
	<hr
		class={`${
			stateData.isDarkMode ? 'border-white' : 'border-zinc-900'
		} w-1/4 pb-4`}
	/>
	<div class="flex flex-col pb-2 md:flex-row md:justify-between">
		<p>
			<b class="text-xl"> INFIN8CO SDN BHD </b>
			• Backend Engineer Intern
		</p>
		<p>Kuala Lumpur, MY</p>
	</div>
	<div
		class="flex flex-col space-y-2 pb-2 md:flex-row md:justify-between md:space-x-7 md:space-y-0"
	>
		<p>Docker • Python • Django • GraphQL API • PostgreSQL • Linux</p>
		<p>Feb ‘23 - Aug ‘23</p>
	</div>
	<ul class="space-y-3 pb-5 text-justify md:pr-24 lg:space-y-1">
		<li>
			• Refactored 25+ GraphQL endpoints for seamless integration of a new
			feature in close collaboration with the frontend team.
		</li>
		<li>
			• Implemented batch loaders for multiple endpoints using Dataloaders,
			resulting in a 500% increase in processing speed by eliminating the N+1
			Query Problem.
		</li>
		<li>
			• Conceptualized and implemented mulitple quality of life features on the
			CSV endpoint, improving developer experience.
		</li>
	</ul>

	<h2 class="text-xl font-semibold">HIGHLIGHTED PROJECT</h2>
	<hr
		class={`${stateData.isDarkMode ? 'border-white' : 'border-zinc-900'} pb-4`}
	/>
	<p>
		LISTENING HISTORY DATA VISUALIZER • <a href="/charts" class="underline">
			https://amirrazak.com/charts
		</a>
	</p>
	<p class="pb-2">
		Docker • Rust • Typescript • Javascript • SvelteKit • GraphQL API •
		PostgreSQL • Linux • Cloud Deployment
	</p>
	<ul class="space-y-3 pb-5 text-justify md:pr-24 lg:space-y-1">
		<li>
			• Utilized batch asynchronous programming to utilize IO downtime and speed
			up mass HTTP requests.
		</li>
		<li>
			• Strategically employed distributed systems for cloud deployment via
			platforms like fly.io and Docker, ensuring high availability and minimal
			downtime.
		</li>
	</ul>
</div>
