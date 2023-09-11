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
		<p>Bachelor of Computer Science, CGPA: 3.61</p>
		<p>Sep ’20 - Aug ‘23</p>
	</div>
	<h2 class="text-xl font-semibold">ADDITIONAL INFORMATION</h2>
	<hr
		class={`${stateData.isDarkMode ? 'border-white' : 'border-zinc-900'} pb-4`}
	/>
	<p>
		<b> Highlighted Skills: </b>
		Python, Rust, Typescript, Docker, GraphQL, RESTful API, Django, Tokio, NodeJS,
		NextJS, SvelteKit, PostgreSQL, SQLx, Prisma ORM
	</p>
	<p>
		<b> Program Management: </b>
		Jira, Confluence, Github, Linux
	</p>
	<p class="pb-5">
		<b> Certifications & Awards: </b>
		Final Year Project Red Carpet Award, Dean’s List (Sep ‘20, Feb ‘21, Sep ‘22)
	</p>

	<h2 class="text-xl font-semibold">PROFESSIONAL EXPERIENCE</h2>
	<hr
		class={`${stateData.isDarkMode ? 'border-white' : 'border-zinc-900'} pb-4`}
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
		<p>Docker • Python • Django • GraphQL API • PostgreSQL • Redis • Linux</p>
		<p>Feb ‘23 - Aug ‘23</p>
	</div>
	<ul class="space-y-3 pb-5 text-justify md:pr-24 lg:space-y-1">
		<li>
			• Committed thousands of lines of code into production, including rigorous
			unit tests that were deployed to ensure reliability.
		</li>
		<li>
			• Spearheaded the implementation of batch loaders for multiple endpoints
			using Dataloaders, resulting in a remarkable 500% increase in processing
			speed by eliminating the N+1 Query Problem.
		</li>
		<li>
			• Developed and maintained a metrics reporting endpoint independently,
			introducing 15+ queryable items and achieving high test coverage for
			robust deployment.
		</li>
		<li>
			• Conceptualized and collaboratively implemented new features on the CSV
			endpoint, resulting in a substantial 50% improvement in user experience.
		</li>
		<li>
			• Refactored 25+ GraphQL endpoints to incorporate a new feature
			seamlessly, addressing hundreds of breaking unit tests and expediting
			frontend development by 30%.
		</li>
		<li>
			• Effective use of Jira, Confluence, and GitHub for program management and
			team collaboration.
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
		Docker • Rust • Typescript • Javascript • Axum • SvelteKit • GraphQL API •
		PostgreSQL • Linux • Cloud Deployment
	</p>
	<ul class="space-y-3 pb-5 pr-24 text-justify lg:space-y-1">
		<li>
			• Achieved blazingly fast API call execution through asynchronous
			programming, processing thousands of calls within seconds, which is
			significantly faster than synchronous methods.
		</li>
		<li>
			• Utilized Rust's robust exception handling to safely parse and manage
			extensive datasets, resulting in a 200% reduction in production errors.
		</li>
		<li>
			• Strategically employed distributed systems for cloud deployment via
			platforms like fly.io and Docker, ensuring high availability and minimal
			downtime.
		</li>
	</ul>
</div>
